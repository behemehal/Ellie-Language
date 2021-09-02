use alloc::string::String;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

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

impl StringTypeCollector {
    pub fn to_definite(self) -> definite::types::string::StringType {
        definite::types::string::StringType {
            value: self.data.value,
            comma_start_pos: self.data.comma_start_pos,
            comma_end_pos: self.data.comma_end_pos,
            value_pos: self.data.value_pos,
        }
    }
}
