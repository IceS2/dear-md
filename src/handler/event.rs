use pulldown_cmark::Event;

use crate::context::Context;
use crate::style::{Content, StyleSet};

use super::{StdoutHandler, TagHandler};

pub(crate) trait EventHandler<'a> {
    fn handle(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler, style_set: &StyleSet);
}

impl<'a> EventHandler<'a> for Event<'a> {
    fn handle(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler, style_set: &StyleSet) {
        // println!("{:?}", self);
        match self {
            Event::Start(tag) => tag.start(context),
            Event::End(tag) => tag.end(context, stdout),
            Event::Text(text) => {
                context
                    .current_block()
                    .clone()
                    .handle_text(context, stdout, style_set, &text);
            }
            Event::Code(text) => {
                stdout.queue_styled_content_v2(style_set.code().get_styled_content(&text, context));
            }
            Event::SoftBreak => {
                stdout.queue_styled_content_v2(vec![Content::String(" ".to_string())]);
            }
            Event::HardBreak => {
                context.set_start_of_line(true);
                stdout.queue_styled_content_v2(vec![Content::String("\n".to_string())]);
            }
            Event::Rule => {
                context.set_start_of_line(true);
                stdout.queue_styled_content_v2(style_set.rule().get_styled_content());
            }
            _ => (),
        }
    }
}
