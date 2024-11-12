use dialoguer::console::Style;
use dialoguer::theme::ColorfulTheme;

pub trait Theme {
    fn index(&self) -> Style;
    fn canonical(&self) -> Style;
    fn raw(&self) -> Style;
}

pub struct PlainTheme;
impl Theme for PlainTheme {
    fn index(&self) -> Style {
        Style::new()
    }

    fn canonical(&self) -> Style {
        Style::new()
    }

    fn raw(&self) -> Style {
        Style::new()
    }
}

impl Theme for ColorfulTheme {
    fn index(&self) -> Style {
        self.active_item_style.clone()
    }

    fn canonical(&self) -> Style {
        self.inactive_item_style.clone()
    }

    fn raw(&self) -> Style {
        self.hint_style.clone()
    }
}