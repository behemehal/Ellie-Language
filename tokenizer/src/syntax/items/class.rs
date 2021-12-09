use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

use crate::processors::items::Processors;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GenericDefining {
    pub name: String,
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
    pub iterator: Box<crate::iterator::Iterator>,
    pub body: Vec<Processors>,
    pub brace_count: usize,
    pub pos: defs::Cursor,
    pub hash: String,
    pub complete: bool,
}

impl Converter<Class, ellie_core::definite::items::class::Class> for Class {
    fn to_definite(self) -> ellie_core::definite::items::class::Class {
        ellie_core::definite::items::class::Class {
            name: self.name,
            public: self.public,
            constructor: self
                .body
                .clone()
                .into_iter()
                .find_map(|f| match f {
                    Processors::Constructor(c) => Some(c),
                    _ => None,
                })
                .unwrap_or(crate::syntax::items::constructor::Constructor::default())
                .to_definite(),
            generic_definings: self
                .generic_definings
                .into_iter()
                .map(|x| ellie_core::definite::items::class::GenericDefining {
                    name: x.name,
                    pos: x.pos,
                })
                .collect(),
            properties: self
                .body
                .clone()
                .into_iter()
                .filter_map(|f| match f {
                    Processors::Variable(x) => Some(x.to_definite()),
                    _ => None,
                })
                .collect(),
            methods: self
                .body
                .clone()
                .into_iter()
                .filter_map(|f| match f {
                    Processors::Function(x) => Some(x.to_definite()),
                    _ => None,
                })
                .collect(),
            name_pos: self.name_pos,
            pos: self.pos,
            hash: self.hash,
            getters: vec![],
            setters: vec![],
        }
    }

    fn from_definite(self, _from: ellie_core::definite::items::class::Class) -> Class {
        todo!()
    }
}
