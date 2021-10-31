use crate::syntax::types;
use alloc::boxed::Box;
use alloc::string::String;
use ellie_core::definite;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum AssignmentOperators {
    Assignment,
    AdditionAssignment,
    SubtractionAssignment,
    MultiplicationAssignment,
    DivisionAssignment,
    ModulusAssignment,
    ExponentiationAssignment,
    Null,
}

impl Default for AssignmentOperators {
    fn default() -> Self {
        AssignmentOperators::Null
    }
}

impl AssignmentOperators {
    pub fn might_assignment_operator(value: &str) -> bool {
        value == "=" || value == "-" || value == "*" || value == "/" || value == "%" || value == "+"
    }

    pub fn is_assignment_operator(value: &str) -> bool {
        value == "="
            || value == "+="
            || value == "-="
            || value == "*="
            || value == "/="
            || value == "%="
            || value == "**="
    }

    pub fn resolve_assignment_operator(value: &str) -> Result<AssignmentOperators, bool> {
        match value {
            "=" => Ok(AssignmentOperators::Assignment),
            "+=" => Ok(AssignmentOperators::AdditionAssignment),
            "-=" => Ok(AssignmentOperators::SubtractionAssignment),
            "*=" => Ok(AssignmentOperators::MultiplicationAssignment),
            "/=" => Ok(AssignmentOperators::DivisionAssignment),
            "%=" => Ok(AssignmentOperators::ModulusAssignment),
            "**=" => Ok(AssignmentOperators::ExponentiationAssignment),
            _ => Err(true),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssignmentType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_filled: bool,
    pub second: Box<types::Types>,
    pub operator: AssignmentOperators,
    pub operator_collect: String,
    pub operator_collected: bool,
}

impl AssignmentType {
    pub fn to_definite(self) -> definite::types::assignment_type::AssignmentType {
        definite::types::assignment_type::AssignmentType {
            cloaked: self.cloaked,
            first: Box::new(self.first.to_definite()),
            second: Box::new(self.second.to_definite()),
            operator: match self.operator {
                AssignmentOperators::Assignment => {
                    definite::types::assignment_type::AssignmentOperators::Assignment
                }
                AssignmentOperators::AdditionAssignment => {
                    definite::types::assignment_type::AssignmentOperators::AdditionAssignment
                }
                AssignmentOperators::SubtractionAssignment => {
                    definite::types::assignment_type::AssignmentOperators::SubtractionAssignment
                }
                AssignmentOperators::MultiplicationAssignment => {
                    definite::types::assignment_type::AssignmentOperators::MultiplicationAssignment
                }
                AssignmentOperators::DivisionAssignment => {
                    definite::types::assignment_type::AssignmentOperators::DivisionAssignment
                }
                AssignmentOperators::ModulusAssignment => {
                    definite::types::assignment_type::AssignmentOperators::ModulusAssignment
                }
                AssignmentOperators::ExponentiationAssignment => {
                    definite::types::assignment_type::AssignmentOperators::ExponentiationAssignment
                }
                AssignmentOperators::Null => {
                    definite::types::assignment_type::AssignmentOperators::Null
                }
            },
        }
    }

    pub fn from_definite(self, from: definite::types::assignment_type::AssignmentType) -> Self {
        AssignmentType {
            cloaked: from.cloaked,
            first: Box::new(types::Types::default().from_definite(*from.first.clone())),
            first_filled: true,
            second: Box::new(types::Types::default().from_definite(*from.first.clone())),
            operator: match from.operator {
                definite::types::assignment_type::AssignmentOperators::Assignment => {
                    AssignmentOperators::Assignment
                }
                definite::types::assignment_type::AssignmentOperators::AdditionAssignment => {
                    AssignmentOperators::AdditionAssignment
                }
                definite::types::assignment_type::AssignmentOperators::SubtractionAssignment => {
                    AssignmentOperators::SubtractionAssignment
                }
                definite::types::assignment_type::AssignmentOperators::MultiplicationAssignment => {
                    AssignmentOperators::MultiplicationAssignment
                }
                definite::types::assignment_type::AssignmentOperators::DivisionAssignment => {
                    AssignmentOperators::DivisionAssignment
                }
                definite::types::assignment_type::AssignmentOperators::ModulusAssignment => {
                    AssignmentOperators::ModulusAssignment
                }
                definite::types::assignment_type::AssignmentOperators::ExponentiationAssignment => {
                    AssignmentOperators::ExponentiationAssignment
                }
                definite::types::assignment_type::AssignmentOperators::Null => {
                    AssignmentOperators::Null
                }
            },
            ..Default::default()
        }
    }
}
