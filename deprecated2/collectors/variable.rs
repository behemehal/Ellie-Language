use crate::collectors;
use crate::alloc::string::String;

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct PositionOfElement {
    pub colmn: usize,
    pub line: usize
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Variable {
    pub type_of: String,
    pub name: String,
    pub value: collectors::value_collector::ValueTypes,
    pub pos: PositionOfElement
}