use crate::syntax::types;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::string::String;

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum ComparisonOperators {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Null,
}

impl Default for ComparisonOperators {
    fn default() -> Self {
        ComparisonOperators::Null
    }
}

impl ComparisonOperators {
    #[no_mangle]
    pub extern "C" fn is_opearator(value: &str) -> bool {
        "=!<>".contains(value)
    }

    #[no_mangle]
    pub extern "C" fn resolve_operator(value: &str) -> Result<ComparisonOperators, bool> {
        match value {
            "==" => Ok(ComparisonOperators::Equal),
            "!=" => Ok(ComparisonOperators::NotEqual),
            ">" => Ok(ComparisonOperators::GreaterThan),
            "<" => Ok(ComparisonOperators::LessThan),
            ">=" => Ok(ComparisonOperators::GreaterThanOrEqual),
            "<=" => Ok(ComparisonOperators::GreaterThan),
            _ => Err(true),
        }
    }
}

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ComparisonType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: ComparisonOperators,
    pub operator_collect: String,
    pub operator_collected: bool,
}
