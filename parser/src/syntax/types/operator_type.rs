//This is will catch operator with unknown behaviour

use crate::syntax::types;
use crate::syntax::variable;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

use crate::syntax::types::arithmetic_type::ArithmeticOperators;
use crate::syntax::types::assignment_type::AssignmentOperators;
use crate::syntax::types::comparison_type::ComparisonOperators;
use crate::syntax::types::logical_type::LogicalOperators;

use alloc::boxed::Box;
use alloc::string::String;
use ellie_core::definite;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Operators {
    ComparisonType(ComparisonOperators),
    LogicalType(LogicalOperators),
    ArithmeticType(ArithmeticOperators),
    AssignmentType(AssignmentOperators),
    Null,
}

impl Operators {
    pub fn is_comparison_operator(value: &str) -> bool {
        value == "=="
            || value == "!="
            || value == ">"
            || value == "<"
            || value == ">="
            || value == "<="
    }

    pub fn might_be_operator(rtype: Operators, value: &str) -> bool {
        match rtype {
            Operators::ComparisonType(_) => {
                types::comparison_type::ComparisonOperators::might_comparison_operator(value)
            }
            Operators::LogicalType(_) => {
                types::logical_type::LogicalOperators::might_logical_operator(value)
            }
            Operators::ArithmeticType(_) => {
                types::arithmetic_type::ArithmeticOperators::might_arithmetic_operator(value)
            }
            Operators::AssignmentType(_) => {
                types::assignment_type::AssignmentOperators::might_assignment_operator(value)
            }
            _ => false,
        }
    }

    pub fn resolve_operator(rtype: Operators, value: &str) -> Result<Operators, bool> {
        match rtype {
            Operators::ComparisonType(_) => {
                if let Ok(e) =
                    types::comparison_type::ComparisonOperators::resolve_comparison_operator(value)
                {
                    Ok(Operators::ComparisonType(e))
                } else {
                    Err(true)
                }
            }
            Operators::LogicalType(_) => {
                if let Ok(e) =
                    types::logical_type::LogicalOperators::resolve_logical_operator(value)
                {
                    Ok(Operators::LogicalType(e))
                } else {
                    Err(true)
                }
            }
            Operators::ArithmeticType(_) => {
                if let Ok(e) =
                    types::arithmetic_type::ArithmeticOperators::resolve_arithmetic_operator(value)
                {
                    Ok(Operators::ArithmeticType(e))
                } else {
                    Err(true)
                }
            }
            Operators::AssignmentType(_) => {
                if let Ok(e) =
                    types::assignment_type::AssignmentOperators::resolve_assignment_operator(value)
                {
                    Ok(Operators::AssignmentType(e))
                } else {
                    Err(true)
                }
            }
            _ => Err(true),
        }
    }
}

