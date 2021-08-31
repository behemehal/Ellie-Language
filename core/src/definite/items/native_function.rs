use crate::definite::definers;
use alloc::string::String;
use alloc::vec::Vec;
use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NativeFunctionParameter {
    pub name: String,
    pub rtype: definers::DefinerCollecting,
    pub pos: defs::Cursor,
    pub multi_capture: bool,
    pub name_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
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