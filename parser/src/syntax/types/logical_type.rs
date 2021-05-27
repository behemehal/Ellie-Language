use crate::syntax::types;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::string::String;

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Serialize)]
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
    #[no_mangle]
    pub extern "C" fn is_opearator(value: &str) -> bool {
        "|&".contains(value)
    }

    #[no_mangle]
    pub extern "C" fn resolve_operator(value: &str) -> Result<LogicalOpearators, bool> {
        if value == "&&" {
            Ok(LogicalOpearators::And)
        } else if value == "||" {
            Ok(LogicalOpearators::Or)
        } else {
            Err(true)
        }
    }
}

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Default, Serialize)]
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
