use ellie_core::defs;
use alloc::string::String;
use serde::Serialize;
use crate::syntax::types;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct FileKey {
    pub keyname: String,
    pub value: types::Types,
    pub keyname_location: defs::Cursor,
    pub value_location: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct FileKeyCollector {
    pub data: FileKey,
    pub keyname_collected: bool,
    pub value_collected: bool,
}