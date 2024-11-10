use std::str::FromStr;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct CdxCommand {
    #[arg(
        help = "Specifies the directory",
        value_parser = clap::value_parser!(CdxArgs)
    )]
    pub args: CdxArgs,
}

impl FromStr for CdxArgs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.strip_prefix("^") {
            None => Ok(CdxArgs::BulitIn(s.to_string())),
            Some(it) => {
                return if it.is_empty() {
                    Ok(CdxArgs::Interactive)
                } else if let Ok(revision) = it.parse::<usize>() {
                    Ok(CdxArgs::Revision(revision))
                } else {
                    Ok(CdxArgs::Shortcut(it.to_string()))
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum CdxArgs {
    BulitIn(String),
    Interactive,
    Shortcut(String),
    Revision(usize),
}