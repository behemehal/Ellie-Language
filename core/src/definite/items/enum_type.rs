use crate::definite::definers;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EnumItem {
    pub has_type: bool,
    pub identifier: String,
    pub enum_type: definers::DefinerCollecting,
    pub identifier_pos: crate::defs::Cursor,
    pub type_pos: crate::defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EnumType {
    pub public: bool,
    pub name: String,
    pub name_pos: crate::defs::Cursor,
    pub pos: crate::defs::Cursor,
    pub brace_start_pos: crate::defs::Cursor,
    pub brace_end_pos: crate::defs::Cursor,
    pub items: Vec<EnumItem>,
}
