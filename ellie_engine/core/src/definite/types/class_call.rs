use crate::definite::definers::DefinerCollecting;
use crate::definite::{definers, types};
use crate::defs;
use alloc::boxed::Box;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ClassCallParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ClassCallGenericParameter {
    pub value: definers::DefinerCollecting,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ClassCall {
    pub target: Box<types::Types>,
    pub keyword_pos: defs::Cursor,
    pub target_pos: defs::Cursor,
    pub resolved_generics: Vec<DefinerCollecting>,
    pub generic_parameters: Vec<ClassCallGenericParameter>,
    pub params: Vec<ClassCallParameter>,
    pub pos: defs::Cursor,
}
