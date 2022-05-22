mod context;
mod handler;
mod style;

use handler::{EventHandler, StdoutHandler};
use pulldown_cmark::{Options, Parser};
use std::{fs, path::PathBuf};

pub fn print_markdown_file(file: PathBuf) {
    let file_content = fs::read_to_string(file).unwrap();

    let mut context = context::Context::default();
    let mut stdout = StdoutHandler::default();
    let parser = Parser::new_ext(&file_content, Options::empty());

    for event in parser {
        event.handle(&mut context, &mut stdout);
    }

    stdout.flush();
}
