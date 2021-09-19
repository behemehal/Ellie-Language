use crate::syntax::types;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::vec::Vec;
use ellie_core::{definite, defs};

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

impl CloakType {
    pub fn to_definite(self) -> definite::types::cloak::CloakType {
        definite::types::cloak::CloakType {
            layer_size: self.layer_size,
            collective: self
                .collective
                .into_iter()
                .map(|x| definite::types::cloak::CloakEntry {
                    value: Box::new(x.value.to_definite()),
                    location: x.location,
                })
                .collect(),
        }
    }
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CloakTypeCollector {
    pub data: CloakType,
    pub complete: bool,
    pub comma: bool,
    pub child_start: bool,
}

impl CloakTypeCollector {
    pub fn to_definite(self) -> definite::types::cloak::CloakType {
        definite::types::cloak::CloakType {
            layer_size: self.data.layer_size,
            collective: self
                .data
                .collective
                .into_iter()
                .map(|x| definite::types::cloak::CloakEntry {
                    value: Box::new(x.value.to_definite()),
                    location: x.location,
                })
                .collect(),
        }
    }

    pub fn from_definite(self, from: definite::types::cloak::CloakType) -> Self {
        CloakTypeCollector {
            data: CloakType {
                layer_size: from.layer_size,
                collective: from
                    .collective
                    .into_iter()
                    .map(|x| CloakEntry {
                        value_complete: true,
                        value: Box::new(types::Types::default().from_definite(*x.value)),
                        location: x.location,
                    })
                    .collect(),
            },
            complete: true,
            comma: false,
            child_start: false,
        }
    }
}
