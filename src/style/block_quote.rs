use crossterm::style::{Color, ContentStyle, Stylize};

use super::Content;
use crate::context::Context;

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

    pub(crate) fn get_styled_content(&self, text: &str, context: &Context) -> Vec<Content> {
        let mut style = self.style;
        for modifier in context.modifiers() {
            style = style.attribute(*modifier)
        }
        if context.start_of_line().to_owned() {
            vec![Content::StyledContent(style.apply(format!(
                "{:width$}{character} {text}",
                "",
                width = context.indentation() * 2,
                character = self.character,
                text = text
            )))]
        } else {
            vec![Content::StyledContent(style.apply(text.to_string()))]
        }
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
