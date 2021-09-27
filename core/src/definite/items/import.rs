use crate::defs;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    pub native: bool,
    pub public: bool,
    pub resolution_id: u64,
    pub id: u64,
    pub path_pos: defs::Cursor,
    pub pos: defs::Cursor,
}
