use crate::syntax::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub pos: defs::Cursor,
    pub value: types::Types,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceType {
    pub reference: Box<types::Types>,
    pub reference_pos: defs::Cursor,
    pub chain: Vec<Chain>,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceTypeCollector {
    pub data: ReferenceType,
    pub on_dot: bool,
    pub root_available: bool,
    pub complete: bool,
}

impl ReferenceTypeCollector {
    pub fn to_definite(self) -> definite::types::reference::ReferenceType {
        definite::types::reference::ReferenceType {
            reference: Box::new(self.data.reference.to_definite()),
            reference_pos: self.data.reference_pos,
            chain: self
                .data
                .chain
                .into_iter()
                .map(|x| definite::types::reference::Chain {
                    pos: x.pos,
                    value: x.value.to_definite(),
                })
                .collect(),
        }
    }
}
