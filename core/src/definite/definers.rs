use crate::{definite::types::Types, defs};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
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
    pub hash: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GenericType {
    pub rtype: String,
    pub pos: defs::Cursor,
    pub hash: String,
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
