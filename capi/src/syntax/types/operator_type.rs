//This is will catch operator with unknown behaviour

use crate::syntax::types;
use crate::syntax::variable;
use crate::syntax::types::arithmetic_type::ArithmeticOperators;
use crate::syntax::types::comparison_type::ComparisonOperators;
use crate::syntax::types::logical_type::LogicalOpearators;
use libc::c_char;


#[repr(C)]
pub enum Operators {
    ComparisonType(ComparisonOperators),
    LogicalType(LogicalOpearators),
    ArithmeticType(ArithmeticOperators),
    Null,
}

#[repr(C)]
pub struct OperatorType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub second_is_not_null: bool,
    pub itered_cache: Box<variable::VariableCollector>,
    pub operator: Operators,
    pub operator_collect: *const c_char,
    pub operator_collected: bool,
}
