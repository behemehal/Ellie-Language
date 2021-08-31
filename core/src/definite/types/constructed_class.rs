use crate::definite::types;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedClassParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructedClass {
    pub value: Box<types::Types>,
    pub keyword_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub params: Vec<ConstructedClassParameter>,
}