use ellie_core::defs;
use serde::Serialize;
use alloc::string::String;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct Import {
    pub path: String,
    pub public: bool,
    pub path_pos: defs::Cursor,
    pub pos: defs::Cursor,
}