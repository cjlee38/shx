use colored::{Color as ColoredColor, ColoredString, Colorize};
use inquire::ui::{Styled, StyleSheet};
use inquire::ui::Color as InquireColor;

/// A bridge between the `Color` from `colored` and `Styled` from `inquire` types.
#[derive(Clone, Copy)]
pub enum ColorBridge {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    LightRed,
    LightYellow,
    LightGreen,
    LightBlue,
}

impl ColorBridge {
    pub fn to_stylesheet(&self) -> StyleSheet {
        match self {
            ColorBridge::Red => StyleSheet::new().with_fg(InquireColor::LightRed),
            ColorBridge::Green => StyleSheet::new().with_fg(InquireColor::LightGreen),
            ColorBridge::Yellow => StyleSheet::new().with_fg(InquireColor::LightYellow),
            ColorBridge::Blue => StyleSheet::new().with_fg(InquireColor::LightBlue),
            ColorBridge::Magenta => StyleSheet::new().with_fg(InquireColor::LightMagenta),
            ColorBridge::Cyan => StyleSheet::new().with_fg(InquireColor::LightCyan),
            ColorBridge::White => StyleSheet::new().with_fg(InquireColor::White),
            ColorBridge::LightRed => StyleSheet::new().with_fg(InquireColor::LightRed),
            ColorBridge::LightYellow => StyleSheet::new().with_fg(InquireColor::LightYellow),
            ColorBridge::LightGreen => StyleSheet::new().with_fg(InquireColor::LightGreen),
            ColorBridge::LightBlue => StyleSheet::new().with_fg(InquireColor::LightBlue),
        }
    }

    pub fn to_colored(&self) -> ColoredColor {
        match self {
            ColorBridge::Red => ColoredColor::Red,
            ColorBridge::Green => ColoredColor::Green,
            ColorBridge::Yellow => ColoredColor::Yellow,
            ColorBridge::Blue => ColoredColor::Blue,
            ColorBridge::Magenta => ColoredColor::Magenta,
            ColorBridge::Cyan => ColoredColor::Cyan,
            ColorBridge::White => ColoredColor::White,
            ColorBridge::LightRed => ColoredColor::BrightRed,
            ColorBridge::LightYellow => ColoredColor::BrightYellow,
            ColorBridge::LightGreen => ColoredColor::BrightGreen,
            ColorBridge::LightBlue => ColoredColor::BrightBlue,
        }
    }
}

pub struct StyledBridge<T>
where
    T: std::fmt::Display,
{
    pub content: T,
    pub style: ColorBridge,
}

impl<T> StyledBridge<T>
where
    T: std::fmt::Display,
{
    pub fn new(content: T, style: ColorBridge) -> Self {
        Self { content, style }
    }

    pub fn to_styled(self) -> Styled<T> {
        Styled::new(self.content).with_style_sheet(self.style.to_stylesheet())
    }

    pub fn colorize(&self) -> ColoredString {
        self.content.to_string().color(self.style.to_colored())
    }

    pub fn colorize_with<F>(&self, f: F) -> ColoredString
    where
        F: Fn(&T) -> T,
    {
        f(&self.content).to_string()
            .color(self.style.to_colored())
    }
}