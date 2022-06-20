use crate::defs;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

use super::file_key::FileKey;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GenericDefining {
    pub name: String,
    pub hash: usize,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub public: bool,
    pub inner_page_id: usize,
    pub generic_definings: Vec<GenericDefining>,
    pub name_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub file_keys: Vec<FileKey>,
    pub hash: usize,
}
