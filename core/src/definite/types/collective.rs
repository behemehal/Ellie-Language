use crate::definite::types;
use crate::defs;
use alloc::boxed::Box;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveEntry {
    pub key: Box<types::Types>,
    pub value: Box<types::Types>,
    pub key_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Collective {
    pub entries: Vec<CollectiveEntry>,
}
