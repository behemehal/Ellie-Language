use crate::syntax::types;
use alloc::boxed::Box;
use alloc::string::String;
use ellie_core::definite;
use serde::{Deserialize, Serialize};

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

impl ComparisonType {
    pub fn to_definite(self) -> definite::types::comparison_type::ComparisonType {
        definite::types::comparison_type::ComparisonType {
            cloaked: self.cloaked,
            first: Box::new(self.first.to_definite()),
            second: Box::new(self.second.to_definite()),
            operator: match self.operator {
                ComparisonOperators::Equal => {
                    definite::types::comparison_type::ComparisonOperators::Equal
                }
                ComparisonOperators::NotEqual => {
                    definite::types::comparison_type::ComparisonOperators::NotEqual
                }
                ComparisonOperators::GreaterThan => {
                    definite::types::comparison_type::ComparisonOperators::GreaterThan
                }
                ComparisonOperators::LessThan => {
                    definite::types::comparison_type::ComparisonOperators::LessThan
                }
                ComparisonOperators::GreaterThanOrEqual => {
                    definite::types::comparison_type::ComparisonOperators::GreaterThanOrEqual
                }
                ComparisonOperators::LessThanOrEqual => {
                    definite::types::comparison_type::ComparisonOperators::LessThanOrEqual
                }
                ComparisonOperators::Null => {
                    definite::types::comparison_type::ComparisonOperators::Null
                }
            },
        }
    }

    pub fn from_definite(self, from: definite::types::comparison_type::ComparisonType) -> Self {
        ComparisonType {
            cloaked: from.cloaked,
            first: Box::new(types::Types::default().from_definite(*from.first.clone())),
            first_filled: true,
            second: Box::new(types::Types::default().from_definite(*from.first.clone())),
            operator: match from.operator {
                definite::types::comparison_type::ComparisonOperators::Equal => {
                    ComparisonOperators::Equal
                }
                definite::types::comparison_type::ComparisonOperators::NotEqual => {
                    ComparisonOperators::NotEqual
                }
                definite::types::comparison_type::ComparisonOperators::GreaterThan => {
                    ComparisonOperators::GreaterThan
                }
                definite::types::comparison_type::ComparisonOperators::LessThan => {
                    ComparisonOperators::LessThan
                }
                definite::types::comparison_type::ComparisonOperators::GreaterThanOrEqual => {
                    ComparisonOperators::GreaterThanOrEqual
                }
                definite::types::comparison_type::ComparisonOperators::LessThanOrEqual => {
                    ComparisonOperators::LessThanOrEqual
                }
                definite::types::comparison_type::ComparisonOperators::Null => {
                    ComparisonOperators::Null
                }
            },
            ..Default::default()
        }
    }
}
