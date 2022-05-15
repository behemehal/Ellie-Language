use crate::definite::definers;
use crate::defs;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]

pub struct Getter {
    pub name: String,
    pub name_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub public: bool,
    pub return_type: definers::DefinerCollecting,
    pub return_pos: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub inner_page_id: u64,
    pub hash: u64,
}
