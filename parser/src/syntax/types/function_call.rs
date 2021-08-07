use crate::syntax::types;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

use alloc::string::String;
use alloc::vec::Vec;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub name_pos: defs::Cursor,
    pub params: Vec<FunctionCallParameter>,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallCollector {
    pub data: FunctionCall,
    pub name_collected: bool,
    pub comma: bool,
    pub complete: bool,
}
