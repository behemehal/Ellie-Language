use crate::syntax::types;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::string::String;

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Serialize)]
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
    #[no_mangle]
    pub extern "C" fn is_opearator(value: &str) -> bool {
        "+-*/%".contains(value)
    }

    #[no_mangle]
    pub extern "C" fn resolve_operator(value: &str) -> Result<ArithmeticOperators, bool> {
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

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ArithmeticType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: ArithmeticOperators,
    pub operator_collect: String,
    pub operator_collected: bool,
}
