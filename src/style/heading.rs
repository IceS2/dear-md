use super::Style;
use crossterm::style::{Attribute, Color, ContentStyle, Stylize};

pub(crate) struct HeadingStyle1 {
    style: ContentStyle,
}

impl HeadingStyle1 {
    fn new(style: ContentStyle) -> Self {
        Self { style }
    }
}

impl Style for HeadingStyle1 {
    fn style(&self) -> ContentStyle {
        self.style
    }
}

impl Default for HeadingStyle1 {
    fn default() -> Self {
        let mut style = ContentStyle::new();
        style = style.with(Color::Blue);
        style = style.attribute(Attribute::Bold);

        HeadingStyle1::new(style)
    }
}

pub(crate) struct HeadingStyle2 {
    style: ContentStyle,
}

impl HeadingStyle2 {
    fn new(style: ContentStyle) -> Self {
        Self { style }
    }
}

impl Style for HeadingStyle2 {
    fn style(&self) -> ContentStyle {
        self.style
    }
}

impl Default for HeadingStyle2 {
    fn default() -> Self {
        let mut style = ContentStyle::new();
        style = style.with(Color::Yellow);
        style = style.attribute(Attribute::Bold);

        HeadingStyle2::new(style)
    }
}
