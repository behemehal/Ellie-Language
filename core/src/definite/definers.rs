use crate::{definite::types::Types, defs};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, boxed::Box};
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayType {
    pub rtype: Box<DefinerCollecting>,
    pub size: Box<Types>,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct VectorType {
    pub rtype: Box<DefinerCollecting>,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GenericParameter {
    pub value: DefinerCollecting,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ParentGenericType {
    pub rtype: String,
    pub parent_pos: defs::Cursor,
    pub generics: Vec<GenericParameter>,
    pub hash: u64,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GenericType {
    pub rtype: String,
    pub pos: defs::Cursor,
    pub hash: u64,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionType {
    pub params: Vec<DefinerCollecting>,
    pub returning: Box<DefinerCollecting>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CloakType {
    pub rtype: Vec<DefinerCollecting>,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveType {
    pub key: Box<DefinerCollecting>,
    pub value: Box<DefinerCollecting>,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NullableType {
    pub pos: defs::Cursor,
    pub value: Box<DefinerCollecting>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, EnumAsInner)]
pub enum DefinerCollecting {
    Array(ArrayType),
    Vector(VectorType),
    Generic(GenericType),
    ParentGeneric(ParentGenericType),
    Function(FunctionType),
    Cloak(CloakType),
    Collective(CollectiveType),
    Nullable(NullableType),
    Dynamic,
}

impl DefinerCollecting {
    pub fn to_string(&self) -> String {
        match self {
            DefinerCollecting::Array(_) => "array".to_owned(),
            DefinerCollecting::Vector(_) => "vector".to_owned(),
            DefinerCollecting::Generic(generic) => generic.rtype.to_owned(),
            DefinerCollecting::ParentGeneric(parent_generic) => format!(
                "{}<{}>",
                parent_generic.rtype,
                parent_generic
                    .generics
                    .iter()
                    .map(|g| g.value.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            DefinerCollecting::Function(function) => format!(
                "Fn({}):{}",
                function
                    .params
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                function.returning.to_string()
            ),
            DefinerCollecting::Cloak(_) => "cloak".to_owned(),
            DefinerCollecting::Collective(_) => "collective".to_owned(),
            DefinerCollecting::Nullable(_) => "nullAble".to_owned(),
            DefinerCollecting::Dynamic => "dyn".to_owned(),
        }
    }

    pub fn same_as(&self, other: DefinerCollecting) -> bool {
        match self {
            DefinerCollecting::Array(data) => {
                if let DefinerCollecting::Array(other_data) = other {
                    other_data.size == data.size && other_data.rtype.same_as(*data.rtype.clone())
                } else {
                    false
                }
            }
            DefinerCollecting::Vector(data) => {
                if let DefinerCollecting::Vector(other_data) = other {
                    other_data.rtype.same_as(*data.rtype.clone())
                } else {
                    false
                }
            }
            DefinerCollecting::Generic(generic) => {
                if let DefinerCollecting::Generic(other_generic) = other {
                    other_generic.rtype == generic.rtype && other_generic.hash == generic.hash
                } else {
                    false
                }
            }
            DefinerCollecting::ParentGeneric(parent_generic) => {
                if let DefinerCollecting::ParentGeneric(other_parent_generic) = other {
                    other_parent_generic.rtype == parent_generic.rtype
                        && other_parent_generic.hash == parent_generic.hash
                        && other_parent_generic.generics.len() == parent_generic.generics.len()
                        && other_parent_generic
                            .generics
                            .iter()
                            .zip(parent_generic.generics.iter())
                            .all(|(a, b)| a.value.same_as(b.value.clone()))
                } else {
                    false
                }
            }
            DefinerCollecting::Function(e) => {
                if let DefinerCollecting::Function(other_e) = other {
                    e.params.len() == other_e.params.len()
                        && e.params
                            .iter()
                            .zip(other_e.params.iter())
                            .all(|(a, b)| a.same_as(b.clone()))
                        && e.returning.same_as(*other_e.returning.clone())
                } else {
                    false
                }
            }
            DefinerCollecting::Cloak(cloak) => {
                if let DefinerCollecting::Cloak(other_cloak) = other {
                    cloak.rtype.len() == other_cloak.rtype.len()
                        && cloak
                            .rtype
                            .iter()
                            .zip(other_cloak.rtype.iter())
                            .all(|(a, b)| a.same_as(b.clone()))
                } else {
                    false
                }
            }
            DefinerCollecting::Collective(e) => {
                //Check if the key and value are the same
                if let DefinerCollecting::Collective(other_e) = other {
                    e.key.same_as(*other_e.key.clone()) && e.value.same_as(*other_e.value.clone())
                } else {
                    false
                }
            }
            DefinerCollecting::Nullable(e) => {
                if let DefinerCollecting::Nullable(other_e) = other {
                    e.value.same_as(*other_e.value.clone())
                } else {
                    false
                }
            }
            DefinerCollecting::Dynamic => {
                if let DefinerCollecting::Dynamic = other {
                    true
                } else {
                    false
                }
            }
        }
    }
}
