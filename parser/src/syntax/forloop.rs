use crate::syntax::{variable, types};
use alloc::string::String;
use ellie_core::defs;
use serde::Serialize;
use crate::parser::Collecting;
use crate::alloc::vec::Vec;
use crate::alloc::boxed::Box;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct Forloop {
    pub parameter: Box<types::Types>,
    pub parameter_pos: defs::Cursor,
    pub code: String,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ForloopCollector {
    pub parameters_collected: bool,
    pub cloak_itered_data: variable::VariableCollector,
    pub brace_count: usize,
    pub has_code: bool,
    pub data: Forloop,

    pub inside_code: Vec<Collecting>,

    pub inside_code_string: String,
    pub inside_object_start: bool,
    pub inside_object_count: i64,
}
