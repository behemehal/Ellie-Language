use crate::definite::definers;
use crate::defs;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NativeFunction {
    pub name: String,                             //NativeFunction Name string
    pub parameters: Vec<super::function::FunctionParameter>, //Parameter vector
    pub return_type: definers::DefinerCollecting, //Return type from enum
    pub public: bool,
    pub name_pos: defs::Cursor, //Name position fn [test] ......
    pub parameters_pos: defs::Cursor,
    pub return_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub hash: String,
    pub no_return: bool,
}
