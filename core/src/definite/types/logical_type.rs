use crate::definite::types;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::string::String;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperators {
    And,
    Or,
    Null,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LogicalType {
    pub first: Box<types::Types>,
    pub second: Box<types::Types>,
    pub operator: LogicalOperators,
}
