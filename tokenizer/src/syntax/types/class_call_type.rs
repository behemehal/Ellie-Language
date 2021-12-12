use crate::{
    processors::types::{self, Processors},
    syntax::items::definers::{DefinerCollector, DefinerTypes},
};
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClassCallParameter {
    pub value: types::Processors,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClassCallGenericParameter {
    pub value: DefinerTypes,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClassCall {
    pub target: Box<types::Processors>,
    pub target_pos: defs::Cursor,
    pub keyword_pos: defs::Cursor,
    pub generic_parameters: Vec<ClassCallGenericParameter>,
    pub parameters: Vec<ClassCallParameter>,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClassCallCollector {
    pub data: ClassCall,
    pub base_collected: bool,
    pub generic_collected: bool,
    pub param_collected: bool,
    pub complete: bool,
    pub generic_cache: DefinerCollector,
    pub itered_cache: Box<types::TypeProcessor>,
}

impl definite::Converter<ClassCallCollector, definite::types::class_call::ClassCall>
    for ClassCallCollector
{
    fn to_definite(self) -> definite::types::class_call::ClassCall {
        definite::types::class_call::ClassCall {
            target: Box::new(self.data.target.to_definite()),
            keyword_pos: self.data.keyword_pos,
            target_pos: self.data.target_pos,
            params: self
                .data
                .parameters
                .into_iter()
                .map(|x| definite::types::class_call::ClassCallParameter {
                    value: x.value.to_definite(),
                    pos: x.pos,
                })
                .collect(),
            generic_parameters: self
                .data
                .generic_parameters
                .into_iter()
                .map(|x| definite::types::class_call::ClassCallGenericParameter {
                    value: x.value.to_definite(),
                    pos: x.pos,
                })
                .collect(),
            pos: self.data.pos,
        }
    }

    fn from_definite(self, from: definite::types::class_call::ClassCall) -> ClassCallCollector {
        ClassCallCollector {
            data: ClassCall {
                target: Box::new(types::Processors::default().from_definite(*from.target)),
                target_pos: from.target_pos,
                generic_parameters: from
                    .generic_parameters
                    .into_iter()
                    .map(|x| ClassCallGenericParameter {
                        value: DefinerTypes::default().from_definite(x.value),
                        pos: x.pos,
                    })
                    .collect(),

                parameters: from
                    .params
                    .into_iter()
                    .map(|x| ClassCallParameter {
                        value: Processors::default().from_definite(x.value),
                        pos: x.pos,
                    })
                    .collect(),
                keyword_pos: from.keyword_pos,
                pos: from.pos,
            },
            ..Default::default()
        }
    }
}
