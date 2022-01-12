use crate::defs;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct VariableType {
    pub value: String,
    pub reference: u64,
    pub pos: defs::Cursor,
}
