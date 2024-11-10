use std::fmt::Debug;
use std::io::BufRead;
use std::process::ExitCode;
use std::str::FromStr;

use anyhow::Context;
use clap::{Args, Parser};

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
        CdxArgs::BulitIn(s) => { run_builtin(s) }
        CdxArgs::Interactive => {
            run_interactive()
        }
        CdxArgs::Shortcut(_) => {
            run_shortcut()
        }
        CdxArgs::Revision(_) => {
            run_revision()
        }
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run_builtin(directory: String) -> anyhow::Result<()> {
    let dir = DirPath::from_string(directory)?;

    let row = CdxHistoryRow::from(&dir)
        .context(format!("failed to create history row for {}", dir))?;
    CdxHistory::append(&row)
        .context("failed to append history")?;
    println!("{}", dir);
    Ok(())
}

fn run_revision() -> anyhow::Result<()> {
    todo!()
}

fn run_shortcut() -> anyhow::Result<()> {
    todo!()
}

fn run_interactive() -> anyhow::Result<()> {
    todo!()
}