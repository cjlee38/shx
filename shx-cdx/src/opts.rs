use shx_config::cdx::CdxConfig;

use crate::history::History;
use crate::path::CanonicalPath;
use crate::theme::formatter::ToPretty;
use crate::theme::Theme;

pub enum Opts {
    ShowHistory,
    Learn(String),
}

impl Opts {
    pub fn show_history(config: CdxConfig, history: History) -> anyhow::Result<String> {
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

    pub fn learn(dir: String) -> anyhow::Result<String> {
        todo!()
    }
}