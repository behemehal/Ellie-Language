use crate::{definite::types, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Loop {
    pub condition: types::Types,
    pub body_pos: defs::Cursor,
    pub inner_page_id: usize,
    pub hash: usize,
    pub pos: defs::Cursor,
}
