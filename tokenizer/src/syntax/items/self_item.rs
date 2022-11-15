use ellie_core::defs::Cursor;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfItem {
    pub class_page: usize,
    pub class_hash: usize,
    pub pos: Cursor,
}
