use crate::syntax::types;
use ellie_core::defs;
use serde::{Serialize, Deserialize};

use alloc::string::String;
use alloc::vec::Vec;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClassCallParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClassCall {
    pub name: String,
    pub name_pos: defs::Cursor,
    pub params: Vec<ClassCallParameter>,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClassCallCollector {
    pub data: ClassCall,
    pub keyword_collected: bool,
    pub keyword_index: i8,
    pub name_collected: bool,
    pub ignore_space: bool,
    pub comma: bool,
    pub complete: bool,
}
