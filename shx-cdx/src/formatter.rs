use crate::history::Entry;

pub trait ToPretty {
    fn prettify(&self) -> Vec<String>;
}

impl ToPretty for Vec<&Entry> {
    fn prettify(&self) -> Vec<String> {
        self.iter()
            .enumerate()
            .map(|(i, entry)| format!("{}: {} ({})", i + 1, entry.canonical, entry.raw))
            .collect::<Vec<String>>()
    }
}