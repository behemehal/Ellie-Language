use crate::syntax::types;
use alloc::string::String;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileKey {
    pub key_name: String,
    pub value: types::Types,
    pub key_name_location: defs::Cursor,
    pub value_location: defs::Cursor,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileKeyCollector {
    pub data: FileKey,
    pub key_name_collected: bool,
    pub value_collected: bool,
}
