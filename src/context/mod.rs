use crossterm::style::Attribute;
use pulldown_cmark::Tag;

#[derive(Debug)]
pub(crate) struct Context<'a> {
    current_block: Tag<'a>,
    code_block_syntax: String,
    indentation: usize,
    modifiers: Vec<Attribute>,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        Self {
            current_block: Tag::Paragraph,
            code_block_syntax: "Plain Text".to_owned(),
            indentation: 0,
            modifiers: vec![],
        }
    }

    pub(crate) fn current_block(&self) -> &Tag<'a> {
        &self.current_block
    }

    pub(crate) fn set_current_block(&mut self, block: Tag<'a>) -> () {
        self.current_block = block;
    }

    pub(crate) fn code_block_syntax(&self) -> &str {
        &self.code_block_syntax
    }

    pub(crate) fn set_code_block_syntax(&mut self, code_block_syntax: &str) -> () {
        self.code_block_syntax = code_block_syntax.to_owned();
    }

    pub(crate) fn indentation(&self) -> &usize {
        &self.indentation
    }

    pub(crate) fn set_indentation(&mut self, level: usize) -> () {
        self.indentation = level;
    }

    pub(crate) fn modifiers(&self) -> &Vec<Attribute> {
        &self.modifiers
    }

    pub(crate) fn add_modifier(&mut self, modifier: Attribute) -> () {
        self.modifiers.push(modifier)
    }

    pub(crate) fn remove_modifier(&mut self, modifier: Attribute) -> () {
        // TODO: Modify the way we are dealing with a possible error by checking if the item is in the vector beforehand
        self.modifiers.remove(
            self.modifiers
                .iter()
                .position(|attr| *attr == modifier)
                .unwrap(),
        );
    }
}

impl<'a> Default for Context<'a> {
    fn default() -> Self {
        Self::new()
    }
}
