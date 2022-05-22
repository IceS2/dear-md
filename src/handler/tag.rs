use super::StdoutHandler;

use crate::context::Context;
use crate::style::{
    self, BlockQuoteStyle, CodeBlockStyle, OrderedListStyle, Style, UnorderedListStyle,
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
    fn start(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler);
    fn end(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler);
    fn handle_text(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler, text: &str);
}

impl<'a> TagHandler<'a> for Tag<'a> {
    //  TODO: Think about having a prefix modifier with some lifecycle instead of actually queuing here
    fn start(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler) {
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
                let block_quote_style = BlockQuoteStyle::default();
                context.set_current_block(self.clone());
                stdout.queue_content(&format!("{:width$}", "", width = context.indentation() * 2));
                stdout.queue_styled_content(
                    &format!("{} ", block_quote_style.character()),
                    block_quote_style.style(),
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
                        let ordered_list_style = OrderedListStyle::default();
                        stdout.queue_styled_content(
                            &format!("{}{} ", order, ordered_list_style.character()),
                            ordered_list_style.style(),
                        );
                        context.set_current_block(Tag::List(Some(order + 1)));
                    }
                    _ => {
                        let unoredered_list_style = UnorderedListStyle::default();
                        stdout.queue_styled_content(
                            &format!("{} ", unoredered_list_style.character()),
                            unoredered_list_style.style(),
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

    fn handle_text(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler, text: &str) {
        match self {
            Tag::CodeBlock(_) => {
                let code_block_style = CodeBlockStyle::default();
                for line in text.lines() {
                    let formatted_line = format!(
                        "{}{:>width$}\n",
                        line,
                        "",
                        width = (code_block_style.width() - line.len())
                    );
                    let ranges: Vec<(syntect::highlighting::Style, &str)> = code_block_style
                        .highlight_lines(context.code_block_syntax())
                        .highlight_line(&formatted_line, code_block_style.syntax_set())
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
                    Some(_) => OrderedListStyle::default().style(),
                    None => UnorderedListStyle::default().style(),
                };
                stdout.queue_styled_content(&text, list_style);
            }
            _ => {
                let mut style = match context.current_block() {
                    Tag::Paragraph => style::ParagraphStyle::default().style(),
                    Tag::Heading(level, ..) => match level {
                        HeadingLevel::H1 => style::HeadingStyle1::default().style(),
                        _ => style::HeadingStyle2::default().style(),
                    },
                    Tag::BlockQuote => style::BlockQuoteStyle::default().style(),
                    Tag::List(order) => match order {
                        Some(_) => style::OrderedListStyle::default().style(),
                        None => style::UnorderedListStyle::default().style(),
                    },
                    _ => style::DefaultStyle::default().style(),
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
