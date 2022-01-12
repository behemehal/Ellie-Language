use core::panic;

use crate::processors::types::TypeProcessor;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloakType {
    pub entries: Vec<DefinerTypes>,
    pub at_comma: bool,
    pub child_cache: Box<DefinerCollector>,
    pub not_empty: bool,
    pub pos: defs::Cursor,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArrayType {
    pub rtype: Box<DefinerTypes>,
    pub size: Box<definite::types::Types>,
    pub size_collected: bool,
    pub raw_size: String,
    pub at_comma: bool,
    pub type_collected: bool,
    pub child_cache: Box<DefinerCollector>,
    pub size_child_cache: Box<TypeProcessor>,
    pub pos: defs::Cursor,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CollectiveType {
    pub key: Box<DefinerTypes>,
    pub value: Box<DefinerTypes>,
    pub key_collected: bool,
    pub at_comma: bool,
    pub child_cache: Box<DefinerCollector>,
    pub pos: defs::Cursor,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VectorType {
    pub rtype: Box<DefinerTypes>,
    pub pos: defs::Cursor,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FutureType {
    pub rtype: Box<DefinerTypes>,
    pub child_cache: Box<DefinerCollector>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NullableType {
    pub rtype: Box<DefinerTypes>,
    pub pos: defs::Cursor,
    pub child_cache: Box<DefinerCollector>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GenericParameter {
    pub value: DefinerTypes,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ParentGenericType {
    pub parent: String,
    pub parent_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub generics: Vec<GenericParameter>,
    pub cache: Box<DefinerCollector>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenericType {
    pub pos: defs::Cursor,
    pub rtype: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FunctionType {
    pub returning: Box<DefinerTypes>,
    pub params: Vec<DefinerTypes>,
    pub child_cache: Box<DefinerCollector>,
    pub brace_stared: bool,
    pub parameters_collected: bool,
    pub return_char_typed: bool,
    pub at_comma: bool,
    pub not_empty: bool,
    pub pos: defs::Cursor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DefinerTypes {
    Cloak(CloakType),
    Array(ArrayType),
    Collective(CollectiveType),
    Vector(VectorType),
    Nullable(NullableType),
    ParentGeneric(ParentGenericType),
    Generic(GenericType),
    Function(FunctionType),
    Dynamic,
}

impl DefinerTypes {
    pub fn get_pos(&self) -> defs::Cursor {
        match self {
            DefinerTypes::Cloak(e) => e.pos,
            DefinerTypes::Array(e) => e.pos,
            DefinerTypes::Collective(e) => e.pos,
            DefinerTypes::Vector(e) => e.pos,
            DefinerTypes::Nullable(e) => e.pos,
            DefinerTypes::ParentGeneric(e) => e.pos,
            DefinerTypes::Generic(e) => e.pos,
            DefinerTypes::Function(e) => e.pos,
            DefinerTypes::Dynamic => panic!("Unexpected behaviour"),
        }
    }
}

impl Default for DefinerTypes {
    fn default() -> Self {
        DefinerTypes::Generic(GenericType::default())
    }
}

impl definite::Converter<DefinerTypes, definite::definers::DefinerCollecting> for DefinerTypes {
    fn to_definite(self) -> definite::definers::DefinerCollecting {
        match self {
            DefinerTypes::Cloak(e) => {
                definite::definers::DefinerCollecting::Cloak(definite::definers::CloakType {
                    rtype: e.entries.into_iter().map(|x| x.to_definite()).collect(),
                    pos: e.pos,
                })
            }
            DefinerTypes::Array(e) => {
                definite::definers::DefinerCollecting::Array(definite::definers::ArrayType {
                    rtype: Box::new(e.rtype.to_definite()),
                    size: e.size,
                    pos: e.pos,
                })
            }
            DefinerTypes::Collective(e) => definite::definers::DefinerCollecting::Collective(
                definite::definers::CollectiveType {
                    key: Box::new(e.key.to_definite()),
                    value: Box::new(e.value.to_definite()),
                    pos: e.pos,
                },
            ),
            DefinerTypes::Vector(e) => {
                definite::definers::DefinerCollecting::Vector(definite::definers::VectorType {
                    rtype: Box::new(e.rtype.to_definite()),
                    pos: e.pos,
                })
            }
            DefinerTypes::Nullable(e) => {
                definite::definers::DefinerCollecting::Nullable(definite::definers::NullableType {
                    value: Box::new(e.rtype.to_definite()),
                    pos: e.pos,
                })
            }
            DefinerTypes::Generic(e) => {
                definite::definers::DefinerCollecting::Generic(definite::definers::GenericType {
                    rtype: e.rtype,
                    hash: 0,
                    pos: e.pos,
                })
            }
            DefinerTypes::Function(e) => {
                definite::definers::DefinerCollecting::Function(definite::definers::FunctionType {
                    params: e.params.into_iter().map(|x| x.to_definite()).collect(),
                    returning: Box::new(e.returning.to_definite()),
                })
            }
            DefinerTypes::Dynamic => definite::definers::DefinerCollecting::Dynamic,
            DefinerTypes::ParentGeneric(e) => definite::definers::DefinerCollecting::ParentGeneric(
                definite::definers::ParentGenericType {
                    hash: 0,
                    rtype: e.parent,
                    generics: e
                        .generics
                        .into_iter()
                        .map(|x| definite::definers::GenericParameter {
                            value: x.value.to_definite(),
                            pos: x.pos,
                        })
                        .collect(),
                    parent_pos: e.parent_pos,
                },
            ),
        }
    }

    fn from_definite(self, from: definite::definers::DefinerCollecting) -> DefinerTypes {
        match from {
            definite::definers::DefinerCollecting::Array(e) => DefinerTypes::Array(ArrayType {
                rtype: Box::new(DefinerTypes::default().from_definite(*e.rtype)),
                size: e.size,
                pos: e.pos,
                ..Default::default()
            }),
            definite::definers::DefinerCollecting::Vector(e) => DefinerTypes::Vector(VectorType {
                rtype: Box::new(DefinerTypes::default().from_definite(*e.rtype)),
                pos: e.pos,
            }),
            definite::definers::DefinerCollecting::Generic(e) => {
                DefinerTypes::Generic(GenericType {
                    rtype: e.rtype,
                    pos: e.pos,
                })
            }
            definite::definers::DefinerCollecting::Function(e) => {
                DefinerTypes::Function(FunctionType {
                    returning: Box::new(DefinerTypes::default().from_definite(*e.returning)),
                    params: e
                        .params
                        .into_iter()
                        .map(|x| DefinerTypes::default().from_definite(x))
                        .collect(),
                    ..Default::default()
                })
            }
            definite::definers::DefinerCollecting::Cloak(e) => DefinerTypes::Cloak(CloakType {
                entries: e
                    .rtype
                    .into_iter()
                    .map(|x| DefinerTypes::default().from_definite(x))
                    .collect(),
                pos: e.pos,

                ..Default::default()
            }),
            definite::definers::DefinerCollecting::Collective(e) => {
                DefinerTypes::Collective(CollectiveType {
                    key: Box::new(DefinerTypes::default().from_definite(*e.key)),
                    value: Box::new(DefinerTypes::default().from_definite(*e.value)),
                    pos: e.pos,
                    ..Default::default()
                })
            }
            definite::definers::DefinerCollecting::Nullable(e) => {
                DefinerTypes::Nullable(NullableType {
                    rtype: Box::new(DefinerTypes::default().from_definite(*e.value)),
                    ..Default::default()
                })
            }
            definite::definers::DefinerCollecting::Dynamic => DefinerTypes::Dynamic,
            definite::definers::DefinerCollecting::ParentGeneric(e) => {
                DefinerTypes::ParentGeneric(ParentGenericType {
                    parent: e.rtype,
                    generics: e
                        .generics
                        .into_iter()
                        .map(|x| GenericParameter {
                            value: DefinerTypes::default().from_definite(x.value),
                            pos: x.pos,
                        })
                        .collect(),
                    ..Default::default()
                })
            }
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DefinerCollector {
    pub definer_type: DefinerTypes,
    pub complete: bool,
}
