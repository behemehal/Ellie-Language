use crate::syntax::types;
use serde::Deserialize;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::vec::Vec;
use ellie_core::defs;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CloakEntry {
    pub value_complete: bool,
    pub value: Box<types::Types>,
    pub location: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CloakType {
    pub layer_size: usize,
    pub collective: Vec<CloakEntry>,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CloakTypeCollector {
    pub data: CloakType,
    pub complete: bool,
    pub comma: bool,
    pub child_start: bool,
}
