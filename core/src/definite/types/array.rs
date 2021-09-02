use crate::definite::types;
use crate::defs;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayEntry {
    pub value: Box<types::Types>,
    pub location: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayType {
    pub layer_size: usize,
    pub collective: Vec<ArrayEntry>,
}
