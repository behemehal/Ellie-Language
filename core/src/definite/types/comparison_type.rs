use crate::definite::types;
use serde::{Deserialize, Serialize};
use alloc::boxed::Box;
use alloc::string::String;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperators {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Null,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonType {
    pub first: Box<types::Types>,
    pub second: Box<types::Types>,
    pub operator: ComparisonOperators,
}