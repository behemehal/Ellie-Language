pub mod array;
pub mod cloak;
pub mod collective;
pub mod function;
pub mod generic;
pub mod nullable;
pub mod parent_generic;

use crate::parser::Parser;
use alloc::vec::Vec;
use ellie_core::{definite::definers::DefinerCollecting, error};
use ellie_tokenizer::syntax::items::definers::DefinerTypes;

impl DefinerParserProcessor for DefinerTypes {
    fn process(
        &self,
        options: &mut DefinerParserProcessorOptions,
    ) -> Result<DefinerCollecting, Vec<error::Error>> {
        match self {
            DefinerTypes::Cloak(e) => e.process(options),
            DefinerTypes::Array(e) => e.process(options),
            DefinerTypes::Collective(e) => e.process(options),
            DefinerTypes::Nullable(e) => e.process(options),
            DefinerTypes::ParentGeneric(e) => e.process(options),
            DefinerTypes::Generic(e) => e.process(options),
            DefinerTypes::Function(e) => e.process(options),
            DefinerTypes::Dynamic => unreachable!(),
        }
    }
}

pub struct DefinerParserProcessorOptions<'a> {
    parser: &'a mut Parser,
    page_id: usize,
    ignore_hash: Option<usize>,
}

impl<'a> DefinerParserProcessorOptions<'a> {
    pub fn new(parser: &'a mut Parser, page_id: usize) -> Self {
        Self {
            parser,
            page_id,
            ignore_hash: None,
        }
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

pub trait DefinerParserProcessor {
    /// Definer parser processor
    /// ## Arguments
    /// * `options` - [`DefinerParserProcessorOptions`]
    /// ## Returns
    /// * [`Ok`] - [`DefinerCollecting`] if parsing should continue
    /// * [`Err`] - [`error::Error`] if parsing should not continue
    fn process(
        &self,
        options: &mut DefinerParserProcessorOptions,
    ) -> Result<DefinerCollecting, Vec<error::Error>>;
}
