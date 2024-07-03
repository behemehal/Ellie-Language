use crate::{definite::types, defs};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CloakEntry {
    pub value: types::Types,
    pub location: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CloakType {
    pub collective: Vec<CloakEntry>,
    pub pos: defs::Cursor,
}
