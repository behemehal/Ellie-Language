use crate::definite::types;
use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Ret {
    pub value: types::Types,
    pub keyword_pos: defs::Cursor,
    pub value_position: defs::Cursor,
    pub pos: defs::Cursor,
}
