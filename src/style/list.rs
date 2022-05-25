use crate::context::Context;

use super::Content;
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

    pub(crate) fn get_styled_content(&self, text: &str, context: &Context) -> Vec<Content> {
        let mut contents: Vec<Content> = vec![];
        let mut style = self.style;
        if context.start_of_line().to_owned() {
            contents.push(Content::StyledContent(style.apply(format!(
                "\n{:width$}{character} ",
                "",
                width = context.indentation() * 2,
                character = self.character
            ))));
        }
        for modifier in context.modifiers() {
            style = style.attribute(*modifier)
        }

        contents.push(Content::StyledContent(style.apply(text.to_string())));
        contents
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

    pub(crate) fn get_styled_content(
        &self,
        text: &str,
        context: &Context,
        order: &u64,
    ) -> Vec<Content> {
        let mut contents: Vec<Content> = vec![];
        let mut style = self.style;
        if context.start_of_line().to_owned() {
            contents.push(Content::StyledContent(style.apply(format!(
                "\n{:width$}{order}{character} ",
                "",
                order = order,
                width = context.indentation() * 2,
                character = self.character
            ))));
        }
        for modifier in context.modifiers() {
            style = style.attribute(*modifier)
        }

        contents.push(Content::StyledContent(style.apply(text.to_string())));
        contents
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
