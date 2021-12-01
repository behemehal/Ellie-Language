use super::Collecting;
use crate::definite::types;
use crate::defs;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ForLoop {
    pub variable: types::Types,
    pub iterator: types::Types,
    pub parameter: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub body: Vec<Collecting>,
    pub pos: defs::Cursor,
}
