use crate::defs;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GenericDefining {
    pub name: String,
    pub hash: usize,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Extend {
    pub target: usize,
    pub target_name: String,
    pub target_generics: Vec<GenericDefining>,
    pub target_pos: defs::Cursor,

    pub from: usize,
    pub from_name: String,
    pub from_generics: Vec<GenericDefining>,
    pub from_pos: defs::Cursor,

    pub pos: defs::Cursor,
    pub inner_page_id: usize,
}
