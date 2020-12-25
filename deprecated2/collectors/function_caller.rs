use crate::alloc::string::{String, ToString};
use crate::alloc::vec::Vec;

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct PositionOfElement {
    pub colmn: usize,
    pub line: usize
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub value: String,
    pub value_type: String
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct FunctionCaller {
    pub name: String,
    pub paramaters: Vec<Parameter>,
    pub pos: PositionOfElement
}