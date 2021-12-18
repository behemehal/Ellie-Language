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
    pub rtype_pos: defs::Cursor,
    pub bracket_start_pos: defs::Cursor,
    pub bracket_end_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
}
