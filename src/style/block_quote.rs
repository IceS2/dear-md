use super::Style;
use crossterm::style::{Color, ContentStyle, Stylize};

pub(crate) struct BlockQuoteStyle {
    style: ContentStyle,
    character: String,
}

impl BlockQuoteStyle {
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

impl Style for BlockQuoteStyle {
    fn style(&self) -> ContentStyle {
        self.style
    }
}

impl Default for BlockQuoteStyle {
    fn default() -> Self {
        let mut style = ContentStyle::new();
        style = style.with(Color::White);

        let character = "\u{2503}";
        BlockQuoteStyle::new(style, character)
    }
}
