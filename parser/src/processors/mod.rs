use crate::parser::Parser;
pub mod class;
pub mod constructor;
pub mod definer_processor;
pub mod file_key;
pub mod function;
pub mod import;
pub mod ret;
pub mod type_processor;
pub mod variable;

pub trait Processor {
    /// Parser element processor
    /// ## Arguments
    /// * `parser` - [`Parser`]
    /// * `page_id` - [`u64`]
    /// ## Returns
    /// [`bool`] - `true` if parsing should continue
    fn process(self, parser: &mut Parser, page_id: u64) -> bool;
}
