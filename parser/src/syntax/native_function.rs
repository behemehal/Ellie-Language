use crate::syntax::definers;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct NativeFunctionParameter {
    pub name: String,
    pub rtype: definers::DefinerCollecting,
    pub pos: defs::Cursor,
    pub multi_capture: bool,
    pub name_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct NativeFunctionParameterCollector {
    pub named: bool,
    pub colon_expected: bool,
    pub child_brace: usize,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct NativeFunction {
    pub name: String,                             //NativeFunction Name string
    pub parameters: Vec<NativeFunctionParameter>, //Parameter vector
    pub return_type: definers::DefinerCollecting, //Return type from enum
    pub public: bool,
    pub name_pos: defs::Cursor, //Name position fn [test] ......
    pub parameters_pos: defs::Cursor,
    pub return_pos: defs::Cursor,
    pub pos: defs::Cursor,
}

impl NativeFunction {
    pub fn to_definite(self) -> definite::items::native_function::NativeFunction {
        definite::items::native_function::NativeFunction {
            name: self.name,
            parameters: self.parameters.into_iter().map(|x| {
                definite::items::native_function::NativeFunctionParameter {
                    name: x.name,
                    rtype: x.rtype.to_definite(),
                    pos: x.pos,
                    multi_capture: x.multi_capture,
                    name_pos: x.name_pos,
                    type_pos: x.type_pos,
                }
            }).collect(),
            return_type: self.return_type.to_definite(),
            public: self.public,
            name_pos: self.name_pos,
            parameters_pos: self.parameters_pos,
            return_pos: self.return_pos,
            pos: self.pos,
        }
    }

    pub fn from_runtime(func: crate::syntax::function::Function) -> NativeFunction {
        NativeFunction {
            name: func.name,
            parameters: func
                .parameters
                .into_iter()
                .map(|x| NativeFunctionParameter {
                    name: x.name,
                    rtype: x.rtype,
                    pos: x.pos,
                    multi_capture: x.multi_capture,
                    name_pos: x.name_pos,
                    type_pos: x.type_pos,
                })
                .collect(),
            return_type: func.return_type,
            public: func.public,
            name_pos: func.name_pos,
            parameters_pos: func.parameters_pos,
            return_pos: func.return_pos,
            pos: func.pos,
        }
    }
}
