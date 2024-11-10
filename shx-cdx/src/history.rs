use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};

use anyhow::Context;
use shx_config::config::path_for;

use crate::path::DirPath;

const SEPARATOR: char = 0x1f as char;

pub struct CdxHistory(Vec<CdxHistoryRow>);

pub struct CdxHistoryRow {
    raw: String,
    canonical: String,
}

impl CdxHistoryRow {
    pub fn from(dir: &DirPath) -> anyhow::Result<Self> {
        Ok(CdxHistoryRow {
            raw: dir.to_string(),
            canonical: dir.canonicalize()?.to_string(),
        })
    }
}

impl CdxHistoryRow {
    fn serialize(&self) -> String {
        let mut s = String::new();
        s.push_str(self.raw.as_str());
        s.push(SEPARATOR);
        s.push_str(self.canonical.as_str());
        s.push('\n');
        return s;
    }
}

impl CdxHistory {
    pub fn read(size: usize) -> Option<Self> {
        return if let Some(path) = path_for("cd_history") {
            let file = OpenOptions::new()
                .read(true)
                .open(path)
                .ok()?;
            // This function can be improved in a efficient way, but left as it is for simplicity.
            let reader = BufReader::new(&file);
            let rows: Vec<CdxHistoryRow> = reader.lines()
                .map(|line| {
                    let text = line.unwrap();
                    let s = text.split(SEPARATOR).collect::<Vec<&str>>();
                    let raw = s[0].to_string();
                    let canonical = s[1].to_string();

                    CdxHistoryRow { raw, canonical }
                })
                .collect::<Vec<CdxHistoryRow>>()
                .into_iter()
                .take(size)
                .rev()
                .collect();
            Some(CdxHistory(rows))
        } else {
            None
        };
    }

    pub fn append(row: &CdxHistoryRow) -> anyhow::Result<()> {
        let path = path_for("cd_history").context("cd_history file not found")?;
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .context("failed to open cd_history file")?;
        let mut file = BufWriter::new(file);
        file.write_all(row.serialize().as_bytes())
            .context("failed to write to cd_history file")?;
        Ok(())
    }
}