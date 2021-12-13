use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructorParameter {
    pub name: String,
    pub name_pos: defs::Cursor,
    pub pos: defs::Cursor,
}
