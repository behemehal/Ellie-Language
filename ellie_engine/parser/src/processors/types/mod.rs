pub mod array;
pub mod as_keyword;
pub mod brace_reference;
pub mod class_call;
pub mod cloak;
pub mod collective;
pub mod function_call;
pub mod negative;
pub mod null_resolver;
pub mod operator;
pub mod reference;
pub mod variable;

use crate::parser::Parser;
use alloc::vec::Vec;
use ellie_core::{definite::types, defs::Cursor, error};
use ellie_tokenizer::processors::types::Processors;

impl TypeParserProcessor for Processors {
    fn process(
        &self,
        options: &mut TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        match self {
            Processors::Variable(e) => e.process(options),
            Processors::Negative(e) => e.process(options),
            Processors::Array(e) => e.process(options),
            Processors::Operator(e) => e.process(options),
            Processors::Reference(e) => e.process(options),
            Processors::BraceReference(e) => e.process(options),
            Processors::FunctionCall(e) => e.process(options),
            Processors::ClassCall(e) => e.process(options),
            Processors::Cloak(e) => e.process(options),
            Processors::Collective(e) => e.process(options),
            Processors::AsKeyword(e) => e.process(options),
            Processors::NullResolver(e) => e.process(options),
            _ => Ok(self.to_definite()),
        }
    }
}

pub struct TypeParserProcessorOptions<'a> {
    exclude_getter: bool,
    include_setter: bool,
    variable_pos: Option<Cursor>,
    ignore_type: bool,
    ignore_hash: Option<usize>,
    parser: &'a mut Parser,
    page_id: usize,
}

impl<'a> TypeParserProcessorOptions<'a> {
    pub fn new(parser: &'a mut Parser, page_id: usize) -> Self {
        Self {
            exclude_getter: false,
            include_setter: false,
            variable_pos: None,
            parser,
            page_id,
            ignore_type: false,
            ignore_hash: None,
        }
    }

    pub fn clone(&'a mut self) -> TypeParserProcessorOptions<'a> {
        TypeParserProcessorOptions::new(self.parser, self.page_id)
    }

    pub fn copy_with_parser(&self, parser: &'a mut Parser) -> TypeParserProcessorOptions<'a> {
        TypeParserProcessorOptions {
            exclude_getter: self.exclude_getter,
            include_setter: self.include_setter,
            variable_pos: self.variable_pos,
            parser,
            page_id: self.page_id,
            ignore_type: self.ignore_type,
            ignore_hash: self.ignore_hash,
        }
    }

    pub fn exclude_getter(&mut self) -> &mut Self {
        self.exclude_getter = true;
        self
    }

    pub fn dont_exclude_getter(&mut self) -> &mut Self {
        self.exclude_getter = false;
        self
    }

    pub fn include_setter(&mut self) -> &mut Self {
        self.include_setter = true;
        self
    }

    pub fn dont_include_setter(&mut self) -> &mut Self {
        self.include_setter = false;
        self
    }

    pub fn variable_pos(&mut self, pos: Cursor) -> &mut Self {
        self.variable_pos = Some(pos);
        self
    }

    pub fn optional_variable_pos(&mut self, pos: Option<Cursor>) -> &mut Self {
        self.variable_pos = pos;
        self
    }

    pub fn ignore_type(&mut self) -> &mut Self {
        self.ignore_type = true;
        self
    }

    pub fn dont_ignore_type(&mut self) -> &mut Self {
        self.ignore_type = false;
        self
    }

    pub fn ignore_hash(&mut self, hash: usize) -> &mut Self {
        self.ignore_hash = Some(hash);
        self
    }

    pub fn optional_ignore_hash(&mut self, hash: Option<usize>) -> &mut Self {
        self.ignore_hash = hash;
        self
    }

    pub fn build(&mut self) -> &mut Self {
        self
    }
}

pub trait TypeParserProcessor {
    /// Parser element processor
    /// ## Arguments
    /// * `options` - [`TypeParserProcessorOptions`]
    /// ## Returns
    /// * [`Ok`] - [`types::Types`] if parsing should continue
    /// * [`Err`] - [`error::Error`] if parsing should not continue
    fn process(
        &self,
        options: &mut TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>>;
}
