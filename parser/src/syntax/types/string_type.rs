use alloc::string::String;
use serde::{Deserialize, Serialize};
use ellie_core::defs;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct StringType {
    pub value: String,
    pub comma_start_pos: defs::Cursor,
    pub comma_end_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
}


#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct StringTypeCollector {
    pub data: StringType,
    pub complete: bool,
}