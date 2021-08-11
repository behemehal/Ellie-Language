use crate::syntax::types;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub pos: defs::Cursor,
    pub value: types::Types
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceType {
    pub reference: Box<types::Types>,
    pub chain: Vec<Chain>,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceTypeCollector {
    pub data: ReferenceType,
    pub on_dot: bool,
}
