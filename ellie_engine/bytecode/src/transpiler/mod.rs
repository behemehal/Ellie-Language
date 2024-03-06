pub mod class;
pub mod class_instance;
pub mod condition;
pub mod constructor;
pub mod for_loop;
pub mod function;
pub mod function_parameter;
pub mod getter_call;
pub mod loop_type;
pub mod native_function;
pub mod ret;
pub mod self_item;
pub mod setter_call;
pub mod type_resolver;
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
