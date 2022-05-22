use crossterm::{
    queue,
    style::{ContentStyle, Print, PrintStyledContent, ResetColor},
};
use std::io::{stdout, Stdout, Write};

pub(crate) struct StdoutHandler {
    stdout: Stdout,
}

impl StdoutHandler {
    fn new() -> Self {
        Self { stdout: stdout() }
    }
    pub(crate) fn queue_content(&mut self, content: &str) -> () {
        queue!(self.stdout, Print(content.to_owned())).unwrap();
    }

    pub(crate) fn queue_styled_content(&mut self, content: &str, style: ContentStyle) -> () {
        queue!(
            self.stdout,
            PrintStyledContent(style.apply(content.to_owned()))
        )
        .unwrap();
    }

    pub(crate) fn reset_color(&mut self) -> () {
        queue!(self.stdout, ResetColor).unwrap();
    }

    pub(crate) fn flush(&mut self) -> () {
        self.stdout.flush().unwrap();
    }
}

impl Default for StdoutHandler {
    fn default() -> Self {
        StdoutHandler::new()
    }
}
