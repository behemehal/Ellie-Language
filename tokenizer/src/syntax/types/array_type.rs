use crate::processors::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayEntry {
    pub value_complete: bool,
    pub value: Box<types::Processors>,
    pub location: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayType {
    pub layer_size: usize,
    pub collective: Vec<ArrayEntry>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayTypeCollector {
    pub data: ArrayType,
    pub complete: bool,
    pub comma: bool,
    pub child_start: bool,
    pub cache: Box<types::TypeProcessor>,
}

impl ArrayTypeCollector {
    pub fn to_definite(self) -> definite::types::array::ArrayType {
        definite::types::array::ArrayType {
            layer_size: self.data.layer_size,
            collective: self
                .data
                .collective
                .into_iter()
                .map(|x| definite::types::array::ArrayEntry {
                    value: Box::new((*x.value).to_definite()),
                    location: x.location,
                })
                .collect(),
        }
    }

    pub fn from_definite(self, from: definite::types::array::ArrayType) -> ArrayTypeCollector {
        ArrayTypeCollector {
            data: ArrayType {
                layer_size: from.layer_size,
                collective: from
                    .collective
                    .into_iter()
                    .map(|x| ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Processors::default().from_definite(*x.value)),
                        location: x.location,
                    })
                    .collect(),
            },
            complete: true,
            comma: false,
            child_start: false,
            cache: Box::new(types::TypeProcessor::default()),
        }
    }
}
