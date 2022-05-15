use crate::definite::definers;
use crate::defs;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]

pub struct Setter {
    pub name: String,
    pub pos: defs::Cursor,
    pub public: bool,
    pub name_pos: defs::Cursor,
    pub rtype_pos: defs::Cursor,
    pub param_name_pos: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub inner_page_id: u64,
    pub hash: u64,
    pub param_name: String,
    pub parameters_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
}
