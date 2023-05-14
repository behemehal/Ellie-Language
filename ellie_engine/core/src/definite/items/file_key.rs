use crate::definite::types;
use crate::defs;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FileKey {
    pub key_name: String,
    pub value: types::Types,
    pub key_name_location: defs::Cursor,
    pub is_global: bool,
    pub value_location: defs::Cursor,
    pub pos: defs::Cursor,
}
