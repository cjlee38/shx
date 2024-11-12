use std::fmt::{Debug, Display};
use std::io::BufRead;
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr;

use anyhow::{anyhow, bail, Context};
use clap::{Args, Parser};
use dialoguer::console::{Style, style};
use dialoguer::FuzzySelect;
use dialoguer::theme::ColorfulTheme;
use shx_config::cdx::CdxConfig;
use shx_config::config::{config, home};

use crate::cli::{Cli, DirArgs};
use crate::formatter::ToPretty;
use crate::history::{Entry, History};
use crate::path::DirPath;

mod cli;
mod history;
mod path;
mod formatter;
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

    let vec = history.read(search_size)
        .prettify(true, &ColorfulTheme::default());
    let output = vec
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
            return cd_revision(_config, history, 1);
        }
        _ => DirPath::from_string(&dest)?.canonicalize(),
    }?;

    let row = Entry::from_dir(&dir)
        .context(format!("[error] failed to create history entry for {}", dir))?;

    history.append_last(row);
    Ok(dir)
}

fn cd_shortcut(config: &CdxConfig, history: &mut History, input: String) -> anyhow::Result<DirPath> {
    // TODO : fuzzy find, but declarative.
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
    let revision = revision;
    let search_size = config.search_size();
    if revision <= 0 || revision > search_size {
        bail!("[error] revision {} is out of range. (0 < r <={})", revision, search_size);
    }
    let found = history.read(search_size)
        .get(revision - 1) // input revision starts with 1
        .map(|entry| DirPath::from_string(&entry.canonical).unwrap())
        .ok_or(anyhow!("[error] failed to find history by revision {}", revision))?;
    let to_append = Entry::from_special(revision.to_string(), &found)?;
    history.append_last(to_append);
    Ok(found)
}

fn cd_interactive(config: &CdxConfig, history: &mut History) -> anyhow::Result<DirPath> {
    let search_size = config.search_size();
    let entries = history.read(search_size);

    let selections = entries.prettify(false, &ColorfulTheme::default());

    let mut theme = ColorfulTheme::default();
    theme.prompt_suffix = style(">>>".to_string()).for_stderr().red();
    theme.active_item_style = Style::new().for_stderr();
    theme.active_item_prefix = theme.active_item_prefix.bright();
    theme.fuzzy_match_highlight_style = Style::new().for_stderr().bold().red();

    let selection = FuzzySelect::with_theme(&theme)
        .with_prompt("Pick a directory or you can search")
        .default(0)
        .items(selections.as_slice())
        .interact()?;

    let selected_entry = entries[selection];
    let selected_dir = DirPath::from_string(&selected_entry.canonical)?;
    let to_append = Entry::from_special("(selected)".to_string(), &selected_dir)?;

    history.append_last(to_append);
    Ok(selected_dir)
}
