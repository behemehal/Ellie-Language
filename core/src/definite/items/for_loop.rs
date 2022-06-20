use crate::definite::types;
use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ForLoop {
    pub variable: types::Types,
    pub iterator: types::Types,
    pub parameter: defs::Cursor,
    pub variable_pos: defs::Cursor,
    pub iterator_pos: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub inner_page_id: usize,
    pub pos: defs::Cursor,
}
