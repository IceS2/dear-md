use std::path::PathBuf;

use dear_md::print_markdown_file;

fn main() {
    print_markdown_file(PathBuf::from("text"));
}
