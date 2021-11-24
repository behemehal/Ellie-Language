use crate::definite::types;
use crate::defs;
use serde::{Deserialize, Serialize};
use alloc::{boxed::Box, vec::Vec};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub target: Box<types::Types>,
    pub target_pos: defs::Cursor,
    pub params: Vec<FunctionCallParameter>,
}
