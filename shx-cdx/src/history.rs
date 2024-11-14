use std::{fs, io};
use std::fmt::{Debug, Display};
use std::io::Write;
use std::path::Path;

use anyhow::Context;
use bincode::Options;
use serde::{Deserialize, Serialize};
use shx_config::config::path_for;

const DB: &str = "cdx.db";

#[derive(Serialize, Deserialize)]
pub struct History(Vec<Entry>);

impl History {
    pub fn open() -> anyhow::Result<Self> {
        let path = path_for(DB)?;

        return match fs::read(&path) {
            Ok(bytes) => {
                let deserializer = bincode::options();
                let history = deserializer.deserialize(&bytes)?;
                Ok(history)
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                Ok(History(Vec::new()))
            }
            Err(e) => {
                Err(e).with_context(|| format!("[fatal] cannot not read from database: {}", path.display()))
            }
        };
    }

    pub fn read(&self, size: usize) -> Vec<Entry> {
        self.0.iter()
            .rev()
            .take(size)
            .map(|it| it.clone())
            .collect()
    }

    // if ever visited, promote to first. else append.
    pub fn append_last(&mut self, new_entry: Entry) -> () {
        self.0.retain(|it| &it.canonical != &new_entry.canonical);
        self.0.push(new_entry);
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = path_for(DB)?;
        let serializer = bincode::options();
        let serialized = serializer.serialize(&self.0)?;
        fs::write(&path, serialized)?;
        Ok(())
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub raw: String,
    pub canonical: String,
}

impl Entry {
    pub fn new<S, P>(raw: S, canonical: P) -> Self
    where
        S: Into<String>,
        P: AsRef<Path>,
    {
        Entry {
            raw: raw.into(),
            canonical: canonical.as_ref().display().to_string(),
        }
    }

    pub fn with_raw(&self, raw: String) -> Self {
        Entry {
            raw,
            canonical: self.canonical.clone(),
        }
    }
}