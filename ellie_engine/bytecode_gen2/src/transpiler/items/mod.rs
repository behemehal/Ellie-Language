pub mod class;
pub mod condition;
pub mod function;
pub mod native_function;
pub mod ret;
pub mod variable;

use ellie_parser::parser::ProcessedPage;

use crate::assembler::Assembler;

pub trait Transpiler {
    /// Bytecode element transpiler
    /// ## Arguments
    /// * `parser` - [`Parser`]
    /// * `page_id` - [`u64`]
    /// ## Returns
    /// [`bool`] - `true` if parsing should continue
    fn transpile(
        &self,
        assembler: &mut Assembler,
        hash: usize,
        processed_page: &ProcessedPage,
    ) -> bool;
}
