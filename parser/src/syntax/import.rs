use alloc::string::String;
use ellie_core::defs;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct Import {
    pub path: String,
    pub pri_keyword: bool,
    pub public: bool,
    pub path_pos: defs::Cursor,
    pub pos: defs::Cursor,
}
