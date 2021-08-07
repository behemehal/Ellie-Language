use crate::syntax::types;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayEntry {
    pub value_complete: bool,
    pub value: Box<types::Types>,
    pub location: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayType {
    pub layer_size: usize,
    pub collective: Vec<ArrayEntry>,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayTypeCollector {
    pub data: ArrayType,
    pub complete: bool,
    pub comma: bool,
    pub child_start: bool,
}
