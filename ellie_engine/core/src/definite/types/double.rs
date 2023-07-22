use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DoubleType {
    pub value: f32,
    pub pos: defs::Cursor,
}