impl Default for Operators {
    fn default() -> Self {
        Operators::Null
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperatorType {
    pub cloaked: bool,
    pub first: Box<types::Types>,
    pub first_pos: defs::Cursor,
    pub second: Box<types::Types>,
    pub second_pos: defs::Cursor,
    pub operator: Operators,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperatorTypeCollector {
    pub data: OperatorType,
    pub cloaked: bool,
    pub first_filled: bool,
    pub second_is_not_null: bool,
    pub itered_cache: Box<variable::VariableCollector>,
    pub operator_collect: String,
    pub operator_collected: bool,
}

impl OperatorTypeCollector {
    pub fn to_definite(self) -> definite::types::operator::OperatorType {
        definite::types::operator::OperatorType {
            cloaked: self.cloaked,
            first_pos: self.data.first_pos,
            second_pos: self.data.second_pos,
            first: Box::new(self.data.first.to_definite()),
            second: Box::new(self.data.second.to_definite()),
            operator: match self.data.operator {
                Operators::ComparisonType(e) => match e {
                    ComparisonOperators::Equal => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::Equal),
                    ComparisonOperators::NotEqual => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::NotEqual),
                    ComparisonOperators::GreaterThan => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::GreaterThan),
                    ComparisonOperators::LessThan => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::LessThan),
                    ComparisonOperators::GreaterThanOrEqual => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::GreaterThanOrEqual),
                    ComparisonOperators::LessThanOrEqual => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::LessThanOrEqual),
                    ComparisonOperators::Null => ellie_core::definite::types::operator::Operators::ComparisonType(definite::types::comparison_type::ComparisonOperators::Null),
                },
                Operators::LogicalType(e) => match e {
                    LogicalOperators::And => definite::types::operator::Operators::LogicalType(definite::types::logical_type::LogicalOperators::And),
                    LogicalOperators::Or => definite::types::operator::Operators::LogicalType(definite::types::logical_type::LogicalOperators::Or),
                    LogicalOperators::Null => definite::types::operator::Operators::LogicalType(definite::types::logical_type::LogicalOperators::Null),
                },
                Operators::ArithmeticType(e) => match e {
                    ArithmeticOperators::Addition => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Addition),
                    ArithmeticOperators::Subtraction => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Subtraction),
                    ArithmeticOperators::Multiplication => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Multiplication),
                    ArithmeticOperators::Exponentiation => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Exponentiation),
                    ArithmeticOperators::Division => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Division),
                    ArithmeticOperators::Modulus => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Modulus),
                    ArithmeticOperators::Null => definite::types::operator::Operators::ArithmeticType(definite::types::arithmetic_type::ArithmeticOperators::Null),
                },
                Operators::AssignmentType(e) => match e {
                    AssignmentOperators::Assignment => definite::types::operator::Operators::AssignmentType(definite::types::assignment_type::AssignmentOperators::Assignment),
                    AssignmentOperators::AdditionAssignment => definite::types::operator::Operators::AssignmentType(definite::types::assignment_type::AssignmentOperators::AdditionAssignment),
                    AssignmentOperators::SubtractionAssignment => definite::types::operator::Operators::AssignmentType(definite::types::assignment_type::AssignmentOperators::SubtractionAssignment),
                    AssignmentOperators::MultiplicationAssignment => definite::types::operator::Operators::AssignmentType(definite::types::assignment_type::AssignmentOperators::MultiplicationAssignment),
                    AssignmentOperators::DivisionAssignment => definite::types::operator::Operators::AssignmentType(definite::types::assignment_type::AssignmentOperators::DivisionAssignment),
                    AssignmentOperators::ModulusAssignment => definite::types::operator::Operators::AssignmentType(definite::types::assignment_type::AssignmentOperators::ModulusAssignment),
                    AssignmentOperators::ExponentiationAssignment => definite::types::operator::Operators::AssignmentType(definite::types::assignment_type::AssignmentOperators::ExponentiationAssignment),
                    AssignmentOperators::Null => definite::types::operator::Operators::AssignmentType(definite::types::assignment_type::AssignmentOperators::Null),
                },
                Operators::Null => definite::types::operator::Operators::Null,
            },
        }
    }

    pub fn from_definite(self, from: definite::types::operator::OperatorType) -> Self {
        OperatorTypeCollector {
            data: OperatorType {
                cloaked: from.cloaked,
                first: Box::new(types::Types::default().from_definite(*from.first)),
                first_pos: from.first_pos,
                second: Box::new(types::Types::default().from_definite(*from.second)),
                second_pos: from.second_pos,
                operator: match from.operator {
                    definite::types::operator::Operators::ComparisonType(e) => {
                        Operators::ComparisonType(match e {
                            definite::types::comparison_type::ComparisonOperators::Equal => ComparisonOperators::Equal,
                            definite::types::comparison_type::ComparisonOperators::NotEqual => ComparisonOperators::NotEqual,
                            definite::types::comparison_type::ComparisonOperators::GreaterThan => ComparisonOperators::GreaterThan,
                            definite::types::comparison_type::ComparisonOperators::LessThan => ComparisonOperators::LessThan,
                            definite::types::comparison_type::ComparisonOperators::GreaterThanOrEqual => ComparisonOperators::GreaterThanOrEqual,
                            definite::types::comparison_type::ComparisonOperators::LessThanOrEqual => ComparisonOperators::LessThanOrEqual,
                            definite::types::comparison_type::ComparisonOperators::Null => ComparisonOperators::Null,
                        })
                    }
                    definite::types::operator::Operators::LogicalType(e) => {
                        Operators::LogicalType(
                            match e {
                                definite::types::logical_type::LogicalOperators::And => LogicalOperators::And,
                                definite::types::logical_type::LogicalOperators::Or => LogicalOperators::Or,
                                definite::types::logical_type::LogicalOperators::Null => LogicalOperators::Null,
                            }
                        )
                    }
                    definite::types::operator::Operators::ArithmeticType(e) => {
                        Operators::ArithmeticType(match e {
                            definite::types::arithmetic_type::ArithmeticOperators::Addition => ArithmeticOperators::Addition,
                            definite::types::arithmetic_type::ArithmeticOperators::Subtraction => ArithmeticOperators::Subtraction,
                            definite::types::arithmetic_type::ArithmeticOperators::Multiplication => ArithmeticOperators::Multiplication,
                            definite::types::arithmetic_type::ArithmeticOperators::Exponentiation => ArithmeticOperators::Exponentiation,
                            definite::types::arithmetic_type::ArithmeticOperators::Division => ArithmeticOperators::Division,
                            definite::types::arithmetic_type::ArithmeticOperators::Modulus => ArithmeticOperators::Modulus,
                            definite::types::arithmetic_type::ArithmeticOperators::Null => ArithmeticOperators::Null,
                        })
                    }
                    definite::types::operator::Operators::AssignmentType(e) => {
                        Operators::AssignmentType(match e {
                            definite::types::assignment_type::AssignmentOperators::Assignment => AssignmentOperators::Assignment,
                            definite::types::assignment_type::AssignmentOperators::AdditionAssignment => AssignmentOperators::AdditionAssignment,
                            definite::types::assignment_type::AssignmentOperators::SubtractionAssignment => AssignmentOperators::SubtractionAssignment,
                            definite::types::assignment_type::AssignmentOperators::MultiplicationAssignment => AssignmentOperators::MultiplicationAssignment,
                            definite::types::assignment_type::AssignmentOperators::DivisionAssignment => AssignmentOperators::DivisionAssignment,
                            definite::types::assignment_type::AssignmentOperators::ModulusAssignment => AssignmentOperators::ModulusAssignment,
                            definite::types::assignment_type::AssignmentOperators::ExponentiationAssignment => AssignmentOperators::ExponentiationAssignment,
                            definite::types::assignment_type::AssignmentOperators::Null => AssignmentOperators::Null,
                        })
                    }
                    definite::types::operator::Operators::Null => Operators::Null,
                },
            },
            cloaked: from.cloaked,
            first_filled: true,
            second_is_not_null: true,
            operator_collected: true,
            ..Default::default()
        }
    }
}
