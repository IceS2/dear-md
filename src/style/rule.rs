use super::Style;
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

    pub(crate) fn rule(&self) -> &str {
        &self.rule
    }
}

impl Style for RuleStyle {
    fn style(&self) -> ContentStyle {
        self.style
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
