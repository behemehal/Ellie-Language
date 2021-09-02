use crate::definite::types;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::string::String;

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
    pub first: Box<types::Types>,
    pub second: Box<types::Types>,
    pub operator: ArithmeticOperators,
}
