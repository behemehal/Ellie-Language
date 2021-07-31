use alloc::string::String;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct VariableType {
    pub value_complete: bool,
    pub value: String,
    pub pos: defs::Cursor,
}
