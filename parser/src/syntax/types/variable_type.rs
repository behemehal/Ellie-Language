use alloc::string::String;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct VariableType {
    pub value: String,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct VariableTypeCollector {
    pub data: VariableType,
    pub value_complete: bool,
}
