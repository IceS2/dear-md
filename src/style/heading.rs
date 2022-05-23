use super::Style;
use crossterm::style::{Attribute, Color, ContentStyle, Stylize};

pub(crate) struct HeadingStyle {
    style: ContentStyle,
}

impl HeadingStyle {
    fn new(style: ContentStyle) -> Self {
        Self { style }
    }
}

impl Style for HeadingStyle {
    fn style(&self) -> ContentStyle {
        self.style
    }
}

impl Default for HeadingStyle {
    fn default() -> Self {
        let mut style = ContentStyle::new();
        style = style.with(Color::Yellow);
        style = style.attribute(Attribute::Bold);

        HeadingStyle::new(style)
    }
}
