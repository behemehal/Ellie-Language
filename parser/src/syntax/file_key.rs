use crate::syntax::types;
use alloc::string::String;
use ellie_core::defs;
use serde::Serialize;

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
