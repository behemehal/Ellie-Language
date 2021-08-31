use crate::definite::types;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::defs;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CloakEntry {
    pub value: Box<types::Types>,
    pub location: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CloakType {
    pub layer_size: usize,
    pub collective: Vec<CloakEntry>,
}