use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use anyhow::{anyhow, Context};

pub struct DirPath(PathBuf);

impl DirPath {
    pub fn from_string(path: String) -> anyhow::Result<Self> {
        Self::from_path(PathBuf::from(path))
    }

    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        if !path.exists() {
            return Err(anyhow!("Path {} does not exist", path.display()));
        }
        if !path.is_dir() {
            return Err(anyhow!("Path {} is not a directory", path.display()));
        }
        Ok(Self(path))
    }

    pub fn canonicalize(&self) -> anyhow::Result<Self> {
        let canonical = self.0.canonicalize()
            .context(format!("failed to canonicalize path {}", self.0.display()))?;
        return Ok(Self::from_path(canonical)?);
    }
}

impl Display for DirPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}