use crate::syntax::types;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::string::String;

#[derive(PartialEq, Debug, Clone, Serialize, Hash)]
pub enum ArithmeticOperators {
    Addition,
    Subtraction,
    Multiplication,
    Exponentiation,
    Division,
    Modulus,
    Null,
}

impl Default for ArithmeticOperators {
    fn default() -> Self {
        ArithmeticOperators::Null
    }
}

impl ArithmeticOperators {
    pub fn _is_arithmetic_opearator(value: &str) -> bool {
        "+-*/%".contains(value)
    }

    pub fn is_arithmetic_opearator(value: &str) -> bool {
        value == "+ "
    }

    pub fn resolve_arithmetic_operator(value: &str) -> Result<ArithmeticOperators, bool> {
        match value {
            "+" => Ok(ArithmeticOperators::Addition),
            "-" => Ok(ArithmeticOperators::Subtraction),
            "*" => Ok(ArithmeticOperators::Multiplication),
            "**" => Ok(ArithmeticOperators::Exponentiation),
            "/" => Ok(ArithmeticOperators::Division),
            "%" => Ok(ArithmeticOperators::Modulus),
            _ => Err(true),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Default, Serialize, Hash)]
pub struct ArithmeticType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: ArithmeticOperators,
    pub operator_collect: String,
    pub operator_collected: bool,
}
