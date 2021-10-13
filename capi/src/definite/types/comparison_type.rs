use crate::definite::types;
use alloc::boxed::Box;

#[repr(C)]
pub enum ComparisonOperators {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Null,
}

#[repr(C)]
pub struct ComparisonType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub second: Box<types::Types>,
    pub operator: ComparisonOperators,
}
