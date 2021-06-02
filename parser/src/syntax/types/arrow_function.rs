use crate::parser::Collecting;
use crate::syntax::{definers, function};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use serde::Serialize;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;

<<<<<<< HEAD
=======
<<<<<<< HEAD
=======

>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
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
    pub inside_code_string: String,
    pub return_typed: bool,
    pub brace_count: i64,
    pub data: ArrowFunction,
}
