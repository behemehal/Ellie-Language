use crate::parser::Collecting;
use crate::syntax::{definers, function};
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArrowFunction {
    pub parameters: Vec<function::FunctionParameter>,
    pub return_type: definers::DefinerCollecting,
    pub inside_code: Vec<Collecting>,
    pub return_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArrowFunctionCollector {
    pub collecting_parameters: function::FunctionParameterCollector,
    pub complete: bool,
    pub param_bracket_opened: bool,
    pub parameter_wrote: bool,
    pub pointer_typed: bool,
    pub return_typed: bool,
    pub brace_count: usize,
    pub data: ArrowFunction,
    pub code: String,
}

impl ArrowFunctionCollector {
    pub fn has_dedup(&self) -> bool {
        let mut existent_names: Vec<String> = Vec::with_capacity(self.data.parameters.len());
        let mut duplicate = false;
        for i in &self.data.parameters {
            if existent_names.contains(&i.name) {
                duplicate = true;
                break;
            } else {
                existent_names.push(i.name.clone())
            }
        }
        duplicate
    }
}
