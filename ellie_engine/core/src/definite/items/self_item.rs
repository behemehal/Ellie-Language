use serde::{Deserialize, Serialize};

use crate::defs::Cursor;

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SelfItem {
    pub class_page: usize,
    pub class_hash: usize,
    pub pos: Cursor,
}
