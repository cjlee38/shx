use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use anyhow::Context;
use shx_config::config::path_for;

use crate::path::DirPath;

/// Unit separator character for serialization.
///
/// This character is used to separate raw path and canonical path.
const SEPARATOR: char = 0x1f as char;

pub struct History(Vec<Entry>);

#[derive(Debug)]
pub struct Entry {
    pub raw: String,
    pub canonical: String,
}

impl Entry {
    pub fn from_special(raw: String, canonical: &DirPath) -> anyhow::Result<Self> {
        Ok(Entry {
            raw: format!("^{}", raw),
            canonical: canonical.canonicalize()?.to_string()
        })
    }

    pub fn from_dir(dir: &DirPath) -> anyhow::Result<Self> {
        Ok(Entry {
            raw: dir.to_string(),
            canonical: dir.canonicalize()?.to_string(),
        })
    }
}

impl Entry {
    fn serialize(&self) -> String {
        let mut s = String::new();
        s.push_str(self.raw.as_str());
        s.push(SEPARATOR);
        s.push_str(self.canonical.as_str());
        s.push('\n');
        return s;
    }
}

impl History {
    pub fn read(size: usize) -> anyhow::Result<Self> {
        let path = path_for("cd_history")?;
        let file = OpenOptions::new()
            .read(true)
            .open(path)?;
        // This function can be improved in a efficient way, but left as it is for simplicity.
        let reader = BufReader::new(&file);
        let rows: Vec<Entry> = reader.lines()
            .map(|line| {
                let text = line.unwrap();
                let s = text.split(SEPARATOR).collect::<Vec<&str>>();
                let raw = s[0].to_string();
                let canonical = s[1].to_string();

                Entry { raw, canonical }
            })
            .collect::<Vec<Entry>>()
            .into_iter()
            .rev()
            .take(size)
            .collect();
        Ok(History(rows))
    }

    pub fn append(&self, to_append: &Entry) -> anyhow::Result<()> {
        if self.is_duplicated(to_append) {
            return Ok(());
        }

        let path = path_for("cd_history").context("cd_history file not found")?;
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .context("failed to open cd_history file")?;
        let mut file = BufWriter::new(file);
        file.write_all(to_append.serialize().as_bytes())
            .context("failed to write to cd_history file")?;
        Ok(())
    }

    fn is_duplicated(&self, to_append: &Entry) -> bool {
        self.0.get(0)
            .map(|row| row.canonical == to_append.canonical)
            .unwrap_or(false)
    }

    pub fn find_by_shortcut(&self, shortcut: PathBuf) -> Option<DirPath> {
        self.0.iter()
            .find(|row| {
                DirPath::from_string(&row.canonical)
                    .unwrap()
                    .ends_with(&shortcut)
            }).map(|row| DirPath::from_string(&row.canonical).unwrap())
    }

    pub fn find_by_revision(&self, revision: usize) -> Option<DirPath> {
        self.0.get(revision)
            .map(|row| DirPath::from_string(&row.canonical).unwrap())
    }

    pub fn entries(self) -> Vec<Entry> {
        self.0
    }
}
