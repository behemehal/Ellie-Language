use serde::{Deserialize, Serialize};

use crate::defs;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct IntegerType {
    pub value: isize,
    pub pos: defs::Cursor,
}
