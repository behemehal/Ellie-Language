use alloc::string::String;
use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    pub pri_keyword: bool,
    pub native: bool,
    pub public: bool,
    pub path_pos: defs::Cursor,
    pub pos: defs::Cursor,
}