use std::fmt::{Debug, Display};
use std::io::BufRead;
use std::process::ExitCode;
use std::str::FromStr;

use anyhow::Context;
use clap::{Args, Parser};
use clap::builder::TypedValueParser;
use shx_config::config::config;

use theme::formatter::ToPretty;

use crate::cd::CD;
use crate::cli::{Cli, DirArgs};
use crate::history::History;
use crate::opts::Opts;
use crate::theme::SelectTheme;

mod cli;
mod history;
mod path;

mod theme;
mod opts;
mod cd;

fn main() -> ExitCode {
    let exec = exec();
    match exec {
        Ok(it) => {
            println!("{}", it);
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("{}", e);
            ExitCode::FAILURE
        }
    }
}

fn exec() -> anyhow::Result<String>
{
    let cli = Cli::parse();
    let mut history = History::open()?;
    let config = config().unwrap().cdx_config;

    if let Some(opt) = cli.opt() {
        match opt {
            Opts::ShowHistory => Opts::show_history(config, history),
            Opts::Learn(dir) => Opts::learn(dir),
        }
    } else {
        let result = match cli.dir() {
            DirArgs::BulitIn(dest) => CD::builtin(&config, &mut history, dest),
            DirArgs::Shortcut(shortcut) => CD::shortcut(&config, &mut history, shortcut),
            DirArgs::Revision(revision) => CD::revision(&config, &mut history, revision),
            DirArgs::Interactive => CD::interactive(&config, &mut history),
        };

        history.save()?;
        result.map(|it| format!("{}", it.display()))
    }
}


