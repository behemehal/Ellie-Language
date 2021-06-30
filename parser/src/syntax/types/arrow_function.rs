use crate::parser::Collecting;
use crate::syntax::{definers, function};
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::string::String;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ArrowFunction {
    pub parameters: Vec<function::FunctionParameterCollector>,
    pub return_type: Box<definers::DefinerCollecting>,
    pub inside_code: Vec<Collecting>,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ArrowFunctionCollector {
    pub complete: bool,
    pub param_bracket_opened: bool,
    pub parameter_wrote: bool,
    pub pointer_typed: bool,
    pub return_typed: bool,
    pub brace_count: usize,
    pub data: ArrowFunction,
    pub code: String,
}
