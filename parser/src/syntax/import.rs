use alloc::string::String;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Import {
    pub path: String,
    pub pri_keyword: bool,
    pub native: bool,
    pub public: bool,
    pub path_pos: defs::Cursor,
    pub pos: defs::Cursor,
}
