use crate::syntax::types;
use libc::c_char;

#[repr(C)]
pub enum ArithmeticOperators {
    Addition,
    Subtraction,
    Multiplication,
    Exponentiation,
    Division,
    Modulus,
    Null,
}

#[repr(C)]
pub struct ArithmeticType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: ArithmeticOperators,
    pub operator_collect: *const c_char,
    pub operator_collected: bool,
}
