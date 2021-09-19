use crate::syntax::types;
use ellie_core::definite;
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

impl Default for ArithmeticOperators {
    fn default() -> Self {
        ArithmeticOperators::Null
    }
}

impl ArithmeticOperators {
    pub fn might_arithmetic_operator(value: &str) -> bool {
        value == "+" || value == "-" || value == "*" || value == "/" || value == "%"
    }

    pub fn is_arithmetic_operator(value: &str) -> bool {
        value == "+"
            || value == "-"
            || value == "*"
            || value == "**"
            || value == "/"
            || value == "%"
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArithmeticType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: ArithmeticOperators,
    pub operator_collect: String,
    pub operator_collected: bool,
}

impl ArithmeticType {
    pub fn to_definite(self) -> definite::types::arithmetic_type::ArithmeticType {
        definite::types::arithmetic_type::ArithmeticType {
            cloaked: self.cloaked,
            first: Box::new(self.first.to_definite()),
            second: Box::new(self.second.to_definite()),
            operator: match self.operator {
                ArithmeticOperators::Addition => {
                    definite::types::arithmetic_type::ArithmeticOperators::Addition
                }
                ArithmeticOperators::Subtraction => {
                    definite::types::arithmetic_type::ArithmeticOperators::Subtraction
                }
                ArithmeticOperators::Multiplication => {
                    definite::types::arithmetic_type::ArithmeticOperators::Multiplication
                }
                ArithmeticOperators::Exponentiation => {
                    definite::types::arithmetic_type::ArithmeticOperators::Exponentiation
                }
                ArithmeticOperators::Division => {
                    definite::types::arithmetic_type::ArithmeticOperators::Division
                }
                ArithmeticOperators::Modulus => {
                    definite::types::arithmetic_type::ArithmeticOperators::Modulus
                }
                ArithmeticOperators::Null => {
                    definite::types::arithmetic_type::ArithmeticOperators::Null
                }
            },
        }
    }

    pub fn from_definite(self, from: definite::types::arithmetic_type::ArithmeticType) -> Self {
        ArithmeticType {
            first: Box::new(types::Types::default().from_definite(*from.first)),
            second: Box::new(types::Types::default().from_definite(*from.second)),
            operator: match from.operator {
                definite::types::arithmetic_type::ArithmeticOperators::Addition => {
                    ArithmeticOperators::Addition
                }
                definite::types::arithmetic_type::ArithmeticOperators::Subtraction => {
                    ArithmeticOperators::Subtraction
                }
                definite::types::arithmetic_type::ArithmeticOperators::Multiplication => {
                    ArithmeticOperators::Multiplication
                }
                definite::types::arithmetic_type::ArithmeticOperators::Exponentiation => {
                    ArithmeticOperators::Exponentiation
                }
                definite::types::arithmetic_type::ArithmeticOperators::Division => {
                    ArithmeticOperators::Division
                }
                definite::types::arithmetic_type::ArithmeticOperators::Modulus => {
                    ArithmeticOperators::Modulus
                }
                definite::types::arithmetic_type::ArithmeticOperators::Null => {
                    ArithmeticOperators::Null
                }
            },
            first_filled: true,
            operator_collected: true,
            ..Default::default()
        }
    }
}
