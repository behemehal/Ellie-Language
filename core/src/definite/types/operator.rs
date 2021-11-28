use crate::definite::types;
use crate::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum ArithmeticOperators {
    Addition,
    Subtraction,
    Multiplication,
    Exponentiation,
    Division,
    Modulus,
    Null,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum AssignmentOperators {
    Assignment,
    AdditionAssignment,
    SubtractionAssignment,
    MultiplicationAssignment,
    DivisionAssignment,
    ModulusAssignment,
    ExponentiationAssignment,
    Null,
}


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
pub enum LogicalOperators {
    And,
    Or,
    Null,
}

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
