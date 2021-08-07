use crate::syntax::types;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedClassParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedClass {
    pub value: Box<types::Types>,
    pub keyword_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub params: Vec<ConstructedClassParameter>,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedClassCollector {
    pub data: ConstructedClass,
    pub keyword_collected: bool,
    pub keyword_index: i8,
    pub raw_value: String,
    pub value_collected: bool,
    pub comma: bool,
    pub complete: bool,
}
