use crate::definite::types;
use crate::defs;
use serde::{Deserialize, Serialize};

use crate::definite::types::arithmetic_type::ArithmeticOperators;
use crate::definite::types::assignment_type::AssignmentOperators;
use crate::definite::types::comparison_type::ComparisonOperators;
use crate::definite::types::logical_type::LogicalOperators;

use alloc::boxed::Box;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Operators {
    ComparisonType(ComparisonOperators),
    LogicalType(LogicalOperators),
    ArithmeticType(ArithmeticOperators),
    AssignmentType(AssignmentOperators),
    Null,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct OperatorType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_pos: defs::Cursor,
    pub second_pos: defs::Cursor,
    pub second: Box<types::Types>,
    pub operator: Operators,
    pub pos: defs::Cursor,
}
