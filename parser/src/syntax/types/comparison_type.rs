use crate::syntax::types;
use serde::Deserialize;
use serde::Serialize;

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

impl Default for ComparisonOperators {
    fn default() -> Self {
        ComparisonOperators::Null
    }
}

impl ComparisonOperators {
    pub fn might_comparison_operator(value: &str) -> bool {
        value == "=" || value == "!" || value == ">" || value == "<"
    }

    pub fn is_comparison_operator(value: &str) -> bool {
        value == "=="
            || value == "!="
            || value == ">"
            || value == "<"
            || value == ">="
            || value == "<="
    }

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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComparisonType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: ComparisonOperators,
    pub operator_collect: String,
    pub operator_collected: bool,
}
