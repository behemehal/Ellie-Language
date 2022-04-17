use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DoubleType {
    pub value: f64,
    pub pos: defs::Cursor,
}
