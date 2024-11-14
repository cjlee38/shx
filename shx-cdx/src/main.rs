use std::fmt::{Debug, Display};
use std::io::BufRead;
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr;

use anyhow::{anyhow, bail, Context};
use clap::{Args, Parser};
use clap::builder::TypedValueParser;
use inquire::Select;
use shx_config::cdx::CdxConfig;
use shx_config::config::{config, home};

use crate::cli::{Cli, DirArgs};
use theme::formatter::ToPretty;
use crate::history::{Entry, History};
use crate::path::DirPath;
use crate::theme::{SelectTheme, Theme};

mod cli;
mod history;
mod path;

mod theme;

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

    return if cli.show_history {
        show_history(config, history)
    } else {
        let result = match cli.dir() {
            DirArgs::BulitIn(dest) => cd_builtin(&config, &mut history, dest),
            DirArgs::Shortcut(shortcut) => cd_shortcut(&config, &mut history, shortcut),
            DirArgs::Revision(revision) => cd_revision(&config, &mut history, revision),
            DirArgs::Interactive => cd_interactive(&config, &mut history),
        };

        history.save()?;
        result.map(|it| format!("{}", it))
    };
}

fn show_history(config: CdxConfig, history: History) -> anyhow::Result<String> {
    let search_size = config.search_size();

    let output = history.read(search_size)
        .iter()
        .enumerate()
        .map(|(index, entry)| entry.prettify(index, &Theme::default()))
        .map(|it| it.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    Ok(output)
}

fn cd_builtin(_config: &CdxConfig, history: &mut History, dest: String) -> anyhow::Result<DirPath> {
    let dir = match dest.as_str() {
        "" => {
            let home = home()?;
            DirPath::from_path(home)?.canonicalize()
        }
        "-" => {
            return cd_revision(_config, history, 2);
        }
        _ => DirPath::from_string(&dest)?.canonicalize(),
    }?;

    let row = Entry::from_dir(&dir)
        .context(format!("[error] failed to create history entry for {}", dir))?;

    history.append_last(row);
    Ok(dir)
}

fn cd_shortcut(config: &CdxConfig, history: &mut History, input: String) -> anyhow::Result<DirPath> {
    let dir = PathBuf::from(input.clone());
    let search_size = config.search_size();

    let found = history.read(search_size).iter()
        .map(|entry| DirPath::from_string(&entry.canonical).unwrap())
        .find(|dir_path| dir_path.ends_with(&dir))
        .ok_or(anyhow!("[error] failed to find history by shortcut {}", input))?;

    let to_append = Entry::from_special(input, &found)?;
    history.append_last(to_append);
    Ok(found)
}

fn cd_revision(config: &CdxConfig, history: &mut History, revision: usize) -> anyhow::Result<DirPath> {
    let search_size = config.search_size();
    if revision <= 0 || revision > search_size {
        bail!("[error] revision {} is out of range. (0 < r <={})", revision, search_size);
    }
    let found = history.read(search_size)
        .get(revision) // input revision starts with 1
        .map(|entry| DirPath::from_string(&entry.canonical).unwrap())
        .ok_or(anyhow!("[error] failed to find history by revision {}", revision))?;
    let to_append = Entry::from_special(revision.to_string(), &found)?;
    history.append_last(to_append);
    Ok(found)
}

fn cd_interactive(config: &CdxConfig, history: &mut History) -> anyhow::Result<DirPath> {
    let search_size = config.search_size();
    let theme = Theme::default();
    let selections = history.read(search_size)
        .iter()
        .enumerate()
        .map(|(index, entry)| entry.prettify(index, &theme))
        .collect::<Vec<_>>();

    let render_config = theme.render_config();
    let selection = Select::new("Pick a directory to change", selections)
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .with_render_config(render_config)
        .prompt()?;

    let entry = selection.to_entry();
    let dir_path = DirPath::from_string(&entry.canonical)?;
    history.append_last(entry);
    return Ok(dir_path);
}
