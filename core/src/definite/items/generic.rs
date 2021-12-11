use alloc::string::String;
use serde::{Deserialize, Serialize};
use crate::defs;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Generic {
    pub name: String,
    pub pos: defs::Cursor,
}
