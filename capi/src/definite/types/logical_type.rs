use crate::definite::types;
use alloc::boxed::Box;


#[repr(C)]
pub enum LogicalOperators {
    And,
    Or,
    Null,
}

#[repr(C)]
pub struct LogicalType {
    pub first: Box<types::Types>,
    pub second: Box<types::Types>,
    pub operator: LogicalOperators,
}
