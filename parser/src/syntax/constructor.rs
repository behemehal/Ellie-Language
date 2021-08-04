use crate::parser::Collecting;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstructorParameter {
    pub name: String,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Constructor {
    pub name: String,                          //Function Name string
    pub parameters: Vec<ConstructorParameter>, //Parameter vector
    pub inside_code: Vec<Collecting>,
    pub name_pos: defs::Cursor, //Name position fn [test] ......
    pub parameters_pos: defs::Cursor,
    pub pos: defs::Cursor
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstructorCollector {
    pub data: Constructor,
    pub named: bool,
    pub parameter_wrote: bool,
    pub collecting: bool,
    pub brace_count: usize,
    pub has_code: bool,
    pub at_comma: bool,
    pub code: String,
}

impl ConstructorCollector {
    pub fn is_parameters_complete(&self) -> bool {
        if self.data.parameters.is_empty() {
            true
        } else {
            !self.data.parameters[self.data.parameters.len() - 1]
                .name
                .is_empty()
        }
    }
}
