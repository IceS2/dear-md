use super::Style;
use crossterm::style::{Color, ContentStyle, Stylize};

pub(crate) struct UnorderedListStyle {
    style: ContentStyle,
    character: String,
}

impl UnorderedListStyle {
    fn new(style: ContentStyle, character: &str) -> Self {
        Self {
            style,
            character: character.to_owned(),
        }
    }

    pub(crate) fn character(&self) -> &str {
        &self.character
    }
}

impl Style for UnorderedListStyle {
    fn style(&self) -> ContentStyle {
        self.style
    }
}

impl Default for UnorderedListStyle {
    fn default() -> Self {
        let mut style = ContentStyle::new();
        style = style.with(Color::White);

        let character = "\u{2727}";
        UnorderedListStyle::new(style, character)
    }
}

pub(crate) struct OrderedListStyle {
    style: ContentStyle,
    character: String,
}

impl OrderedListStyle {
    fn new(style: ContentStyle, character: &str) -> Self {
        Self {
            style,
            character: character.to_owned(),
        }
    }

    pub(crate) fn character(&self) -> &str {
        &self.character
    }
}

impl Style for OrderedListStyle {
    fn style(&self) -> ContentStyle {
        self.style
    }
}

impl Default for OrderedListStyle {
    fn default() -> Self {
        let mut style = ContentStyle::new();
        style = style.with(Color::White);

        let character = ".";
        OrderedListStyle::new(style, character)
    }
}
