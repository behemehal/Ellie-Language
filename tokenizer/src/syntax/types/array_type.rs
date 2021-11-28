use crate::processors::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayEntry {
    pub value: types::Processors,
    pub location: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayType {
    pub collective: Vec<ArrayEntry>,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayTypeCollector {
    pub data: ArrayType,
    pub complete: bool,
    pub brace_started: bool,
    pub itered_cache: Box<types::TypeProcessor>,
}

impl definite::Converter<ArrayTypeCollector, definite::types::array::ArrayType>
    for ArrayTypeCollector
{
    fn to_definite(self) -> definite::types::array::ArrayType {
        definite::types::array::ArrayType {
            collective: self
                .data
                .collective
                .into_iter()
                .map(|x| definite::types::array::ArrayEntry {
                    value: x.value.to_definite(),
                    location: x.location,
                })
                .collect(),
            pos: self.data.pos,
        }
    }

    fn from_definite(self, from: definite::types::array::ArrayType) -> ArrayTypeCollector {
        ArrayTypeCollector {
            data: ArrayType {
                collective: from
                    .collective
                    .into_iter()
                    .map(|x| ArrayEntry {
                        value: types::Processors::default().from_definite(x.value),
                        location: x.location,
                    })
                    .collect(),
                pos: from.pos,
            },
            ..Default::default()
        }
    }
}
