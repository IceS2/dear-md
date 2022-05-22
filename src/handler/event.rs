use crossterm::style::ContentStyle;
use pulldown_cmark::Event;

use crate::context::Context;
use crate::style::{CodeStyle, RuleStyle, Style};

use super::{StdoutHandler, TagHandler};

pub(crate) trait EventHandler<'a> {
    fn handle(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler);
}

impl<'a> EventHandler<'a> for Event<'a> {
    fn handle(&self, context: &mut Context<'a>, stdout: &mut StdoutHandler) {
        match self {
            Event::Start(tag) => tag.start(context, stdout),
            Event::End(tag) => tag.end(context, stdout),
            Event::Text(text) => {
                context
                    .current_block()
                    .clone()
                    .handle_text(context, stdout, &text);
            }
            Event::Code(text) => {
                let code_style = CodeStyle::default();
                stdout.queue_styled_content(&text, code_style.style());
            }
            Event::SoftBreak => {
                stdout.queue_styled_content("\n", ContentStyle::new());
            }
            Event::HardBreak => {
                stdout.queue_styled_content("\n", ContentStyle::new());
            }
            Event::Rule => {
                let rule_style = RuleStyle::default();
                stdout.queue_styled_content(rule_style.rule(), rule_style.style());
            }
            _ => (),
        }
    }
}
