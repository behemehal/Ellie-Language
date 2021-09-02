//This is will catch operator with unknown behaviour

use crate::syntax::types;
use crate::syntax::variable;
use serde::{Deserialize, Serialize};

use crate::syntax::types::arithmetic_type::ArithmeticOperators;
use crate::syntax::types::comparison_type::ComparisonOperators;
use crate::syntax::types::logical_type::LogicalOperators;

use alloc::boxed::Box;
use alloc::string::String;
use ellie_core::definite;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Operators {
    ComparisonType(ComparisonOperators),
    LogicalType(LogicalOperators),
    ArithmeticType(ArithmeticOperators),
    Null,
}

impl Operators {
    pub fn is_comparison_operator(value: &str) -> bool {
        value == "=="
            || value == "!="
            || value == ">"
            || value == "<"
            || value == ">="
            || value == "<="
    }

    pub fn might_be_operator(rtype: Operators, value: &str) -> bool {
        match rtype {
            Operators::ComparisonType(_) => {
                types::comparison_type::ComparisonOperators::might_comparison_operator(value)
            }
            Operators::LogicalType(_) => {
                types::logical_type::LogicalOperators::might_logical_operator(value)
            }
            Operators::ArithmeticType(_) => {
                types::arithmetic_type::ArithmeticOperators::might_arithmetic_operator(value)
            }
            _ => false,
        }
    }

    pub fn resolve_operator(rtype: Operators, value: &str) -> Result<Operators, bool> {
        match rtype {
            Operators::ComparisonType(_) => {
                if let Ok(e) =
                    types::comparison_type::ComparisonOperators::resolve_comparison_operator(value)
                {
                    Ok(Operators::ComparisonType(e))
                } else {
                    Err(true)
                }
            }
            Operators::LogicalType(_) => {
                if let Ok(e) =
                    types::logical_type::LogicalOperators::resolve_logical_operator(value)
                {
                    Ok(Operators::LogicalType(e))
                } else {
                    Err(true)
                }
            }
            Operators::ArithmeticType(_) => {
                if let Ok(e) =
                    types::arithmetic_type::ArithmeticOperators::resolve_arithmetic_operator(value)
                {
                    Ok(Operators::ArithmeticType(e))
                } else {
                    Err(true)
                }
            }
            _ => Err(true),
        }
    }
}

impl Default for Operators {
    fn default() -> Self {
        Operators::Null
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperatorType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub second: Box<types::Types>,
    pub operator: Operators,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperatorTypeCollector {
    pub data: OperatorType,
    pub cloaked: bool,
    pub first_filled: bool,
    pub second_is_not_null: bool,
    pub itered_cache: Box<variable::VariableCollector>,
    pub operator_collect: String,
    pub operator_collected: bool,
}

impl OperatorTypeCollector {
    pub fn to_definite(self) -> definite::types::operator::OperatorType {
        definite::types::operator::OperatorType {
            cloaked: self.cloaked,
            first: Box::new(self.data.first.to_definite()),
            second: Box::new(self.data.second.to_definite()),
            operator: match self.data.operator {
                Operators::ComparisonType(e) => match e {
                    ComparisonOperators::Equal => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::Equal),
                    ComparisonOperators::NotEqual => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::NotEqual),
                    ComparisonOperators::GreaterThan => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::GreaterThan),
                    ComparisonOperators::LessThan => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::LessThan),
                    ComparisonOperators::GreaterThanOrEqual => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::GreaterThanOrEqual),
                    ComparisonOperators::LessThanOrEqual => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::LessThanOrEqual),
                    ComparisonOperators::Null => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::Null),
                },
                Operators::LogicalType(e) => match e {
                    LogicalOperators::And => definite::types::operator::Operators::LogicalType(definite::types::logical_type::LogicalOperators::And),
                    LogicalOperators::Or => definite::types::operator::Operators::LogicalType(definite::types::logical_type::LogicalOperators::Or),
                    LogicalOperators::Null => definite::types::operator::Operators::LogicalType(definite::types::logical_type::LogicalOperators::Null),
                },
                Operators::ArithmeticType(e) => match e {
                    ArithmeticOperators::Addition => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Addition),
                    ArithmeticOperators::Subtraction => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Subtraction),
                    ArithmeticOperators::Multiplication => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Multiplication),
                    ArithmeticOperators::Exponentiation => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Exponentiation),
                    ArithmeticOperators::Division => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Division),
                    ArithmeticOperators::Modulus => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Modulus),
                    ArithmeticOperators::Null => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Null),
                },
                Operators::Null => definite::types::operator::Operators::Null,
            },
        }
    }
}
