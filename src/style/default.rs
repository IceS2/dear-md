use super::Style;
use crossterm::style::ContentStyle;

pub(crate) struct DefaultStyle {
    style: ContentStyle,
}

impl DefaultStyle {
    fn new(style: ContentStyle) -> Self {
        Self { style }
    }
}

impl Style for DefaultStyle {
    fn style(&self) -> ContentStyle {
        self.style
    }
}

impl Default for DefaultStyle {
    fn default() -> Self {
        let style = ContentStyle::new();
        DefaultStyle::new(style)
    }
}
