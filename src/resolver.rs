use std::fmt;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

use crate::profiles::{LocationSpec, Root, Traversal};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPath {
    path: PathBuf,
    exists: bool,
}

impl ResolvedPath {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn exists(&self) -> bool {
        self.exists
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    HomeNotAbsolute,
    HomeMissing(PathBuf),
    HomeNotDirectory(PathBuf),
    HomeSymlink(PathBuf),
    InvalidRelativePath(PathBuf),
    SymlinkComponent(PathBuf),
    AccessDenied { path: PathBuf },
    Io { path: PathBuf, message: String },
    TraversalLimitExceeded { path: PathBuf, max_entries: usize },
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HomeNotAbsolute => write!(f, "trusted home must be absolute"),
            Self::HomeMissing(path) => write!(f, "trusted home does not exist: {path:?}"),
            Self::HomeNotDirectory(path) => write!(f, "trusted home is not a directory: {path:?}"),
            Self::HomeSymlink(path) => write!(f, "trusted home is a symlink: {path:?}"),
            Self::InvalidRelativePath(path) => write!(f, "invalid relative path: {path:?}"),
            Self::SymlinkComponent(path) => {
                write!(f, "symlink component is not followed: {path:?}")
            }
            Self::AccessDenied { path } => write!(f, "access denied at {path:?}"),
            Self::Io { path, message } => write!(f, "filesystem error at {path:?}: {message}"),
            Self::TraversalLimitExceeded { path, max_entries } => {
                write!(
                    f,
                    "traversal limit exceeded at {path:?}: max entries {max_entries}"
                )
            }
        }
    }
}

impl std::error::Error for ResolveError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolver {
    home: PathBuf,
}

impl Resolver {
    pub fn new<P: Into<PathBuf>>(home: P) -> Result<Self, ResolveError> {
        let home = home.into();
        if !home.is_absolute() {
            return Err(ResolveError::HomeNotAbsolute);
        }

        let metadata = match fs::symlink_metadata(&home) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Err(ResolveError::HomeMissing(home));
            }
            Err(error) => return Err(io_error(home, error)),
        };
        if metadata.file_type().is_symlink() {
            return Err(ResolveError::HomeSymlink(home));
        }
        if !metadata.is_dir() {
            return Err(ResolveError::HomeNotDirectory(home));
        }

        Ok(Self { home })
    }

    pub fn root_path(&self, root: Root) -> PathBuf {
        match root {
            Root::Home => self.home.clone(),
            Root::XdgConfig => self.home.join(".config"),
            Root::MacApplicationSupport => self.home.join("Library/Application Support"),
        }
    }

    pub fn resolve(&self, location: &LocationSpec) -> Result<Vec<ResolvedPath>, ResolveError> {
        validate_relative_path(&location.path)?;
        let root = self.root_path(location.root);
        let path = root.join(&location.path);
        let exists = self.check_components(&root, &location.path)?;
        if !exists {
            return Ok(vec![ResolvedPath {
                path,
                exists: false,
            }]);
        }

        match location.traversal {
            Traversal::Exact => Ok(vec![ResolvedPath { path, exists: true }]),
            Traversal::Bounded {
                max_depth,
                max_entries,
            } => {
                let mut resolved = Vec::new();
                self.walk(&path, 0, max_depth, max_entries, &mut resolved)?;
                Ok(resolved)
            }
        }
    }

    fn check_components(&self, root: &Path, relative: &Path) -> Result<bool, ResolveError> {
        let root_metadata = match fs::symlink_metadata(root) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(io_error(root.to_path_buf(), error)),
        };
        if root_metadata.file_type().is_symlink() {
            return Err(ResolveError::SymlinkComponent(root.to_path_buf()));
        }

        let mut current = root.to_path_buf();
        for component in relative.components() {
            let Component::Normal(part) = component else {
                if matches!(component, Component::CurDir) {
                    continue;
                }
                return Err(ResolveError::InvalidRelativePath(relative.to_path_buf()));
            };
            current.push(part);
            match fs::symlink_metadata(&current) {
                Ok(metadata) if metadata.file_type().is_symlink() => {
                    return Err(ResolveError::SymlinkComponent(current));
                }
                Ok(_) => {}
                Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
                Err(error) => return Err(io_error(current, error)),
            }
        }
        Ok(true)
    }

    fn walk(
        &self,
        path: &Path,
        depth: u8,
        max_depth: u8,
        max_entries: usize,
        resolved: &mut Vec<ResolvedPath>,
    ) -> Result<(), ResolveError> {
        let metadata =
            fs::symlink_metadata(path).map_err(|error| io_error(path.to_path_buf(), error))?;
        if metadata.file_type().is_symlink() {
            return Err(ResolveError::SymlinkComponent(path.to_path_buf()));
        }

        resolved.push(ResolvedPath {
            path: path.to_path_buf(),
            exists: true,
        });
        if resolved.len() > max_entries {
            return Err(ResolveError::TraversalLimitExceeded {
                path: path.to_path_buf(),
                max_entries,
            });
        }
        if !metadata.is_dir() || depth >= max_depth {
            return Ok(());
        }

        let mut children = fs::read_dir(path)
            .map_err(|error| io_error(path.to_path_buf(), error))?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| io_error(path.to_path_buf(), error))?;
        children.sort_by_key(|child| {
            child
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default()
        });
        for child in children {
            self.walk(&child, depth + 1, max_depth, max_entries, resolved)?;
        }
        Ok(())
    }
}

fn validate_relative_path(path: &Path) -> Result<(), ResolveError> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(ResolveError::InvalidRelativePath(path.to_path_buf()));
    }
    Ok(())
}

fn io_error(path: PathBuf, error: io::Error) -> ResolveError {
    if error.kind() == io::ErrorKind::PermissionDenied {
        return ResolveError::AccessDenied { path };
    }
    ResolveError::Io {
        path,
        message: error.to_string(),
    }
}
