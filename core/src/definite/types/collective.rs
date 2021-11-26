use crate::definite::types;
use crate::defs;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveEntry {
    pub key: types::Types,
    pub value: types::Types,
    pub key_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveType {
    pub entries: Vec<CollectiveEntry>,
}
