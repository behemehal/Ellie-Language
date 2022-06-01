use alloc::string::String;
use crate::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructorParameter {
    pub name: String,
    pub pos: defs::Cursor,
}
