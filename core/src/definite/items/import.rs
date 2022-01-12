use crate::defs;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    pub link_module: bool,
    pub public: bool,
    pub reference: String,
    pub path_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub hash: u64,
}
