use crate::syntax::types;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::string::String;

#[derive(PartialEq, Debug, Clone, Serialize, Hash)]
pub enum LogicalOpearators {
    And,
    Or,
    Null,
}

impl Default for LogicalOpearators {
    fn default() -> Self {
        LogicalOpearators::Null
    }
}

impl LogicalOpearators {
    pub fn is_logical_opearator(value: &str) -> bool {
        "|&".contains(value)
    }

    pub fn resolve_logical_operator(value: &str) -> Result<LogicalOpearators, bool> {
        if value == "&&" {
            Ok(LogicalOpearators::And)
        } else if value == "||" {
            Ok(LogicalOpearators::Or)
        } else {
            Err(true)
        }
    }
}

#[derive(PartialEq, Debug, Clone, Default, Serialize, Hash)]
pub struct LogicalType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: LogicalOpearators,
    pub operator_collect: String,
    pub operator_collected: bool,
    pub child_start: bool,
}
