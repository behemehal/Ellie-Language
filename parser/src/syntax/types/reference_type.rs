use crate::syntax::{definers, types};
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub pos: defs::Cursor,
    pub value: String,
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
    pub last_entry: definers::DefinerCollecting,
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
                    value: x.value,
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn from_definite(self, from: definite::types::reference::ReferenceType) -> Self {
        ReferenceTypeCollector {
            data: ReferenceType {
                reference: Box::new(types::Types::default().from_definite(*from.reference)),
                reference_pos: from.reference_pos,
                chain: from
                    .chain
                    .into_iter()
                    .map(|x| Chain {
                        pos: x.pos,
                        value: x.value,
                    })
                    .collect::<Vec<_>>(),
            },
            complete: true,
            ..Default::default()
        }
    }
}
