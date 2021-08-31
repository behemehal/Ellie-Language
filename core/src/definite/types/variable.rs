use crate::defs;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct VariableType {
    pub value: String,
    pub pos: defs::Cursor,
}
