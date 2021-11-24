use crate::processors::types::TypeProcessor;
use ellie_core::definite;

#[derive(Default, Clone, Debug)]
pub struct CloakType {
    pub entries: Vec<DefinerTypes>,
    pub at_comma: bool,
    pub child_cache: Box<DefinerCollector>,
    pub not_empty: bool,
}

#[derive(Default, Clone, Debug)]
pub struct ArrayType {
    pub rtype: Box<DefinerTypes>,
    pub size: Box<definite::types::Types>,
    pub size_collected: bool,
    pub raw_size: String,
    pub at_comma: bool,
    pub type_collected: bool,
    pub child_cache: Box<DefinerCollector>,
    pub size_child_cache: Box<TypeProcessor>,
}

#[derive(Default, Clone, Debug)]
pub struct CollectiveType {
    pub key: Box<DefinerTypes>,
    pub value: Box<DefinerTypes>,
    pub key_collected: bool,
    pub at_comma: bool,
    pub child_cache: Box<DefinerCollector>,
}

#[derive(Default, Clone, Debug)]
pub struct VectorType {
    pub rtype: Box<DefinerTypes>,
}

#[derive(Default, Clone, Debug)]
pub struct FutureType {
    pub rtype: Box<DefinerTypes>,
    pub child_cache: Box<DefinerCollector>,
}

#[derive(Default, Clone, Debug)]
pub struct NullableType {
    pub rtype: Box<DefinerTypes>,
    pub child_cache: Box<DefinerCollector>,
}

#[derive(Default, Clone, Debug)]
pub struct GenericType {
    pub rtype: String,
}

#[derive(Default, Clone, Debug)]
pub struct FunctionType {
    pub returning: Box<DefinerTypes>,
    pub params: Vec<DefinerTypes>,
    pub child_cache: Box<DefinerCollector>,
    pub brace_stared: bool,
    pub parameters_collected: bool,
    pub return_char_typed: bool,
    pub at_comma: bool,
    pub not_empty: bool,
}

#[derive(Clone, Debug)]
pub enum DefinerTypes {
    Cloak(CloakType),
    Array(ArrayType),
    Collective(CollectiveType),
    Vector(VectorType),
    Future(FutureType),
    Nullable(NullableType),
    Generic(GenericType),
    Function(FunctionType),
    Dynamic,
}

impl definite::Converter<DefinerTypes, definite::definers::DefinerCollecting> for DefinerTypes {
    fn to_definite(self) -> definite::definers::DefinerCollecting {
        match self {
            DefinerTypes::Cloak(e) => {
                definite::definers::DefinerCollecting::Cloak(definite::definers::CloakType {
                    rtype: e.entries.into_iter().map(|x| x.to_definite()).collect(),
                })
            }
            DefinerTypes::Array(e) => {
                definite::definers::DefinerCollecting::Array(definite::definers::ArrayType {
                    rtype: Box::new(e.rtype.to_definite()),
                    size: e.size,
                })
            }
            DefinerTypes::Collective(e) => definite::definers::DefinerCollecting::Collective(
                definite::definers::CollectiveType {
                    key: Box::new(e.key.to_definite()),
                    value: Box::new(e.value.to_definite()),
                },
            ),
            DefinerTypes::Vector(e) => {
                definite::definers::DefinerCollecting::Vector(definite::definers::VectorType {
                    rtype: Box::new(e.rtype.to_definite()),
                })
            }
            DefinerTypes::Future(e) => {
                definite::definers::DefinerCollecting::Future(definite::definers::FutureType {
                    value: Box::new(e.rtype.to_definite()),
                })
            }
            DefinerTypes::Nullable(e) => {
                definite::definers::DefinerCollecting::Nullable(definite::definers::NullableType {
                    value: Box::new(e.rtype.to_definite()),
                })
            }
            DefinerTypes::Generic(e) => {
                definite::definers::DefinerCollecting::Generic(definite::definers::GenericType {
                    rtype: e.rtype,
                    hash: String::new(),
                })
            }
            DefinerTypes::Function(e) => {
                definite::definers::DefinerCollecting::Function(definite::definers::FunctionType {
                    params: e.params.into_iter().map(|x| x.to_definite()).collect(),
                    returning: Box::new(e.returning.to_definite()),
                })
            }
            DefinerTypes::Dynamic => definite::definers::DefinerCollecting::Dynamic,
        }
    }

    fn from_definite(self, from: definite::definers::DefinerCollecting) -> DefinerTypes {
        match from {
            definite::definers::DefinerCollecting::Array(e) => DefinerTypes::Array(ArrayType {
                rtype: Box::new(DefinerTypes::default().from_definite(*e.rtype)),
                size: e.size,
                ..Default::default()
            }),
            definite::definers::DefinerCollecting::Future(e) => DefinerTypes::Future(FutureType {
                rtype: Box::new(DefinerTypes::default().from_definite(*e.value)),
                ..Default::default()
            }),
            definite::definers::DefinerCollecting::Vector(e) => DefinerTypes::Vector(VectorType {
                rtype: Box::new(DefinerTypes::default().from_definite(*e.rtype)),
            }),
            definite::definers::DefinerCollecting::Generic(e) => {
                DefinerTypes::Generic(GenericType { rtype: e.rtype })
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
                ..Default::default()
            }),
            definite::definers::DefinerCollecting::Collective(e) => {
                DefinerTypes::Collective(CollectiveType {
                    key: Box::new(DefinerTypes::default().from_definite(*e.key)),
                    value: Box::new(DefinerTypes::default().from_definite(*e.value)),
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
        }
    }
}

impl Default for DefinerTypes {
    fn default() -> Self {
        DefinerTypes::Generic(GenericType {
            rtype: String::new(),
        })
    }
}

#[derive(Default, Clone, Debug)]
pub struct DefinerCollector {
    pub definer_type: DefinerTypes,
    pub complete: bool,
}
