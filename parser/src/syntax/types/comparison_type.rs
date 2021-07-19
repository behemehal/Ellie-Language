use crate::syntax::types;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(PartialEq, Debug, Clone, Serialize)]
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
    pub fn _is_comparison_opearator(value: &str) -> bool {
        let v: Vec<char> = "=!<>".chars().collect();
        let q: Vec<char> = value.chars().filter(|x| !v.contains(x)).collect();
        q.is_empty()
    }

    pub fn is_comparison_opearator(value: &str) -> bool {
        value == "=="
            || value == "!="
            || value == "> "
            || value == "< "
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
            "<=" => Ok(ComparisonOperators::GreaterThan),
            _ => Err(true),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ComparisonType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: ComparisonOperators,
    pub operator_collect: String,
    pub operator_collected: bool,
}
