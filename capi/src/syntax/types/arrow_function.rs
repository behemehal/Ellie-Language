use crate::parser::Collecting;
use crate::syntax::{definers, function};
use libc::c_char;

#[repr(C)]
pub struct ArrowFunction {
    pub parameters: *const function::FunctionParameterCollector,
    pub return_type: Box<definers::DefinerCollecting>,
    pub inside_code: *const Collecting,
}

#[repr(C)]
pub struct ArrowFunctionCollector {
    pub complete: bool,
    pub param_bracket_opened: bool,
    pub parameter_wrote: bool,
    pub pointer_typed: bool,
    pub inside_code_string: *const c_char,
    pub return_typed: bool,
    pub brace_count: i64,
    pub data: ArrowFunction,
}
