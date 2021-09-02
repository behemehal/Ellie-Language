use crate::definite::types;
use crate::defs;
use serde::{Deserialize, Serialize};

use alloc::string::String;
use alloc::vec::Vec;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub name_pos: defs::Cursor,
    pub params: Vec<FunctionCallParameter>,
}
