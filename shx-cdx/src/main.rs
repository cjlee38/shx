use std::fmt::Debug;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use clap::{Args, Parser};
use shx_config::config::{config, home};

use crate::cli::{Cli, DirArgs};
use crate::history::{Entry, History};
use crate::path::DirPath;

mod cli;
mod history;
mod path;

fn main() -> ExitCode {
    let cli = Cli::parse();
    eprintln!("{:?}", cli); // TODO : temp

    return if cli.show_history {
        match show_history() {
            Err(e) => {
                eprintln!("{}", e);
                ExitCode::FAILURE
            }
            Ok(_) => ExitCode::SUCCESS,
        }
    } else {
        let result = match cli.dir() {
            DirArgs::BulitIn(s) => cd_builtin(s),
            DirArgs::Shortcut(s) => cd_shortcut(s),
            DirArgs::Revision(i) => cd_revision(i),
            DirArgs::Interactive => cd_interactive(),
        };

        match result {
            Err(e) => {
                eprintln!("{}", e);
                ExitCode::FAILURE
            }
            Ok(dir) => {
                println!("{}", dir);
                ExitCode::SUCCESS
            }
        }
    };
}

fn show_history() -> anyhow::Result<()> {
    eprintln!("show history");
    let config = config()?;
    let search_size = config.cdx_config.search_size.ok_or(anyhow!("search_size is not set"))?;
    let history = History::read(search_size)?;
    history.entries()
        .iter()
        .for_each(|entry| {
            println!("{}", entry.canonical);
        });
    Ok(())
}

fn cd_builtin(input: String) -> anyhow::Result<DirPath> {
    let dir = match input.as_str() {
        "" => {
            let home = home()?;
            DirPath::from_path(home)
        }
        "-" => {
            todo!()
        }
        _ => DirPath::from_string(&input),
    }?;

    let row = Entry::from_dir(&dir)
        .context(format!("failed to create history row for {}", dir))?;

    // read 1 for check if last row is same with input
    History::read(1)?
        .append(&row)?;
    Ok(dir)
}

fn cd_shortcut(input: String) -> anyhow::Result<DirPath> {
    let dir = PathBuf::from(input.clone());
    let config = config().context("failed to load config")?;
    let search_size = config.cdx_config.search_size.context("search_size is not set")?;
    let history = History::read(search_size)?;

    let found = history.find_by_shortcut(dir)
        .ok_or(anyhow!("failed to find history by shortcut {}", input))?;
    let to_append = Entry::from_special(input, &found)?;
    history.append(&to_append)?;
    Ok(found)
}

fn cd_revision(i: usize) -> anyhow::Result<DirPath> {
    let config = config()?;
    let search_size = config.cdx_config.search_size.ok_or(anyhow!("search_size is not set"))?;
    let history = History::read(search_size)?;

    let found = history.find_by_revision(i)
        .ok_or(anyhow!("failed to find history by revision {}", i))?;
    let to_append = Entry::from_special(i.to_string(), &found)?;
    history.append(&to_append)?;
    Ok(found)
}

fn cd_interactive() -> anyhow::Result<DirPath> {
    todo!()
}
