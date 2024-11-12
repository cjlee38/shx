use std::fmt::Display;

use crate::history::Entry;
use crate::theme::Theme;

pub trait ToPretty {
    fn prettify<T>(&self, desc: bool, theme: &T) -> Vec<String>
    where
        T: Theme;
}

impl ToPretty for Vec<&Entry> {
    fn prettify<T>(&self, desc: bool, theme: &T) -> Vec<String>
    where
        T: Theme,
    {
        let mut v = self.iter()
            .enumerate()
            .map(|(i, entry)| {
                let index = theme.index().apply_to(format!("{}", i + 1));
                let canonical = theme.canonical().apply_to(entry.canonical.clone());
                let raw = theme.raw().apply_to(format!("<{}>", &entry.raw));

                format!("{index}: {canonical} {raw}")
            })
            .collect::<Vec<String>>();
        if desc {
            v.reverse();
        }
        v
    }
}