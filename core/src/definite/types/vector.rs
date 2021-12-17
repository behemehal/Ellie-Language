use crate::definite::types;
use crate::defs;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct VectorEntry {
    pub value: types::Types,
    pub location: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct VectorType {
    pub collective: Vec<VectorEntry>,
    pub pos: defs::Cursor,
}
