use std::path::PathBuf;

use anyhow::{bail, Context};
use inquire::Select;
use shx_config::cdx::CdxConfig;
use shx_config::config::home;

use crate::history::{Entry, History};
use crate::theme::{SelectTheme, Theme};
use crate::theme::formatter::ToPretty;

pub struct CD;

impl CD {
    pub fn builtin(_config: &CdxConfig, history: &mut History, dest: String) -> anyhow::Result<PathBuf> {
        let path = match dest.as_str() {
            "" => home()?,
            "-" => Self::revision(_config, history, 2)?,
            _ => PathBuf::from(&dest),
        }.canonicalize().context("failed to canonicalize path.")?;
        let row = Entry::new(&dest, &path);
        history.append_last(row);
        Ok(path)
    }

    pub fn shortcut(config: &CdxConfig, history: &mut History, input: String) -> anyhow::Result<PathBuf> {
        let input_path = PathBuf::from(input.clone());
        let search_size = config.search_size();

        for entry in history.read(search_size) {
            let p = PathBuf::from(&entry.canonical);
            if p.ends_with(&input_path) {
                let to_append = entry.with_raw(format!("^{}", input));
                history.append_last(to_append);
                return Ok(p);
            }
        }
        bail!("[error] failed to find history by shortcut `{}`", input);
    }

    pub fn revision(config: &CdxConfig, history: &mut History, revision: usize) -> anyhow::Result<PathBuf> {
        let search_size = config.search_size();
        if revision <= 0 || revision > search_size {
            bail!("[error] revision {} is out of range. (0 < r <={})", revision, search_size);
        }

        if let Some(entry) = history.read(search_size).get(revision) {
            let p = PathBuf::from(&entry.canonical);
            let to_append = entry.with_raw(format!("^{}", revision));
            history.append_last(to_append);
            return Ok(p);
        }

        bail!("failed to find history by revision {}", revision);
    }

    pub fn interactive(config: &CdxConfig, history: &mut History) -> anyhow::Result<PathBuf> {
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

        let to_append = selection.to_entry()
            .with_raw("^(selected)".to_string());
        let p = PathBuf::from(&to_append.canonical);
        history.append_last(to_append);
        return Ok(p);
    }
}
