use crate::processors::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperators {
    And,
    Or,
    Null,
}

impl LogicalOperators {
    pub fn resolve_logical_operator(value: &str) -> Result<LogicalOperators, bool> {
        match value {
            "&&" => Ok(LogicalOperators::And),
            "||" => Ok(LogicalOperators::Or),
            _ => Err(true),
        }
    }
}

impl Default for LogicalOperators {
    fn default() -> Self {
        LogicalOperators::Null
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

impl ComparisonOperators {
    pub fn resolve_comparison_operator(value: &str) -> Result<ComparisonOperators, bool> {
        match value {
            "==" => Ok(ComparisonOperators::Equal),
            "!=" => Ok(ComparisonOperators::NotEqual),
            ">" => Ok(ComparisonOperators::GreaterThan),
            "<" => Ok(ComparisonOperators::LessThan),
            ">=" => Ok(ComparisonOperators::GreaterThanOrEqual),
            "<=" => Ok(ComparisonOperators::LessThanOrEqual),
            _ => Err(true),
        }
    }
}

impl Default for ComparisonOperators {
    fn default() -> Self {
        ComparisonOperators::Null
    }
}

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

impl ArithmeticOperators {
    pub fn resolve_arithmetic_operator(value: &str) -> Result<ArithmeticOperators, bool> {
        match value {
            "+" => Ok(ArithmeticOperators::Addition),
            "-" => Ok(ArithmeticOperators::Subtraction),
            "*" => Ok(ArithmeticOperators::Multiplication),
            "**" => Ok(ArithmeticOperators::Exponentiation),
            "/" => Ok(ArithmeticOperators::Division),
            "%" => Ok(ArithmeticOperators::Modulus),
            _ => Err(true),
        }
    }
}

impl Default for ArithmeticOperators {
    fn default() -> Self {
        ArithmeticOperators::Null
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

impl AssignmentOperators {
    pub fn resolve_assignment_operator(value: &str) -> Result<AssignmentOperators, bool> {
        match value {
            "=" => Ok(AssignmentOperators::Assignment),
            "+=" => Ok(AssignmentOperators::AdditionAssignment),
            "-=" => Ok(AssignmentOperators::SubtractionAssignment),
            "*=" => Ok(AssignmentOperators::MultiplicationAssignment),
            "/=" => Ok(AssignmentOperators::DivisionAssignment),
            "%=" => Ok(AssignmentOperators::ModulusAssignment),
            "**=" => Ok(AssignmentOperators::ExponentiationAssignment),
            _ => Err(true),
        }
    }
}

impl Default for AssignmentOperators {
    fn default() -> Self {
        AssignmentOperators::Null
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Operators {
    ComparisonType(ComparisonOperators),
    LogicalType(LogicalOperators),
    ArithmeticType(ArithmeticOperators),
    AssignmentType(AssignmentOperators),
    Null,
}

impl Operators {
    pub fn has_priority(&self) -> bool {
        match *self {
            Operators::ComparisonType(_) => todo!(),
            Operators::LogicalType(_) => todo!(),
            Operators::ArithmeticType(_) => todo!(),
            Operators::AssignmentType(_) => todo!(),
            Operators::Null => todo!(),
        }
    }
}

impl Default for Operators {
    fn default() -> Self {
        Operators::Null
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperatorType {
    pub first: Box<types::Processors>,
    pub first_pos: defs::Cursor,
    pub second: Box<types::Processors>,
    pub second_pos: defs::Cursor,
    pub operator: Operators,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperatorTypeCollector {
    pub data: OperatorType,
    pub first_filled: bool,
    pub second_is_not_null: bool,
    pub itered_cache: Box<types::TypeProcessor>,
    pub operator_collect: String,
    pub operator_collected: bool,
}

impl OperatorTypeCollector {
    pub fn to_definite(self) -> definite::types::operator::OperatorType {
        definite::types::operator::OperatorType {
            cloaked: false,
            first: Box::new(self.data.first.to_definite()),
            first_pos: self.data.first_pos,
            second_pos: self.data.second_pos,
            second: Box::new(self.data.first.to_definite()),
            operator: match self.data.operator {
                Operators::ComparisonType(e) => match e {
                    ComparisonOperators::Equal => ellie_core::definite::types::operator::Operators::ComparisonType(ellie_core::definite::types::comparison_type::ComparisonOperators::Equal),
                    ComparisonOperators::NotEqual => ellie_core::definite::types::operator::Operators::ComparisonType(ellie_core::definite::types::comparison_type::ComparisonOperators::NotEqual),
                    ComparisonOperators::GreaterThan => ellie_core::definite::types::operator::Operators::ComparisonType(ellie_core::definite::types::comparison_type::ComparisonOperators::GreaterThan),
                    ComparisonOperators::LessThan => ellie_core::definite::types::operator::Operators::ComparisonType(ellie_core::definite::types::comparison_type::ComparisonOperators::LessThan),
                    ComparisonOperators::GreaterThanOrEqual => ellie_core::definite::types::operator::Operators::ComparisonType(ellie_core::definite::types::comparison_type::ComparisonOperators::GreaterThanOrEqual),
                    ComparisonOperators::LessThanOrEqual => ellie_core::definite::types::operator::Operators::ComparisonType(ellie_core::definite::types::comparison_type::ComparisonOperators::LessThanOrEqual),
                    ComparisonOperators::Null => ellie_core::definite::types::operator::Operators::ComparisonType(ellie_core::definite::types::comparison_type::ComparisonOperators::Null),
                },
                Operators::LogicalType(e) => match e {
                    LogicalOperators::And => ellie_core::definite::types::operator::Operators::LogicalType(ellie_core::definite::types::logical_type::LogicalOperators::And),
                    LogicalOperators::Or => ellie_core::definite::types::operator::Operators::LogicalType(ellie_core::definite::types::logical_type::LogicalOperators::Or),
                    LogicalOperators::Null => ellie_core::definite::types::operator::Operators::LogicalType(ellie_core::definite::types::logical_type::LogicalOperators::Null),
                },
                Operators::ArithmeticType(e) => match e {
                    ArithmeticOperators::Addition => ellie_core::definite::types::operator::Operators::ArithmeticType(ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Addition),
                    ArithmeticOperators::Subtraction => ellie_core::definite::types::operator::Operators::ArithmeticType(ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Subtraction),
                    ArithmeticOperators::Multiplication => ellie_core::definite::types::operator::Operators::ArithmeticType(ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Multiplication),
                    ArithmeticOperators::Exponentiation => ellie_core::definite::types::operator::Operators::ArithmeticType(ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Exponentiation),
                    ArithmeticOperators::Division => ellie_core::definite::types::operator::Operators::ArithmeticType(ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Division),
                    ArithmeticOperators::Modulus => ellie_core::definite::types::operator::Operators::ArithmeticType(ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Modulus),
                    ArithmeticOperators::Null => ellie_core::definite::types::operator::Operators::ArithmeticType(ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Null),
                },
                Operators::AssignmentType(e) => match e {
                    AssignmentOperators::Assignment => ellie_core::definite::types::operator::Operators::AssignmentType(ellie_core::definite::types::assignment_type::AssignmentOperators::Assignment),
                    AssignmentOperators::AdditionAssignment => ellie_core::definite::types::operator::Operators::AssignmentType(ellie_core::definite::types::assignment_type::AssignmentOperators::AdditionAssignment),
                    AssignmentOperators::SubtractionAssignment => ellie_core::definite::types::operator::Operators::AssignmentType(ellie_core::definite::types::assignment_type::AssignmentOperators::SubtractionAssignment),
                    AssignmentOperators::MultiplicationAssignment => ellie_core::definite::types::operator::Operators::AssignmentType(ellie_core::definite::types::assignment_type::AssignmentOperators::MultiplicationAssignment),
                    AssignmentOperators::DivisionAssignment => ellie_core::definite::types::operator::Operators::AssignmentType(ellie_core::definite::types::assignment_type::AssignmentOperators::DivisionAssignment),
                    AssignmentOperators::ModulusAssignment => ellie_core::definite::types::operator::Operators::AssignmentType(ellie_core::definite::types::assignment_type::AssignmentOperators::ModulusAssignment),
                    AssignmentOperators::ExponentiationAssignment => ellie_core::definite::types::operator::Operators::AssignmentType(ellie_core::definite::types::assignment_type::AssignmentOperators::ExponentiationAssignment),
                    AssignmentOperators::Null => ellie_core::definite::types::operator::Operators::AssignmentType(ellie_core::definite::types::assignment_type::AssignmentOperators::Null),
                },
                Operators::Null => panic!("Unexpected behaviour"),
            },
        }
    }
}