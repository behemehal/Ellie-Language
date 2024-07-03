use ellie_core::defs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericItem {
    pub generic_name: String,
    pub pos: defs::Cursor,
    pub hash: usize,
}
