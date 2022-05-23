use pulldown_cmark::Event;

use crate::context::Context;
use crate::style::{Style, StyleSet};

use super::{StdoutHandler, TagHandler};

pub(crate) trait EventHandler<'a> {
    fn handle(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler, style_set: &StyleSet);
}

impl<'a> EventHandler<'a> for Event<'a> {
    fn handle(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler, style_set: &StyleSet) {
        match self {
            Event::Start(tag) => tag.start(context, stdout, style_set),
            Event::End(tag) => tag.end(context, stdout),
            Event::Text(text) => {
                context
                    .current_block()
                    .clone()
                    .handle_text(context, stdout, style_set, &text);
            }
            Event::Code(text) => {
                stdout.queue_styled_content(&text, style_set.code().style());
            }
            Event::SoftBreak => {
                stdout.queue_styled_content("\n", style_set.default().style());
            }
            Event::HardBreak => {
                stdout.queue_styled_content("\n", style_set.default().style());
            }
            Event::Rule => {
                stdout.queue_styled_content(style_set.rule().rule(), style_set.rule().style());
            }
            _ => (),
        }
    }
}
