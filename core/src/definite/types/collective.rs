use crate::definite::types;
use crate::defs;
use alloc::{vec::Vec, string::String};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveEntry {
    pub key: String,
    pub key_pos: defs::Cursor,
    pub value: types::Types,
    pub value_pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveType {
    pub entries: Vec<CollectiveEntry>,
    pub pos: defs::Cursor,
}
