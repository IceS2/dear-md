use super::Style;
use crossterm::style::{Color, ContentStyle, Stylize};

pub(crate) struct CodeStyle {
    style: ContentStyle,
}

impl CodeStyle {
    fn new(style: ContentStyle) -> Self {
        Self { style }
    }
}

impl Style for CodeStyle {
    fn style(&self) -> ContentStyle {
        self.style
    }
}

impl Default for CodeStyle {
    fn default() -> Self {
        let mut style = ContentStyle::new();
        style = style.on(Color::DarkBlue);
        CodeStyle::new(style)
    }
}
