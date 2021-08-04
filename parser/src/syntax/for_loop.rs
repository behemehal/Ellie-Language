use crate::alloc::boxed::Box;
use crate::alloc::vec::Vec;
use crate::parser::Collecting;
use crate::syntax::{types, variable};
use alloc::string::String;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForLoop {
    pub parameter: Box<types::Types>,
    pub parameter_pos: defs::Cursor,
    pub code: String,
    pub pos: defs::Cursor
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForLoopCollector {
    pub parameters_collected: bool,
    pub cloak_itered_data: variable::VariableCollector,
    pub brace_count: usize,
    pub has_code: bool,
    pub data: ForLoop,

    pub inside_code: Vec<Collecting>,

    pub inside_code_string: String,
    pub inside_object_start: bool,
    pub inside_object_count: i64,
}
