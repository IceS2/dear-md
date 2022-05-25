use super::Content;
use crossterm::style::{Color, ContentStyle, Stylize};

pub(crate) struct RuleStyle {
    style: ContentStyle,
    rule: String,
}

impl RuleStyle {
    fn new(style: ContentStyle, rule: &str) -> Self {
        Self {
            style,
            rule: rule.to_owned(),
        }
    }

    pub(crate) fn get_styled_content(&self) -> Vec<Content> {
        vec![Content::StyledContent(self.style.apply(self.rule.clone()))]
    }
}

impl Default for RuleStyle {
    fn default() -> Self {
        let mut style = ContentStyle::new();
        style = style.with(Color::DarkGrey);

        let rule = format!("  {:\u{2500}<1$}\n", "", 80);
        RuleStyle::new(style, &rule)
    }
}
