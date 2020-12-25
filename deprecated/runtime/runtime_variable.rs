use crate::utils;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec::Vec;
use crate::alloc::borrow::ToOwned;
use crate::collectors;
use crate::runtime::{runtime_function};


#[derive(Debug, Clone , PartialEq, Eq)]
pub struct NumberVariable {
    pub positive: bool,
    pub value: i32,
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct StringVariable {
    pub length: usize,
    pub value: String
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct FunctionVariable {
    pub name: String,
    pub dynamic: bool,
    pub type_return: String,
    pub code: crate::runtime::runtime_function::Code,
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct ArrayItem {
    pub type_identifier: String,
    pub value: ValueTypes
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct ArrayVariable {
    pub length: usize,
    pub value: Vec<ArrayItem>
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct BooleanVariable {
    pub value: bool
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub enum ValueTypes {
    TypeNumber(NumberVariable),
    TypeString(StringVariable),
    TypeArray(ArrayVariable),
    TypeBool(BooleanVariable),
    TypeFunction(FunctionVariable),
    None(bool)
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub value: ValueTypes,
    pub type_identifier: String,
    pub muteable: bool
}

pub enum CollectedVariableDataResponse {
    SyntaxError(collectors::SyntaxError),
    Collected(ValueTypes)
}
