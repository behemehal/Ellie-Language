use crate::definite::types;
use crate::defs;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use super::class_instance::AttributeType;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct IndexChainAttribute {
    pub rtype: AttributeType,
    pub hash: usize,
    pub page_hash: usize,
    pub idx: usize,
}

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
    pub index_chain: Vec<IndexChainAttribute>,
    pub pos: defs::Cursor,
}
