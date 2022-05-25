use super::StdoutHandler;

use crate::context::Context;
use crate::style::{Content, StyleSet};

use crossterm::style::Attribute;
use pulldown_cmark::{CodeBlockKind, HeadingLevel, Tag};

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub(crate) trait TagHandler<'a> {
    fn start(&self, context: &mut Context<'a>);
    fn end(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler);
    fn handle_text(
        &self,
        context: &mut Context<'a>,
        stdout: &mut StdoutHandler,
        style_set: &StyleSet,
        text: &str,
    );
}

impl<'a> TagHandler<'a> for Tag<'a> {
    //  TODO: Think about having a prefix modifier with some lifecycle instead of actually queuing here
    fn start(&self, context: &mut Context<'a>) {
        match self {
            Tag::Paragraph => {
                if std::mem::discriminant(context.current_block())
                    != std::mem::discriminant(&Tag::BlockQuote)
                {
                    context.set_current_block(self.clone());
                }
                context.set_start_of_line(true);
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
                context.set_start_of_line(true);
            }
            Tag::Emphasis => {
                context.add_modifier(Attribute::Underlined);
            }
            Tag::Strong => {
                context.add_modifier(Attribute::Bold);
            }
            _ => (),
        }
    }

    fn end(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler) {
        match self {
            Tag::Paragraph => {
                stdout.queue_styled_content_v2(vec![Content::String("\n\n".to_string())]);
            }
            Tag::Heading(..) => {
                stdout.queue_styled_content_v2(vec![Content::String("\n\n".to_string())]);
            }
            Tag::CodeBlock(..) => {
                context.set_indentation(context.indentation() - 1);
                stdout.queue_styled_content_v2(vec![Content::String("\n".to_string())]);
            }
            Tag::BlockQuote => {
                stdout.queue_styled_content_v2(vec![Content::String("\n".to_string())]);
            }
            Tag::List(_) => {
                if context.indentation() > &0 {
                    context.set_indentation(context.indentation() - 1);
                }
                stdout.queue_styled_content_v2(vec![Content::String("\n".to_string())]);
            }
            Tag::Item => {
                let current_block = context.current_block().clone();
                match current_block {
                    Tag::List(Some(order)) => {
                        context.set_current_block(Tag::List(Some(order + 1)));
                    }
                    _ => {}
                }
            }
            Tag::Emphasis => {
                context.remove_modifier(Attribute::Underlined);
            }
            Tag::Strong => {
                context.remove_modifier(Attribute::Bold);
            }
            _ => (),
        }
    }

    fn handle_text(
        &self,
        context: &mut Context<'a>,
        stdout: &mut StdoutHandler,
        style_set: &StyleSet,
        text: &str,
    ) {
        match self {
            Tag::CodeBlock(_) => {
                stdout.queue_styled_content_v2(
                    style_set.code_block().get_styled_content(text, context),
                );
            }
            Tag::List(order) => {
                let content = match order {
                    Some(o) => style_set
                        .ordered_list()
                        .get_styled_content(text, context, o),
                    None => style_set.unordered_list().get_styled_content(text, context),
                };
                stdout.queue_styled_content_v2(content);
            }
            Tag::BlockQuote => {
                let styled_content = style_set.block_quote().get_styled_content(text, context);
                stdout.queue_styled_content_v2(styled_content)
            }
            _ => {
                let contents = match context.current_block() {
                    Tag::Paragraph => style_set.paragraph().get_styled_content(text, context),
                    Tag::Heading(level, ..) => style_set
                        .heading(HeadingLevelWrapper::new(level).into())
                        .get_styled_content(text, context),
                    _ => style_set.default().get_styled_content(text, context),
                };
                stdout.queue_styled_content_v2(contents);
            }
        }
        context.set_start_of_line(false);
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
