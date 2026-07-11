use std::fmt;
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

use crate::profiles::{LocationSpec, NodeKind, Policy};
use crate::resolver::{ResolveError, Resolver};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    RegularFile,
    Directory,
    Symlink,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataSummary {
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
    pub node: NodeType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindingReason {
    WrongOwner {
        expected: u32,
        actual: u32,
    },
    UnexpectedNodeType {
        expected: NodeKind,
        actual: NodeType,
    },
    GroupOrOtherAccess {
        mode: u32,
    },
    GroupOrOtherWrite {
        mode: u32,
    },
    WritableAncestor {
        path: PathBuf,
        mode: u32,
    },
    SymlinkComponent {
        path: PathBuf,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InspectionResult {
    Pass {
        path: PathBuf,
        metadata: MetadataSummary,
    },
    Finding {
        path: PathBuf,
        metadata: Option<MetadataSummary>,
        reasons: Vec<FindingReason>,
    },
    MissingOptional {
        path: PathBuf,
    },
    MissingRequired {
        path: PathBuf,
    },
    AccessDenied {
        path: PathBuf,
        message: String,
    },
    Unknown {
        path: PathBuf,
        reason: String,
    },
}

impl InspectionResult {
    pub fn is_pass(&self) -> bool {
        matches!(self, Self::Pass { .. })
    }
    pub fn is_finding(&self) -> bool {
        matches!(self, Self::Finding { .. })
    }
    pub fn is_missing_optional(&self) -> bool {
        matches!(self, Self::MissingOptional { .. })
    }
    pub fn is_missing_required(&self) -> bool {
        matches!(self, Self::MissingRequired { .. })
    }
    pub fn reasons(&self) -> &[FindingReason] {
        match self {
            Self::Finding { reasons, .. } => reasons,
            _ => &[],
        }
    }
}

#[derive(Debug)]
pub enum InspectionError {
    Resolver(ResolveError),
}

impl fmt::Display for InspectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resolver(error) => error.fmt(f),
        }
    }
}
impl std::error::Error for InspectionError {}
impl From<ResolveError> for InspectionError {
    fn from(error: ResolveError) -> Self {
        Self::Resolver(error)
    }
}

pub struct MetadataInspector {
    home: PathBuf,
    owner_uid: u32,
    resolver: Resolver,
}

impl MetadataInspector {
    pub fn new<P: Into<PathBuf>>(home: P, owner_uid: u32) -> Result<Self, ResolveError> {
        let home = home.into();
        let resolver = Resolver::new(home.clone())?;
        Ok(Self {
            home,
            owner_uid,
            resolver,
        })
    }

    pub fn inspect_location(
        &self,
        location: &LocationSpec,
    ) -> Result<Vec<InspectionResult>, InspectionError> {
        match self.resolver.resolve(location) {
            Ok(paths) => Ok(paths
                .into_iter()
                .map(|resolved| {
                    if !resolved.exists() {
                        if location.optional {
                            InspectionResult::MissingOptional {
                                path: resolved.path().to_path_buf(),
                            }
                        } else {
                            InspectionResult::MissingRequired {
                                path: resolved.path().to_path_buf(),
                            }
                        }
                    } else {
                        self.inspect_path(resolved.path(), location.kind, location.policy)
                    }
                })
                .collect()),
            Err(ResolveError::SymlinkComponent(path)) => Ok(vec![InspectionResult::Finding {
                path: path.clone(),
                metadata: None,
                reasons: vec![FindingReason::SymlinkComponent { path }],
            }]),
            Err(ResolveError::AccessDenied { path }) => Ok(vec![InspectionResult::AccessDenied {
                path,
                message: "permission denied".into(),
            }]),
            Err(error) => Err(error.into()),
        }
    }

    fn inspect_path(&self, path: &Path, kind: NodeKind, policy: Policy) -> InspectionResult {
        let metadata = match fs::symlink_metadata(path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
                return InspectionResult::AccessDenied {
                    path: path.to_path_buf(),
                    message: error.to_string(),
                }
            }
            Err(error) => {
                return InspectionResult::Unknown {
                    path: path.to_path_buf(),
                    reason: error.to_string(),
                }
            }
        };
        let summary = summarize(&metadata);
        let mut reasons = Vec::new();
        if summary.uid != self.owner_uid {
            reasons.push(FindingReason::WrongOwner {
                expected: self.owner_uid,
                actual: summary.uid,
            });
        }
        if !matches_kind(kind, summary.node) {
            reasons.push(FindingReason::UnexpectedNodeType {
                expected: kind,
                actual: summary.node,
            });
        }
        match policy {
            Policy::SecretFile | Policy::CredentialConfig | Policy::PrivateDirectory
                if summary.mode & 0o077 != 0 =>
            {
                reasons.push(FindingReason::GroupOrOtherAccess { mode: summary.mode })
            }
            Policy::TrustedConfig | Policy::ExecutableConfig if summary.mode & 0o022 != 0 => {
                reasons.push(FindingReason::GroupOrOtherWrite { mode: summary.mode })
            }
            _ => {}
        }
        if matches!(policy, Policy::ExecutableConfig) {
            self.add_writable_ancestor_reasons(path, &mut reasons);
        }
        if reasons.is_empty() {
            InspectionResult::Pass {
                path: path.to_path_buf(),
                metadata: summary,
            }
        } else {
            InspectionResult::Finding {
                path: path.to_path_buf(),
                metadata: Some(summary),
                reasons,
            }
        }
    }

    fn add_writable_ancestor_reasons(&self, path: &Path, reasons: &mut Vec<FindingReason>) {
        let mut current = path.parent();
        while let Some(ancestor) = current {
            if ancestor == self.home {
                break;
            }
            match fs::symlink_metadata(ancestor) {
                Ok(metadata) if metadata.file_type().is_symlink() => {
                    reasons.push(FindingReason::SymlinkComponent {
                        path: ancestor.to_path_buf(),
                    })
                }
                Ok(metadata) if metadata.permissions().mode() & 0o022 != 0 => {
                    reasons.push(FindingReason::WritableAncestor {
                        path: ancestor.to_path_buf(),
                        mode: metadata.permissions().mode() & 0o7777,
                    })
                }
                Ok(_) => {}
                Err(_) => {}
            }
            current = ancestor.parent();
        }
    }
}

fn summarize(metadata: &std::fs::Metadata) -> MetadataSummary {
    let node = if metadata.file_type().is_symlink() {
        NodeType::Symlink
    } else if metadata.is_file() {
        NodeType::RegularFile
    } else if metadata.is_dir() {
        NodeType::Directory
    } else {
        NodeType::Other
    };
    MetadataSummary {
        uid: metadata.uid(),
        gid: metadata.gid(),
        mode: metadata.permissions().mode() & 0o7777,
        node,
    }
}
fn matches_kind(kind: NodeKind, node: NodeType) -> bool {
    match kind {
        NodeKind::File => node == NodeType::RegularFile,
        NodeKind::Directory => node == NodeType::Directory,
        NodeKind::Either => node != NodeType::Symlink,
    }
}
