//This is will catch operator with unknown behaviour

use crate::syntax::types;
use crate::syntax::variable;
use serde::Serialize;

use crate::syntax::types::arithmetic_type::ArithmeticOperators;
use crate::syntax::types::comparison_type::ComparisonOperators;
use crate::syntax::types::logical_type::LogicalOperators;

use alloc::boxed::Box;
use alloc::string::String;

#[derive(PartialEq, Debug, Clone, Serialize)]
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
                types::comparison_type::ComparisonOperators::is_comparison_operator(value)
            }
            Operators::LogicalType(_) => {
                types::logical_type::LogicalOperators::is_logical_operator(value)
            }
            Operators::ArithmeticType(_) => {
                types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(value)
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

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct OperatorType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub second: Box<types::Types>,
    pub operator: Operators,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct OperatorTypeCollector {
    pub data: OperatorType,
    pub cloaked: bool,
    pub first_filled: bool,
    pub second_is_not_null: bool,
    pub itered_cache: Box<variable::VariableCollector>,
    pub operator_collect: String,
    pub operator_collected: bool,
}
