use crate::definite::types;
use crate::definite::items::variable;
use serde::{Deserialize, Serialize};

use crate::definite::types::arithmetic_type::ArithmeticOperators;
use crate::definite::types::comparison_type::ComparisonOperators;
use crate::definite::types::logical_type::LogicalOperators;

use alloc::boxed::Box;
use alloc::string::String;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Operators {
    ComparisonType(ComparisonOperators),
    LogicalType(LogicalOperators),
    ArithmeticType(ArithmeticOperators),
    Null,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct OperatorType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub second: Box<types::Types>,
    pub operator: Operators,
}