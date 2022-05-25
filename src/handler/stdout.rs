use crossterm::{
    queue,
    style::{Print, PrintStyledContent, ResetColor},
};
use std::io::{stdout, Stdout, Write};

use crate::style::Content;

pub(crate) struct StdoutHandler {
    stdout: Stdout,
}

impl StdoutHandler {
    fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub(crate) fn queue_styled_content_v2(&mut self, contents: Vec<Content>) -> () {
        for content in contents {
            match content {
                Content::StyledContent(content) => {
                    queue!(self.stdout, PrintStyledContent(content)).unwrap()
                }
                Content::String(content) => queue!(self.stdout, Print(content)).unwrap(),
            }
            self.reset_color();
        }
    }

    fn reset_color(&mut self) -> () {
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
