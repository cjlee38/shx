use std::str::FromStr;

use clap::{Args, Parser};

use crate::opts::Opts;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(
        index = 1,
        help = "Specifies the directory to jump.",
        value_parser = clap::value_parser!(DirArgs),
        group = "cd",
        conflicts_with = "opts"
    )]
    pub dir: Option<DirArgs>,

    #[arg(
        short = 's',
        long,
        action = clap::ArgAction::SetTrue,
        help = "Show cd history",
        group = "opts",
    )]
    pub show_history: bool,

    #[arg(
        short = 'l',
        long,
        help = "Learn",
        group = "opts",
    )]
    pub learn: Option<String>,
}

impl Cli {
    pub fn dir(&self) -> DirArgs {
        return self.dir.clone().unwrap_or(DirArgs::BulitIn("".to_string()));
    }

    pub fn opt(&self) -> Option<Opts> {
        if self.show_history {
            Some(Opts::ShowHistory)
        } else if let Some(learn) = &self.learn {
            Some(Opts::Learn(learn.clone()))
        } else {
            None
        }
    }
}

impl FromStr for DirArgs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.strip_prefix("^") {
            None => Ok(DirArgs::BulitIn(s.to_string())),
            Some(it) => {
                return if it.is_empty() {
                    Ok(DirArgs::Interactive)
                } else if let Ok(revision) = it.parse::<usize>() {
                    Ok(DirArgs::Revision(revision))
                } else {
                    Ok(DirArgs::Shortcut(it.to_string()))
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum DirArgs {
    BulitIn(String),
    Interactive,
    Shortcut(String),
    Revision(usize),
}