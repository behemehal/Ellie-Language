use crate::definite::types;
use serde::{Deserialize, Serialize};

use crate::defs;
use alloc::boxed::Box;
use alloc::vec::Vec;

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
