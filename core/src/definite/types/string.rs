use crate::defs;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct StringType {
    pub value: String,
    pub comma_start_pos: defs::Cursor,
    pub comma_end_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
}
