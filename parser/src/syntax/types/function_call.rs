use crate::syntax::types;
use ellie_core::defs;
use serde::Serialize;

use alloc::string::String;
use alloc::vec::Vec;


#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct FunctionCallParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}


#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct FunctionCall {
    pub name: String,
    pub name_pos: defs::Cursor,
    pub comma: bool,
    pub complete: bool,
    pub params: Vec<FunctionCallParameter>,
}
