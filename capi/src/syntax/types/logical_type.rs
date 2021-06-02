use crate::syntax::types;
use libc::c_char;

#[repr(C)]
pub enum LogicalOpearators {
    And,
    Or,
    Null,
}

#[repr(C)]
pub struct LogicalType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: LogicalOpearators,
    pub operator_collect: *const c_char,
    pub operator_collected: bool,
    pub child_start: bool,
}
