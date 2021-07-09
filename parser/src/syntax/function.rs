use crate::parser::Collecting;
use crate::syntax::definers;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::defs;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Default, Serialize, Hash)]
pub struct FunctionParameter {
    pub name: String,
    pub rtype: definers::DefinerCollecting,
    pub pos: defs::Cursor,
    pub name_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize, Hash)]
pub struct FunctionParameterCollector {
    pub data: FunctionParameter,
    pub named: bool,
    pub colon_expected: bool,
    pub child_brace: usize,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize, Hash)]
pub struct Function {
    pub name: String,                                //Function Name string
    pub parameters: Vec<FunctionParameterCollector>, //Parameter vector
    pub return_type: definers::DefinerCollecting,    //Return type from enum
    pub public: bool,
    pub inside_code: Vec<Collecting>,
    pub name_pos: defs::Cursor,           //Name position fn [test] ......
    pub code_bracket_start: defs::Cursor, //Bracket start fn test() > String [{]
    pub code_bracket_end: defs::Cursor,   //Bracket start fn test() > String { ... [}]
    pub parameters_pos: defs::Cursor,
    pub return_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize, Hash)]
pub struct FunctionCollector {
    pub data: Function,
    pub initialized: bool,
    pub named: bool,                //Function named
    pub parameter_wrote: bool,      //Parameter type complete
    pub return_typed: bool,         //Function return typed
    pub return_pointer_typed: bool, // > typed
    pub at_comma: bool,
    pub brace_count: usize,
    pub code: String,
}

impl FunctionCollector {
    pub fn has_dedup(&self) -> bool {
        let mut existent_names: Vec<String> = Vec::with_capacity(self.data.parameters.len());
        let mut duplicate = false;
        for i in &self.data.parameters {
            if existent_names.contains(&i.data.name) {
                duplicate = true;
                break;
            } else {
                existent_names.push(i.data.name.clone())
            }
        }
        duplicate
    }
}
