use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CdxConfig {
    search_size: Option<usize>,
    max_size: Option<usize>,
}

impl CdxConfig {
    pub fn search_size(&self) -> usize {
        self.search_size.unwrap_or(30)
    }

    pub fn max_size(&self) -> usize {
        self.max_size.unwrap_or(1024)
    }
}

impl Default for CdxConfig {
    fn default() -> Self {
        CdxConfig {
            search_size: Some(30),
            max_size: Some(1024),
        }
    }
}
