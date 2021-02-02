use crate::alloc::string::{String, ToString};
use crate::alloc::vec::Vec;
use crate::collectors;

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct PositionOfElement {
    pub colmn: usize,
    pub line: usize
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct VariableDefinier {
    pub target: collectors::value_collector::ValueTypes,
    pub value: collectors::value_collector::ValueTypes,
    pub pos: PositionOfElement
}