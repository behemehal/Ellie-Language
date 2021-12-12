use crate::definite::types;
use crate::defs;
use alloc::{boxed::Box, vec::Vec};
use serde::{Deserialize, Serialize};

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
    pub pos: defs::Cursor,
}
