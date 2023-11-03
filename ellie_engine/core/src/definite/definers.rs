use crate::{definite::types::Types, defs};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, boxed::Box};
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

use super::types::class_instance::ClassInstance;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayType {
    pub rtype: Box<DefinerCollecting>,
    pub has_size: bool,
    pub size: Option<Box<Types>>,
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
    pub hash: usize,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GenericType {
    pub rtype: String,
    pub pos: defs::Cursor,
    pub hash: usize,
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
pub enum EnumFieldData {
    NoData,
    Data(Box<DefinerCollecting>),
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct EnumField {
    pub field_name: String,
    pub field_data: EnumFieldData,
    pub name: String,
    pub hash: usize,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, EnumAsInner)]
pub enum DefinerCollecting {
    Array(ArrayType),
    Generic(GenericType),
    ParentGeneric(ParentGenericType),
    Function(FunctionType),
    Cloak(CloakType),
    Collective(CollectiveType),
    Nullable(NullableType),
    EnumField(EnumField),
    ClassInstance(ClassInstance),
    Dynamic,
}

impl DefinerCollecting {
    pub fn to_string(&self) -> String {
        match self {
            DefinerCollecting::Array(_) => "array".to_owned(),
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
            DefinerCollecting::EnumField(e) => match &e.field_data {
                EnumFieldData::NoData => format!("{}.{}", e.name, e.field_name),
                EnumFieldData::Data(inner_def) => {
                    //format!("{}.{}({})", e.name, e.field_name, inner_def.to_string())
                    inner_def.to_string()
                }
            },
            DefinerCollecting::ClassInstance(e) => {
                format!("Object<{}>", e.class_name)
            }
        }
    }

    pub fn same_as(&self, other: DefinerCollecting) -> bool {
        if matches!(&other, DefinerCollecting::Generic(generic) if generic.rtype == "dyn")
            || matches!(&self, DefinerCollecting::Generic(generic) if generic.rtype == "dyn")
        {
            return true;
        }
        match self {
            DefinerCollecting::Array(data) => {
                if let DefinerCollecting::Array(other_data) = other {
                    other_data.size == data.size && other_data.rtype.same_as(*data.rtype.clone())
                } else if DefinerCollecting::Dynamic == other {
                    true
                } else {
                    false
                }
            }
            DefinerCollecting::Generic(generic) => {
                if let DefinerCollecting::Generic(other_generic) = other {
                    (other_generic.rtype == generic.rtype && other_generic.hash == generic.hash)
                        || (other_generic.rtype == "dyn" || generic.rtype == "dyn")
                } else if DefinerCollecting::Dynamic == other {
                    true
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
            DefinerCollecting::EnumField(e) => {
                if let DefinerCollecting::EnumField(other_e) = other {
                    e.name == other_e.name && e.field_name == other_e.field_name
                } else {
                    false
                }
            }
            DefinerCollecting::ClassInstance(_) => todo!(),
            DefinerCollecting::Dynamic => unreachable!(),
        }
    }

    pub fn convert_generic(&mut self, generic_hash: usize, replacement_generic: DefinerCollecting) {
        match self {
            DefinerCollecting::Array(array) => {
                array
                    .rtype
                    .convert_generic(generic_hash, replacement_generic);
            }
            DefinerCollecting::Generic(generic) => {
                if generic.hash == generic_hash {
                    *self = replacement_generic;
                }
            }
            DefinerCollecting::ParentGeneric(parrent_generic) => {
                if parrent_generic.hash == generic_hash {
                    *self = replacement_generic;
                } else {
                    parrent_generic.generics.iter_mut().for_each(|g| {
                        g.value
                            .convert_generic(generic_hash, replacement_generic.clone())
                    });
                }
            }
            DefinerCollecting::Function(function) => {
                function
                    .params
                    .iter_mut()
                    .for_each(|param: &mut DefinerCollecting| {
                        param.convert_generic(generic_hash, replacement_generic.clone())
                    });
                function
                    .returning
                    .convert_generic(generic_hash, replacement_generic.clone());
            }
            DefinerCollecting::Cloak(cloak) => {
                cloak
                    .rtype
                    .iter_mut()
                    .for_each(|r| r.convert_generic(generic_hash, replacement_generic.clone()));
            }
            DefinerCollecting::Collective(collective) => {
                collective
                    .key
                    .convert_generic(generic_hash, replacement_generic.clone());
                collective
                    .value
                    .convert_generic(generic_hash, replacement_generic);
            }
            DefinerCollecting::Nullable(nullable) => {
                nullable
                    .value
                    .convert_generic(generic_hash, replacement_generic);
            }
            DefinerCollecting::EnumField(enum_field) => match &mut enum_field.field_data {
                EnumFieldData::NoData => (),
                EnumFieldData::Data(enum_field_data) => {
                    enum_field_data.convert_generic(generic_hash, replacement_generic)
                }
            },
            DefinerCollecting::Dynamic => (),
            DefinerCollecting::ClassInstance(_) => (),
        }
    }
}
