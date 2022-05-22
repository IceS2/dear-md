// use crossterm::style::{ContentStyle, Color, Stylize};
use syntect::{
    easy::HighlightLines,
    highlighting::{Theme, ThemeSet},
    parsing::SyntaxSet,
};
// use super::Style;

pub(crate) struct CodeBlockStyle {
    syntax_set: SyntaxSet,
    theme: Theme,
    width: usize,
}

impl CodeBlockStyle {
    fn new(width: usize, theme: &str) -> Self {
        let theme_set = ThemeSet::load_defaults();
        let syntax_set = SyntaxSet::load_defaults_newlines();

        Self {
            syntax_set,
            theme: theme_set.themes[theme].clone(),
            width,
        }
    }

    pub(crate) fn syntax_set(&self) -> &SyntaxSet {
        &self.syntax_set
    }

    pub(crate) fn width(&self) -> &usize {
        &self.width
    }

    pub(crate) fn highlight_lines(&self, syntax_name: &str) -> HighlightLines {
        let syntax = self.syntax_set.find_syntax_by_name(syntax_name).unwrap();
        HighlightLines::new(syntax, &self.theme)
    }
}

// impl Style for CodeBlockStyle {
//     fn style(&self) -> ContentStyle {
//         self.style
//     }
// }

impl Default for CodeBlockStyle {
    fn default() -> Self {
        let width = 80;
        let theme = "base16-ocean.dark";
        CodeBlockStyle::new(width, theme)
    }
}
