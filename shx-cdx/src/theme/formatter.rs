use std::fmt::{Debug, Display};

use colored::Colorize;

use crate::history::Entry;
use crate::theme::color::StyledBridge;
use crate::theme::Theme;

pub trait ToPretty {
    fn prettify(&self, index: usize, theme: &Theme) -> StyledEntry;
}


impl ToPretty for Entry {
    fn prettify(&self, index: usize, theme: &Theme) -> StyledEntry {
        let index = StyledBridge::new(index, theme.index());
        let canonical = StyledBridge::new(self.canonical.clone(), theme.canonical());
        let raw = StyledBridge::new(self.raw.clone(), theme.raw());

        StyledEntry { index, canonical, raw }
    }
}

pub struct StyledEntry {
    index: StyledBridge<usize>,
    raw: StyledBridge<String>,
    canonical: StyledBridge<String>,
}

impl StyledEntry {
    pub fn to_entry(&self) -> Entry {
        Entry {
            raw: self.raw.content.clone(),
            canonical: self.canonical.content.clone(),
        }
    }
}

impl Display for StyledEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index = self.index.colorize();
        let canonical = self.canonical.colorize();
        let raw = self.raw.colorize_with(|it| format!("<{}>", it));

        write!(f, "{index}: {canonical} {raw}")
    }
}