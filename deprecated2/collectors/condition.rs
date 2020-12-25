use crate::alloc::string::{String, ToString};
use crate::alloc::vec::Vec;

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct PositionOfElement {
    pub colmn: usize,
    pub line: usize
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct ConditionChain {
    pub type_of_condition: String,
    pub param: String,
    pub inner_code: String,
    pub pos: PositionOfElement
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Condition {
    pub chain: Vec<ConditionChain>,
}