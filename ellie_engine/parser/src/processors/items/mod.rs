use crate::parser::Parser;

pub mod brk;
pub mod class;
pub mod condition;
pub mod constructor;
pub mod enum_type;
pub mod file_key;
pub mod for_loop;
pub mod function;
pub mod generic_item;
pub mod getter;
pub mod getter_call;
pub mod go;
pub mod import;
pub mod loop_type;
pub mod ret;
pub mod setter;
pub mod setter_call;
pub mod variable;

pub struct ItemParserProcessorOptions<'a> {
    pub parser: &'a mut Parser,
    pub page_idx: usize,
    pub processed_page_idx: usize,
    pub page_hash: usize,
}

impl<'a> ItemParserProcessorOptions<'a> {
    pub fn build(
        parser: &'a mut Parser,
        page_idx: usize,
        processed_page_idx: usize,
        page_hash: usize,
    ) -> Self {
        Self {
            parser,
            page_idx,
            processed_page_idx,
            page_hash,
        }
    }
}

pub trait ItemParserProcessor {
    /// Item parser processor
    /// ## Arguments
    /// * `options` - [`ItemParserProcessorOptions`]
    /// ## Returns
    /// [`bool`] - `true` if parsing should continue
    fn process(&self, options: &mut ItemParserProcessorOptions) -> bool;
}
