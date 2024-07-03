use crate::{definite::types, defs};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayEntry {
    pub value: types::Types,
    pub location: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayType {
    pub collective: Vec<ArrayEntry>,
    pub pos: defs::Cursor,
}
