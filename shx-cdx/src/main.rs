use std::fmt::Debug;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use clap::{Args, Parser};
use shx_config::config::config;

use crate::cli::{CdxArgs, CdxCommand};
use crate::history::{CdxHistory, CdxHistoryRow};
use crate::path::DirPath;

mod cli;
mod history;
mod path;

fn main() -> ExitCode {
    let command = CdxCommand::parse();
    eprintln!("{:?}", command); // temp
    let result = match command.args {
        CdxArgs::BulitIn(s) => run_builtin(s),
        CdxArgs::Shortcut(s) => run_shortcut(s),
        CdxArgs::Interactive => run_interactive(),
        CdxArgs::Revision(_) => run_revision(),
    };

    return match result {
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
        Ok(dir) => {
            println!("{}", dir);
            ExitCode::SUCCESS
        }
    }
}

fn run_builtin(input: String) -> anyhow::Result<DirPath> {
    let dir = DirPath::from_string(&input)?;

    let row = CdxHistoryRow::from(&dir)
        .context(format!("failed to create history row for {}", dir))?;
    CdxHistory::append(&row)
        .context("failed to append history")?;
    Ok(dir)
}

fn run_shortcut(input: String) -> anyhow::Result<DirPath> {
    let dir = PathBuf::from(input.clone());
    let config = config().ok_or(anyhow!("failed to load config"))?;
    let search_size = config.cdx_config.search_size.ok_or(anyhow!("search_size is not set"))?;
    let history = CdxHistory::read(search_size)
        .ok_or(anyhow!("failed to read history"))?;

    let found = history.find_by_shortcut(dir)
        .ok_or(anyhow!("failed to find history by shortcut {}", input))?;
    Ok(found)
}

fn run_interactive() -> anyhow::Result<DirPath> {
    todo!()
}

fn run_revision() -> anyhow::Result<DirPath> {
    todo!()
}
