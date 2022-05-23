pub(crate) mod default;
pub(crate) use default::DefaultStyle;

pub(crate) mod paragraph;
pub(crate) use paragraph::ParagraphStyle;

pub(crate) mod heading;
pub(crate) use heading::HeadingStyle;

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

pub(crate) struct StyleSet {
    heading: Vec<HeadingStyle>,

    paragraph: ParagraphStyle,

    unordered_list: UnorderedListStyle,
    ordered_list: OrderedListStyle,
    block_quote: BlockQuoteStyle,

    code: CodeStyle,

    code_block: CodeBlockStyle,

    rule: RuleStyle,

    default: DefaultStyle,
}

impl StyleSet {
    pub(crate) fn heading(&self, level: usize) -> &HeadingStyle {
        let max_level = self.heading.len();

        if level >= max_level {
            &self.heading[max_level - 1]
        } else {
            &self.heading[level]
        }
    }

    pub(crate) fn paragraph(&self) -> &ParagraphStyle {
        &self.paragraph
    }

    pub(crate) fn unordered_list(&self) -> &UnorderedListStyle {
        &self.unordered_list
    }

    pub(crate) fn ordered_list(&self) -> &OrderedListStyle {
        &self.ordered_list
    }

    pub(crate) fn block_quote(&self) -> &BlockQuoteStyle {
        &self.block_quote
    }

    pub(crate) fn code(&self) -> &CodeStyle {
        &self.code
    }

    pub(crate) fn code_block(&self) -> &CodeBlockStyle {
        &self.code_block
    }

    pub(crate) fn rule(&self) -> &RuleStyle {
        &self.rule
    }

    pub(crate) fn default(&self) -> &DefaultStyle {
        &self.default
    }
}

pub(crate) struct StyleSetBuilder {
    heading: Option<Vec<HeadingStyle>>,

    paragraph: Option<ParagraphStyle>,

    unordered_list: Option<UnorderedListStyle>,
    ordered_list: Option<OrderedListStyle>,
    block_quote: Option<BlockQuoteStyle>,

    code: Option<CodeStyle>,

    code_block: Option<CodeBlockStyle>,

    rule: Option<RuleStyle>,

}

impl StyleSetBuilder {
    pub(crate) fn new() -> Self {
        Self {
            heading: None,
            paragraph: None,
            unordered_list: None,
            ordered_list: None,
            block_quote: None,
            code: None,
            code_block: None,
            rule: None,
        }
    }

    pub(crate) fn heading(mut self, heading: Vec<HeadingStyle>) -> Self {
        self.heading = Some(heading);
        self
    }

    pub(crate) fn paragraph(mut self, paragraph: ParagraphStyle) -> Self {
        self.paragraph = Some(paragraph);
        self
    }

    pub(crate) fn unordered_list(mut self, unordered_list: UnorderedListStyle) -> Self {
        self.unordered_list = Some(unordered_list);
        self
    }

    pub(crate) fn ordered_list(mut self, ordered_list: OrderedListStyle) -> Self {
        self.ordered_list = Some(ordered_list);
        self
    }

   pub(crate) fn block_quote(mut self, block_quote: BlockQuoteStyle) -> Self {
        self.block_quote = Some(block_quote);
        self
    }

    pub(crate) fn code(mut self, code: CodeStyle) -> Self {
        self.code = Some(code);
        self
    }

    pub(crate) fn code_block(mut self, code_block: CodeBlockStyle) -> Self {
        self.code_block = Some(code_block);
        self
    }

    pub(crate) fn rule(mut self, rule: RuleStyle) -> Self {
        self.rule = Some(rule);
        self
    }

    pub(crate) fn build(self) -> StyleSet {
        StyleSet {
            heading: self.heading.unwrap_or_else(|| vec![HeadingStyle::default()]),
            paragraph: self.paragraph.unwrap_or_else(|| ParagraphStyle::default()),
            unordered_list: self.unordered_list.unwrap_or_else(|| UnorderedListStyle::default()),
            ordered_list: self.ordered_list.unwrap_or_else(|| OrderedListStyle::default()),
            block_quote: self.block_quote.unwrap_or_else(|| BlockQuoteStyle::default()),
            code: self.code.unwrap_or_else(|| CodeStyle::default()),
            code_block: self.code_block.unwrap_or_else(|| CodeBlockStyle::default()),
            rule: self.rule.unwrap_or_else(|| RuleStyle::default()),
            default: DefaultStyle::default()
        }
    }
}
