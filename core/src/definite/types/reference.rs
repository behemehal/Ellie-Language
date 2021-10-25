use crate::definite::types;
use crate::defs;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::string::String;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub pos: defs::Cursor,
    pub value: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceType {
    pub reference: Box<types::Types>,
    pub reference_pos: defs::Cursor,
    pub chain: Vec<Chain>,
}
