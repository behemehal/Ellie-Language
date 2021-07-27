use crate::syntax::types;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::string::String;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum LogicalOperators {
    And,
    Or,
    Null,
}

impl Default for LogicalOperators {
    fn default() -> Self {
        LogicalOperators::Null
    }
}

impl LogicalOperators {
    pub fn is_logical_operator(value: &str) -> bool {
        value == "&&" || value == "||"
    }

    pub fn resolve_logical_operator(value: &str) -> Result<LogicalOperators, bool> {
        match value {
            "&&" => Ok(LogicalOperators::And),
            "||" => Ok(LogicalOperators::Or),
            _ => Err(true),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct LogicalType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: LogicalOperators,
    pub operator_collect: String,
    pub operator_collected: bool,
    pub child_start: bool,
}
