use crate::definite::types;
use alloc::string::String;
use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FileKey {
    pub key_name: String,
    pub value: types::Types,
    pub key_name_location: defs::Cursor,
    pub value_location: defs::Cursor,
    pub pos: defs::Cursor,
}