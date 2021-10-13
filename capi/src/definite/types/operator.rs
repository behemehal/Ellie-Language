use crate::definite::types;

use crate::definite::types::arithmetic_type::ArithmeticOperators;
use crate::definite::types::comparison_type::ComparisonOperators;
use crate::definite::types::logical_type::LogicalOperators;
use alloc::boxed::Box;
use ellie_core::definite::types::operator;

#[repr(C)]
pub enum Operators {
    ComparisonType(ComparisonOperators),
    LogicalType(LogicalOperators),
    ArithmeticType(ArithmeticOperators),
    Null,
}

#[repr(C)]
pub struct OperatorType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub second: Box<types::Types>,
    pub operator: Operators,
}

pub unsafe fn build_operator_from(target: operator::OperatorType) -> OperatorType {
    OperatorType {
        cloaked: target.cloaked,
        first: Box::new(types::build_collecting_from(*target.first)),
        second: Box::new(types::build_collecting_from(*target.second)),
        operator: match target.operator {
            operator::Operators::ComparisonType(e) => match e {
                ellie_core::definite::types::comparison_type::ComparisonOperators::Equal => Operators::ComparisonType(ComparisonOperators::Equal),
                ellie_core::definite::types::comparison_type::ComparisonOperators::NotEqual => Operators::ComparisonType(ComparisonOperators::NotEqual),
                ellie_core::definite::types::comparison_type::ComparisonOperators::GreaterThan => Operators::ComparisonType(ComparisonOperators::GreaterThan),
                ellie_core::definite::types::comparison_type::ComparisonOperators::LessThan => Operators::ComparisonType(ComparisonOperators::LessThan),
                ellie_core::definite::types::comparison_type::ComparisonOperators::GreaterThanOrEqual => Operators::ComparisonType(ComparisonOperators::GreaterThanOrEqual),
                ellie_core::definite::types::comparison_type::ComparisonOperators::LessThanOrEqual => Operators::ComparisonType(ComparisonOperators::LessThanOrEqual),
                ellie_core::definite::types::comparison_type::ComparisonOperators::Null => Operators::ComparisonType(ComparisonOperators::Null),
            },
            operator::Operators::LogicalType(e) => match e {
                ellie_core::definite::types::logical_type::LogicalOperators::And => Operators::LogicalType(LogicalOperators::And),
                ellie_core::definite::types::logical_type::LogicalOperators::Or => Operators::LogicalType(LogicalOperators::Or),
                ellie_core::definite::types::logical_type::LogicalOperators::Null => Operators::LogicalType(LogicalOperators::Null),
            },
            operator::Operators::ArithmeticType(e) => match e {
                ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Addition => Operators::ArithmeticType(ArithmeticOperators::Addition),
                ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Subtraction => Operators::ArithmeticType(ArithmeticOperators::Subtraction),
                ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Multiplication => Operators::ArithmeticType(ArithmeticOperators::Multiplication),
                ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Exponentiation => Operators::ArithmeticType(ArithmeticOperators::Exponentiation),
                ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Division => Operators::ArithmeticType(ArithmeticOperators::Division),
                ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Modulus => Operators::ArithmeticType(ArithmeticOperators::Modulus),
                ellie_core::definite::types::arithmetic_type::ArithmeticOperators::Null => Operators::ArithmeticType(ArithmeticOperators::Null),
            } ,
            operator::Operators::Null => Operators::Null,
        },
    }
}
