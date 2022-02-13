use crate::utils::generate_hash;
use crate::{definite::types::Types, defs};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, boxed::Box};
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
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
            DefinerCollecting::Function(_) => "function".to_owned(),
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
            },
            DefinerCollecting::ParentGeneric(parent_generic) => {
                if let DefinerCollecting::ParentGeneric(other_parent_generic) = other {
                    other_parent_generic.rtype == parent_generic.rtype && other_parent_generic.hash == parent_generic.hash
                        && other_parent_generic.generics.len() == parent_generic.generics.len()
                        && other_parent_generic
                            .generics
                            .iter()
                            .zip(parent_generic.generics.iter())
                            .all(|(a, b)| a.value.same_as(b.value.clone()))
                } else {
                    false
                }
            },
            DefinerCollecting::Function(_) => todo!(),
            DefinerCollecting::Cloak(_) => todo!(),
            DefinerCollecting::Collective(_) => todo!(),
            DefinerCollecting::Nullable(_) => todo!(),
            DefinerCollecting::Dynamic => todo!(),
        }
    }

    
}
