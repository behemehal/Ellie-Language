use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

use crate::processors::items::Processors;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GenericDefining {
    pub name: String,
    pub hash: usize,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub public: bool,
    pub generics_collected: bool,
    pub name_collected: bool,
    pub generic_definings: Vec<GenericDefining>,
    pub continuum_collected: bool,
    pub name_pos: defs::Cursor,
    #[serde(skip)]
    pub iterator: Box<crate::iterator::Iterator>,
    pub body: Vec<Processors>,
    pub brace_count: usize,
    pub pos: defs::Cursor,
    pub hash: usize,
    pub complete: bool,
}

impl Converter<Class, ellie_core::definite::items::class::Class> for Class {
    fn to_definite(self) -> ellie_core::definite::items::class::Class {
        panic!("Not required")
    }

    fn from_definite(self, from: ellie_core::definite::items::class::Class) -> Class {
        Class {
            name: from.name,
            public: from.public,
            name_pos: from.name_pos,
            body: vec![],
            brace_count: 0,
            hash: from.hash,
            pos: from.pos,
            generic_definings: from
                .generic_definings
                .iter()
                .map(|x| GenericDefining {
                    name: x.name.clone(),
                    hash: x.hash.clone(),
                    pos: x.pos,
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        }
    }
}
