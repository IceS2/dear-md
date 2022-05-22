use super::Style;
use crossterm::style::{Color, ContentStyle, Stylize};

pub(crate) struct ParagraphStyle {
    style: ContentStyle,
}

impl ParagraphStyle {
    fn new(style: ContentStyle) -> Self {
        Self { style }
    }
}

impl Style for ParagraphStyle {
    fn style(&self) -> ContentStyle {
        self.style
    }
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        let mut style = ContentStyle::new();
        style = style.with(Color::White);
        ParagraphStyle::new(style)
    }
}
