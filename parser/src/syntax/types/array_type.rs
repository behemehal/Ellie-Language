use crate::syntax::types;
use ellie_core::defs;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct ArrayEntry {
    pub value_complete: bool,
    pub value: Box<types::Types>,
    pub location: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct ArrayType {
    pub layer_size: usize,
    pub complete: bool,
    pub comma: bool,
    pub child_start: bool,
    pub collective: Vec<ArrayEntry>,
}
