use std::fmt::Display;

use inquire::ui::{Attributes, RenderConfig, Styled, StyleSheet};

use color::ColorBridge;

pub mod color;
pub mod formatter;

pub struct Theme {
    prompt: ColorBridge,
    user_input: ColorBridge,
    index: ColorBridge,
    canonical: ColorBridge,
    raw: ColorBridge,
    selected: ColorBridge,
    assist: ColorBridge,
}

impl Theme {
    pub fn prompt(&self) -> ColorBridge {
        self.prompt.clone()
    }

    pub fn index(&self) -> ColorBridge {
        self.index.clone()
    }

    pub fn canonical(&self) -> ColorBridge {
        self.canonical.clone()
    }

    pub fn raw(&self) -> ColorBridge {
        self.raw.clone()
    }

    pub fn selected(&self) -> ColorBridge {
        self.selected.clone()
    }

    pub fn assist(&self) -> ColorBridge {
        self.assist.clone()
    }
}

pub trait SelectTheme {
    fn render_config(&self) -> RenderConfig;
    fn prompt_stylesheet(&self) -> StyleSheet;
    fn user_input_stylesheet(&self) -> StyleSheet;
    fn index_stylesheet(&self) -> StyleSheet;
    fn canonical_stylesheet(&self) -> StyleSheet;
    fn raw_stylesheet(&self) -> StyleSheet;
    fn selected_stylesheet(&self) -> StyleSheet;
    fn assist_stylesheet(&self) -> StyleSheet;
}


impl SelectTheme for Theme {
    fn render_config(&self) -> RenderConfig {
        let mut render_config = RenderConfig::default();

        // prompt
        render_config.prompt_prefix = Styled::new("$").with_style_sheet(self.prompt_stylesheet());
        render_config.prompt = self.prompt_stylesheet();

        // user input
        render_config.answer = self.user_input_stylesheet().with_attr(Attributes::BOLD);
        render_config.text_input = self.user_input_stylesheet();

        // options(a.k.a. selections) will be styled by itself
        render_config.highlighted_option_prefix = Styled::new(">").with_style_sheet(self.selected_stylesheet());

        // helps
        render_config.help_message = self.assist_stylesheet();
        render_config
    }

    fn prompt_stylesheet(&self) -> StyleSheet {
        self.prompt.to_stylesheet()
    }

    fn user_input_stylesheet(&self) -> StyleSheet {
        self.user_input.to_stylesheet()
    }

    fn index_stylesheet(&self) -> StyleSheet {
        self.index.to_stylesheet()
    }

    fn canonical_stylesheet(&self) -> StyleSheet {
        self.canonical.to_stylesheet()
    }

    fn raw_stylesheet(&self) -> StyleSheet {
        self.raw.to_stylesheet()
    }

    fn selected_stylesheet(&self) -> StyleSheet {
        self.selected.to_stylesheet()
    }

    fn assist_stylesheet(&self) -> StyleSheet {
        self.assist.to_stylesheet()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            prompt: ColorBridge::LightRed,
            user_input: ColorBridge::White,
            index: ColorBridge::LightYellow,
            canonical: ColorBridge::LightGreen,
            raw: ColorBridge::LightBlue,
            assist: ColorBridge::LightYellow,
            selected: ColorBridge::LightGreen,
        }
    }
}