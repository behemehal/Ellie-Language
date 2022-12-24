use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FloatType {
    pub value: f32,
    pub pos: defs::Cursor,
}
