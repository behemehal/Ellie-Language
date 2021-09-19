use crate::parser::Collecting;
use crate::syntax::definers;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionParameter {
    pub name: String,
    pub rtype: definers::DefinerCollecting,
    pub pos: defs::Cursor,
    pub multi_capture: bool,
    pub name_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionParameterCollector {
    pub named: bool,
    pub colon_expected: bool,
    pub child_brace: usize,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Function {
    pub name: String,                             //Function Name string
    pub parameters: Vec<FunctionParameter>,       //Parameter vector
    pub return_type: definers::DefinerCollecting, //Return type from enum
    pub public: bool,
    pub inside_code: Vec<Collecting>,
    pub name_pos: defs::Cursor,           //Name position fn [test] ......
    pub code_bracket_start: defs::Cursor, //Bracket start fn test() > String [{]
    pub code_bracket_end: defs::Cursor,   //Bracket start fn test() > String { ... [}]
    pub parameters_pos: defs::Cursor,
    pub return_pos: defs::Cursor,
    pub pos: defs::Cursor,
}

impl Function {
    pub fn to_definite(self) -> definite::items::function::Function {
        definite::items::function::Function {
            name: self.name,
            parameters: self
                .parameters
                .into_iter()
                .map(|x| definite::items::function::FunctionParameter {
                    name: x.name,
                    rtype: x.rtype.to_definite(),
                    pos: x.pos,
                    multi_capture: x.multi_capture,
                    name_pos: x.name_pos,
                    type_pos: x.type_pos,
                })
                .collect(),
            return_type: self.return_type.to_definite(),
            public: self.public,
            inside_code: self
                .inside_code
                .into_iter()
                .map(|x| x.to_definite())
                .collect(),
            name_pos: self.name_pos,
            code_bracket_start: self.code_bracket_start,
            code_bracket_end: self.code_bracket_end,
            parameters_pos: self.parameters_pos,
            return_pos: self.return_pos,
            pos: self.pos,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionCollector {
    pub data: Function,
    pub collecting_parameters: FunctionParameterCollector, //Parameter vector
    pub initialized: bool,
    pub named: bool,                //Function named
    pub parameter_wrote: bool,      //Parameter type complete
    pub return_typed: bool,         //Function return typed
    pub return_pointer_typed: bool, // > typed
    pub at_comma: bool,
    pub brace_count: usize,
    pub code: Box<crate::parser::RawParser>,
}

impl FunctionCollector {
    pub fn to_definite(self) -> definite::items::function::Function {
        self.data.to_definite()
    }

    pub fn from_definite(self, from: definite::items::function::Function) -> Self {
        FunctionCollector {
            data: Function {
                name: from.name,
                parameters: from
                    .parameters
                    .into_iter()
                    .map(|x| FunctionParameter {
                        name: x.name,
                        rtype: definers::DefinerCollecting::default().from_definite(x.rtype),
                        pos: x.pos,
                        multi_capture: x.multi_capture,
                        name_pos: x.name_pos,
                        type_pos: x.type_pos,
                    })
                    .collect(),
                return_type: definers::DefinerCollecting::default().from_definite(from.return_type),
                public: from.public,
                inside_code: from
                    .inside_code
                    .into_iter()
                    .map(|x| Collecting::default().from_definite(x))
                    .collect(),
                name_pos: from.name_pos,
                code_bracket_start: from.code_bracket_start,
                code_bracket_end: from.code_bracket_end,
                parameters_pos: from.parameters_pos,
                return_pos: from.return_pos,
                pos: from.pos,
            },
            ..Default::default()
        }
    }

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
