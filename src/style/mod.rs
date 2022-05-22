pub(crate) mod default;
pub(crate) use default::DefaultStyle;

pub(crate) mod paragraph;
pub(crate) use paragraph::ParagraphStyle;

pub(crate) mod heading;
pub(crate) use heading::{HeadingStyle1, HeadingStyle2};

pub(crate) mod block_quote;
pub(crate) use block_quote::BlockQuoteStyle;

pub(crate) mod rule;
pub(crate) use rule::RuleStyle;

pub(crate) mod list;
pub(crate) use list::{OrderedListStyle, UnorderedListStyle};

pub(crate) mod code;
pub(crate) use code::CodeStyle;

pub(crate) mod code_block;
pub(crate) use code_block::CodeBlockStyle;

use crossterm::style::ContentStyle;

pub(crate) trait Style {
    fn style(&self) -> ContentStyle;
}
