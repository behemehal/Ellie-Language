use crate::{definite::types, defs};
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

pub fn arithmetic_operator_to_string(operator: ArithmeticOperators) -> &'static str {
    match operator {
        ArithmeticOperators::Addition => "Addition",
        ArithmeticOperators::Subtraction => "Subtraction",
        ArithmeticOperators::Multiplication => "Multiplication",
        ArithmeticOperators::Exponentiation => "Exponentiation",
        ArithmeticOperators::Division => "Division",
        ArithmeticOperators::Modulus => "Modulus",
        ArithmeticOperators::Null => "",
    }
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

pub fn assignment_operator_to_string(operator: AssignmentOperators) -> &'static str {
    match operator {
        AssignmentOperators::Assignment => "Assignment",
        AssignmentOperators::AdditionAssignment => "AdditionAssignment",
        AssignmentOperators::SubtractionAssignment => "SubtractionAssignment",
        AssignmentOperators::MultiplicationAssignment => "MultiplicationAssignment",
        AssignmentOperators::DivisionAssignment => "DivisionAssignment",
        AssignmentOperators::ModulusAssignment => "ModulusAssignment",
        AssignmentOperators::ExponentiationAssignment => "ExponentiationAssignment",
        AssignmentOperators::Null => "",
    }
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

pub fn comparison_operator_to_string(operator: ComparisonOperators) -> &'static str {
    match operator {
        ComparisonOperators::Equal => "Equal",
        ComparisonOperators::NotEqual => "NotEqual",
        ComparisonOperators::GreaterThan => "GreaterThan",
        ComparisonOperators::LessThan => "LessThan",
        ComparisonOperators::GreaterThanOrEqual => "GreaterThanOrEqual",
        ComparisonOperators::LessThanOrEqual => "LessThanOrEqual",
        ComparisonOperators::Null => "",
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperators {
    And,
    Or,
    Null,
}

pub fn logical_operator_to_string(operator: LogicalOperators) -> &'static str {
    match operator {
        LogicalOperators::And => "And",
        LogicalOperators::Or => "Or",
        LogicalOperators::Null => "",
    }
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
