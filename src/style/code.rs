use crate::context::Context;

use super::Content;
use crossterm::style::{Color, ContentStyle, Stylize};

pub(crate) struct CodeStyle {
    style: ContentStyle,
}

impl CodeStyle {
    fn new(style: ContentStyle) -> Self {
        Self { style }
    }

    pub(crate) fn get_styled_content(&self, text: &str, context: &Context) -> Vec<Content> {
        let mut style = self.style;
        for modifier in context.modifiers() {
            style = style.attribute(*modifier)
        }
        if context.start_of_line().to_owned() {
            vec![Content::StyledContent(style.apply(format!(
                "{:width$}{text}",
                "",
                width = context.indentation() * 2,
                text = text
            )))]
        } else {
            vec![Content::StyledContent(style.apply(text.to_string()))]
        }
    }
}

impl Default for CodeStyle {
    fn default() -> Self {
        let mut style = ContentStyle::new();
        style = style.on(Color::DarkBlue);
        CodeStyle::new(style)
    }
}
