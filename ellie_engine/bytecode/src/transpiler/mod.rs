mod class;
mod class_instance;
mod condition;
mod constructor;
mod for_loop;
mod function;
mod function_parameter;
mod getter_call;
mod loop_type;
mod native_function;
mod ret;
mod setter_call;
mod type_resolver;
mod variable;
mod self_item;

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
