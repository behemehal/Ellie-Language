use crate::definite::definers;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

use super::file_key::FileKey;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum EnumValue {
    NoValue,
    Value(definers::DefinerCollecting),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EnumItem {
    pub identifier: String,
    pub identifier_pos: crate::defs::Cursor,
    pub type_pos: crate::defs::Cursor,
    pub value: EnumValue,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EnumType {
    pub public: bool,
    pub name: String,
    pub name_pos: crate::defs::Cursor,
    pub pos: crate::defs::Cursor,
    pub body_pos: crate::defs::Cursor,
    pub file_keys: Vec<FileKey>,
    pub items: Vec<EnumItem>,
    pub hash: usize,
}
