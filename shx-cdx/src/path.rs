use std::fmt::{Display, Formatter};
use std::path::PathBuf;

/// Represents a directory path, which is guaranteed to be a canonicalized directory.
pub struct CanonicalPath(PathBuf);

impl CanonicalPath {
    pub fn from_string(path: &String) -> anyhow::Result<Self> {
        Self::from_path(PathBuf::from(path))
    }

    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self(path.canonicalize()?))
    }
}

impl Display for CanonicalPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}