use crate::definite::definers;
use crate::defs;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub rtype: definers::DefinerCollecting,
    pub pos: defs::Cursor,
    pub multi_capture: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: definers::DefinerCollecting,
    pub inner_page_id: u64,
    pub public: bool,
    pub name_pos: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub no_return: bool,
    pub parameters_pos: defs::Cursor,
    pub return_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub hash: String,
}
