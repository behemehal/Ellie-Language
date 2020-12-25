use crate::alloc::string::{ToString, String};
use crate::alloc::vec::Vec;


#[derive(Debug, Clone , PartialEq, Eq)]
pub struct PositionOfElement {
    pub colmn: usize,
    pub line: usize
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub type_of: String
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub type_return: String,
    pub inner_code: String,
    pub pos: PositionOfElement
}