use crate::definite::types;
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum ArithmeticOperators {
    Addition,
    Subtraction,
    Multiplication,
    Exponentiation,
    Division,
    Modulus,
    Null,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ArithmeticType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub second: Box<types::Types>,
    pub operator: ArithmeticOperators,
}
