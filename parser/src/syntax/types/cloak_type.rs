use crate::syntax::types;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Hash)]
pub struct CloakEntry {
    pub value_complete: bool,
    pub value: Box<types::Types>,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Hash)]
pub struct CloakType {
    pub layer_size: usize,
    pub complete: bool,
    pub comma: bool,
    pub child_start: bool,
    pub collective: Vec<CloakEntry>,
}
