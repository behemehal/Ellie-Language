use crate::syntax::types;
use libc::c_char;

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
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: ComparisonOperators,
    pub operator_collect: *const c_char,
    pub operator_collected: bool,
}
