use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ByteType {
    pub value: i8,
    pub pos: defs::Cursor,
}
