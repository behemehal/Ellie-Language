use crate::definite::types;
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

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
