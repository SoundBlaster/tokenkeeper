use std::collections::HashSet;
use std::fmt;
use std::path::{Component, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    MacOs,
    Linux,
}

impl Platform {
    #[cfg(target_os = "macos")]
    pub const fn current() -> Option<Self> {
        Some(Self::MacOs)
    }

    #[cfg(target_os = "linux")]
    pub const fn current() -> Option<Self> {
        Some(Self::Linux)
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    pub const fn current() -> Option<Self> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Root {
    Home,
    XdgConfig,
    MacApplicationSupport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    File,
    Directory,
    Either,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    SecretFile,
    CredentialConfig,
    PrivateDirectory,
    TrustedConfig,
    ExecutableConfig,
}

impl Policy {
    /// Confidentiality and integrity are independent requirements. Credential
    /// locations intentionally compose both, rather than choosing one mode.
    pub const fn requires_confidentiality(self) -> bool {
        matches!(
            self,
            Self::SecretFile | Self::CredentialConfig | Self::PrivateDirectory
        )
    }

    pub const fn requires_integrity(self) -> bool {
        matches!(
            self,
            Self::CredentialConfig | Self::TrustedConfig | Self::ExecutableConfig
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Traversal {
    Exact,
    Bounded { max_depth: u8, max_entries: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationSpec {
    pub root: Root,
    pub path: PathBuf,
    pub kind: NodeKind,
    pub policy: Policy,
    pub optional: bool,
    pub traversal: Traversal,
}

impl LocationSpec {
    pub fn exact<P: Into<PathBuf>>(
        root: Root,
        path: P,
        kind: NodeKind,
        policy: Policy,
        optional: bool,
    ) -> Self {
        Self {
            root,
            path: path.into(),
            kind,
            policy,
            optional,
            traversal: Traversal::Exact,
        }
    }

    pub fn bounded<P: Into<PathBuf>>(
        root: Root,
        path: P,
        kind: NodeKind,
        policy: Policy,
        optional: bool,
        max_depth: u8,
        max_entries: usize,
    ) -> Self {
        Self {
            root,
            path: path.into(),
            kind,
            policy,
            optional,
            traversal: Traversal::Bounded {
                max_depth,
                max_entries,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileSpec {
    pub id: String,
    pub display_name: String,
    pub platforms: Vec<Platform>,
    pub locations: Vec<LocationSpec>,
    pub source: Option<String>,
    pub verified_on: Option<String>,
}

impl ProfileSpec {
    pub fn new<I, D>(
        id: I,
        display_name: D,
        platforms: Vec<Platform>,
        locations: Vec<LocationSpec>,
    ) -> Self
    where
        I: Into<String>,
        D: Into<String>,
    {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            platforms,
            locations,
            source: None,
            verified_on: None,
        }
    }

    pub fn with_evidence<S, D>(mut self, source: S, verified_on: D) -> Self
    where
        S: Into<String>,
        D: Into<String>,
    {
        self.source = Some(source.into());
        self.verified_on = Some(verified_on.into());
        self
    }

    pub fn available_on(&self, platform: Option<Platform>) -> bool {
        platform.is_some_and(|platform| self.platforms.contains(&platform))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    EmptyId,
    InvalidId { id: String },
    EmptyDisplayName { id: String },
    NoPlatforms { id: String },
    NoLocations { id: String },
    EmptyPath { id: String },
    AbsolutePath { id: String, path: PathBuf },
    ParentTraversal { id: String, path: PathBuf },
    InvalidDepth { id: String, max_depth: u8 },
    InvalidEntryLimit { id: String, max_entries: usize },
    DuplicateId { id: String },
}

impl ValidationError {
    pub fn is_duplicate_id(&self) -> bool {
        matches!(self, Self::DuplicateId { .. })
    }

    pub fn is_parent_traversal(&self) -> bool {
        matches!(self, Self::ParentTraversal { .. })
    }

    pub fn is_absolute_path(&self) -> bool {
        matches!(self, Self::AbsolutePath { .. })
    }

    pub fn is_invalid_limit(&self) -> bool {
        matches!(
            self,
            Self::InvalidDepth { .. } | Self::InvalidEntryLimit { .. }
        )
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId => write!(f, "profile id must not be empty"),
            Self::InvalidId { id } => write!(f, "invalid profile id `{id}`"),
            Self::EmptyDisplayName { id } => write!(f, "profile `{id}` has an empty display name"),
            Self::NoPlatforms { id } => write!(f, "profile `{id}` has no platforms"),
            Self::NoLocations { id } => write!(f, "profile `{id}` has no locations"),
            Self::EmptyPath { id } => write!(f, "profile `{id}` has an empty location path"),
            Self::AbsolutePath { id, path } => {
                write!(f, "profile `{id}` has an absolute location path `{path:?}`")
            }
            Self::ParentTraversal { id, path } => {
                write!(f, "profile `{id}` has parent traversal in `{path:?}`")
            }
            Self::InvalidDepth { id, max_depth } => {
                write!(f, "profile `{id}` has invalid max depth {max_depth}")
            }
            Self::InvalidEntryLimit { id, max_entries } => {
                write!(f, "profile `{id}` has invalid max entries {max_entries}")
            }
            Self::DuplicateId { id } => write!(f, "duplicate profile id `{id}`"),
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProfileRegistry {
    profiles: Vec<ProfileSpec>,
}

impl ProfileRegistry {
    pub fn new(profiles: Vec<ProfileSpec>) -> Self {
        Self { profiles }
    }

    pub fn profiles(&self) -> &[ProfileSpec] {
        &self.profiles
    }

    pub fn find(&self, id: &str) -> Option<&ProfileSpec> {
        self.profiles.iter().find(|profile| profile.id == id)
    }

    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        // ProfileRegistry не должен валидировать, он просто хранит, а валидирует либо сам себя профиль, либо специальный валидатор, либо ValidatableProfileSpec(ProfileSpec)...
        let mut errors = Vec::new();
        let mut ids = HashSet::new();

        for profile in &self.profiles {
            if profile.id.is_empty() {
                errors.push(ValidationError::EmptyId);
            } else if !valid_id(&profile.id) {
                errors.push(ValidationError::InvalidId {
                    id: profile.id.clone(),
                });
            }

            if !ids.insert(profile.id.clone()) {
                errors.push(ValidationError::DuplicateId {
                    id: profile.id.clone(),
                });
            }
            if profile.display_name.trim().is_empty() {
                errors.push(ValidationError::EmptyDisplayName {
                    id: profile.id.clone(),
                });
            }
            if profile.platforms.is_empty() {
                errors.push(ValidationError::NoPlatforms {
                    id: profile.id.clone(),
                });
            }
            if profile.locations.is_empty() {
                errors.push(ValidationError::NoLocations {
                    id: profile.id.clone(),
                });
            }

            for location in &profile.locations {
                validate_location(&profile.id, location, &mut errors);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub fn builtin_registry() -> ProfileRegistry {
    let source = "docs/agent-storage-locations.md";
    let date = "2026-07-11";
    ProfileRegistry::new(vec![
        ProfileSpec::new(
            "codex",
            "OpenAI Codex",
            vec![Platform::MacOs, Platform::Linux],
            vec![
                LocationSpec::exact(
                    Root::Home,
                    ".codex/auth.json",
                    NodeKind::File,
                    Policy::CredentialConfig,
                    true,
                ),
                LocationSpec::exact(
                    Root::Home,
                    ".codex/config.toml",
                    NodeKind::File,
                    Policy::CredentialConfig,
                    true,
                ),
            ],
        )
        .with_evidence(source, date),
        ProfileSpec::new(
            "claude-code",
            "Claude Code",
            vec![Platform::MacOs, Platform::Linux],
            vec![
                LocationSpec::exact(
                    Root::Home,
                    ".claude.json",
                    NodeKind::File,
                    Policy::CredentialConfig,
                    true,
                ),
                LocationSpec::exact(
                    Root::Home,
                    ".claude/settings.json",
                    NodeKind::File,
                    Policy::CredentialConfig,
                    true,
                ),
                LocationSpec::exact(
                    Root::Home,
                    ".claude/settings.local.json",
                    NodeKind::File,
                    Policy::CredentialConfig,
                    true,
                ),
            ],
        )
        .with_evidence(source, date),
        ProfileSpec::new(
            "opencode",
            "OpenCode",
            vec![Platform::MacOs, Platform::Linux],
            vec![
                LocationSpec::exact(
                    Root::XdgConfig,
                    "opencode/opencode.json",
                    NodeKind::File,
                    Policy::CredentialConfig,
                    true,
                ),
                LocationSpec::exact(
                    Root::Home,
                    ".local/share/opencode/auth.json",
                    NodeKind::File,
                    Policy::CredentialConfig,
                    true,
                ),
            ],
        )
        .with_evidence(source, date),
        ProfileSpec::new(
            "cursor",
            "Cursor MCP",
            vec![Platform::MacOs, Platform::Linux],
            vec![LocationSpec::exact(
                Root::Home,
                ".cursor/mcp.json",
                NodeKind::File,
                Policy::CredentialConfig,
                true,
            )],
        )
        .with_evidence(source, date),
        ProfileSpec::new(
            "mcp-integrations",
            "MCP and utility credential configs",
            vec![Platform::MacOs, Platform::Linux],
            vec![
                LocationSpec::exact(
                    Root::MacApplicationSupport,
                    "Claude/claude_desktop_config.json",
                    NodeKind::File,
                    Policy::CredentialConfig,
                    true,
                ),
                LocationSpec::exact(
                    Root::Home,
                    ".aws/credentials",
                    NodeKind::File,
                    Policy::CredentialConfig,
                    true,
                ),
                LocationSpec::exact(
                    Root::Home,
                    ".aws/config",
                    NodeKind::File,
                    Policy::CredentialConfig,
                    true,
                ),
            ],
        )
        .with_evidence(source, date),
    ])
}

fn valid_id(id: &str) -> bool {
    let mut chars = id.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_lowercase() || first.is_ascii_digit())
        && chars.all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || character == '-'
                || character == '_'
        })
}

fn validate_location(id: &str, location: &LocationSpec, errors: &mut Vec<ValidationError>) {
    if location.path.as_os_str().is_empty() {
        errors.push(ValidationError::EmptyPath { id: id.to_owned() });
    }
    if location.path.is_absolute() {
        errors.push(ValidationError::AbsolutePath {
            id: id.to_owned(),
            path: location.path.clone(),
        });
    }
    if location
        .path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        errors.push(ValidationError::ParentTraversal {
            id: id.to_owned(),
            path: location.path.clone(),
        });
    }
    if let Traversal::Bounded {
        max_depth,
        max_entries,
    } = location.traversal
    {
        if max_depth == 0 {
            errors.push(ValidationError::InvalidDepth {
                id: id.to_owned(),
                max_depth,
            });
        }
        if max_entries == 0 {
            errors.push(ValidationError::InvalidEntryLimit {
                id: id.to_owned(),
                max_entries,
            });
        }
    }
}
