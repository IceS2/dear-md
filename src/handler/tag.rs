use super::StdoutHandler;

use crate::context::Context;
use crate::style::{
    BlockQuoteStyle, Style, StyleSet,
};

use crossterm::style::{Attribute, Stylize};
use pulldown_cmark::{CodeBlockKind, HeadingLevel, Tag};
use syntect::util::as_24_bit_terminal_escaped;

pub(crate) fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub(crate) trait TagHandler<'a> {
    fn start(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler, style_set: &StyleSet);
    fn end(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler);
    fn handle_text(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler, style_set: &StyleSet, text: &str);
}

impl<'a> TagHandler<'a> for Tag<'a> {
    //  TODO: Think about having a prefix modifier with some lifecycle instead of actually queuing here
    fn start(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler, style_set: &StyleSet) {
        match self {
            Tag::Paragraph => {
                context.set_current_block(self.clone());
            }
            Tag::Heading(..) => {
                context.set_current_block(self.clone());
            }
            Tag::CodeBlock(kind) => {
                context.set_current_block(self.clone());
                context.set_indentation(context.indentation() + 1);
                match kind {
                    CodeBlockKind::Indented => {
                        context.set_code_block_syntax("Plain Text");
                    }
                    CodeBlockKind::Fenced(syntax) => {
                        context.set_code_block_syntax(&capitalize(syntax));
                    }
                }
            }
            Tag::BlockQuote => {
                context.set_current_block(self.clone());
                stdout.queue_content(&format!("{:width$}", "", width = context.indentation() * 2));
                stdout.queue_styled_content(
                    &format!("{} ", style_set.block_quote().character()),
                    style_set.block_quote().style(),
                );
            }
            Tag::List(_) => {
                if std::mem::discriminant(context.current_block())
                    == std::mem::discriminant(&Tag::List(None))
                {
                    context.set_indentation(context.indentation() + 1);
                }
                context.set_current_block(self.clone());
            }
            Tag::Item => {
                stdout.queue_content(&format!(
                    "\n{:width$}",
                    "",
                    width = context.indentation() * 2
                ));

                let current_block = context.current_block().clone();
                match current_block {
                    Tag::List(Some(order)) => {
                        stdout.queue_styled_content(
                            &format!("{}{} ", order, style_set.ordered_list().character()),
                            style_set.ordered_list().style(),
                        );
                        context.set_current_block(Tag::List(Some(order + 1)));
                    }
                    _ => {
                        stdout.queue_styled_content(
                            &format!("{} ", style_set.unordered_list().character()),
                            style_set.unordered_list().style(),
                        );
                    }
                }
            }
            Tag::Emphasis => context.add_modifier(Attribute::Underlined),
            Tag::Strong => context.add_modifier(Attribute::Bold),
            _ => (),
        }
    }

    fn end(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler) {
        match self {
            Tag::Paragraph => {
                stdout.queue_content("\n\n");
            }
            Tag::Heading(..) => {
                stdout.queue_content("\n\n");
            }
            Tag::CodeBlock(..) => {
                context.set_indentation(context.indentation() - 1);
                stdout.queue_content("\n");
            }
            Tag::BlockQuote => {
                stdout.queue_content("\n");
            }
            Tag::List(_) => {
                if context.indentation() > &0 {
                    context.set_indentation(context.indentation() - 1);
                }
                stdout.queue_content("\n");
            }
            Tag::Item => (),
            Tag::Emphasis => {
                context.remove_modifier(Attribute::Underlined);
            }
            Tag::Strong => {
                context.remove_modifier(Attribute::Bold);
            }
            _ => (),
        }
    }

    fn handle_text(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler, style_set: &StyleSet, text: &str) {
        match self {
            Tag::CodeBlock(_) => {
                for line in text.lines() {
                    let formatted_line = format!(
                        "{}{:>width$}\n",
                        line,
                        "",
                        width = (style_set.code_block().width() - line.len())
                    );
                    let ranges: Vec<(syntect::highlighting::Style, &str)> = style_set.code_block()
                        .highlight_lines(context.code_block_syntax())
                        .highlight_line(&formatted_line, style_set.code_block().syntax_set())
                        .unwrap();
                    let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                    stdout.queue_content(&format!(
                        "{:width$}",
                        "",
                        width = context.indentation() * 2
                    ));
                    stdout.queue_content(&escaped);
                    stdout.reset_color();
                }
            }
            Tag::List(order) => {
                let list_style = match order {
                    Some(_) => style_set.ordered_list().style(),
                    None => style_set.unordered_list().style(),
                };
                stdout.queue_styled_content(&text, list_style);
            }
            _ => {
                let mut style = match context.current_block() {
                    Tag::Paragraph => style_set.paragraph().style(),
                    Tag::Heading(level, ..) => style_set.heading(HeadingLevelWrapper::new(level).into()).style(),
                    Tag::BlockQuote => style_set.block_quote().style(),
                    Tag::List(order) => match order {
                        Some(_) => style_set.ordered_list().style(),
                        None => style_set.unordered_list().style(),
                    },
                    _ => style_set.default().style(),
                };
                for modifier in context.modifiers() {
                    style = style.attribute(*modifier)
                }
                stdout.queue_content(&format!("{:width$}", "", width = context.indentation() * 2));
                stdout.queue_styled_content(&text, style);
            }
        }
    }
}

struct HeadingLevelWrapper(HeadingLevel);

impl HeadingLevelWrapper {
    fn new(level: &HeadingLevel) -> Self {
        Self(level.clone())
    }
}

impl From<HeadingLevelWrapper> for usize {
    fn from(level_wrapper: HeadingLevelWrapper) -> Self {
        match level_wrapper.0 {
            HeadingLevel::H1 => 0,
            HeadingLevel::H2 => 1,
            HeadingLevel::H3 => 2,
            HeadingLevel::H4 => 3,
            HeadingLevel::H5 => 4,
            HeadingLevel::H6 => 5,
        }
    }
}
