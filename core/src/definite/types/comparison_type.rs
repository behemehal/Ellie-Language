use crate::definite::types;
use alloc::boxed::Box;
use alloc::string::String;
use serde::{Deserialize, Serialize};

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
