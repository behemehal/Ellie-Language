use crate::defs;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GenericDefining {
    pub name: String,
    pub hash: u64,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub public: bool,
    pub inner_page_id: u64,
    pub generic_definings: Vec<GenericDefining>,
    pub name_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub hash: u64,
}
