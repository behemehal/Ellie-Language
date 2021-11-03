use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArrayType {
    pub rtype: Box<DefinerCollecting>,
    pub len: crate::definite::types::integer::IntegerType,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FutureType {
    pub value: Box<DefinerCollecting>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GrowableArrayType {
    pub rtype: Box<DefinerCollecting>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GenericType {
    pub rtype: String,
    pub hash: String
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FunctionType {
    pub params: Vec<DefinerCollecting>,
    pub returning: Box<DefinerCollecting>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CloakType {
    pub rtype: Vec<DefinerCollecting>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveType {
    pub key: Box<DefinerCollecting>,
    pub value: Box<DefinerCollecting>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NullableType {
    pub value: Box<DefinerCollecting>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum DefinerCollecting {
    Array(ArrayType),
    Future(FutureType),
    GrowableArray(GrowableArrayType),
    Generic(GenericType),
    Function(FunctionType),
    Cloak(CloakType),
    Collective(CollectiveType),
    Nullable(NullableType),
    Dynamic,
}
