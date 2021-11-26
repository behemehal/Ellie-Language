use crate::syntax::types::operator_type;
use ellie_core::{defs, error, utils};

impl super::Processor for operator_type::OperatorTypeCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.operator_collected {
            if let Some(operator) =
                utils::resolve_operator(&(self.operator_collect.clone() + &letter_char.to_string()))
            {
                match operator {
                    utils::FoundExtended::LogicalOperator => {
                        match operator_type::LogicalOperators::resolve_logical_operator(
                            &(self.operator_collect.clone() + &letter_char.to_string()),
                        ) {
                            Ok(op) => {
                                self.data.operator = operator_type::Operators::LogicalType(op)
                            }
                            Err(_) => panic!("Unexpected behaviour"),
                        }
                    }
                    utils::FoundExtended::ComparisonOperator => {
                        match operator_type::ComparisonOperators::resolve_comparison_operator(
                            &(self.operator_collect.clone() + &letter_char.to_string()),
                        ) {
                            Ok(op) => {
                                self.data.operator = operator_type::Operators::ComparisonType(op)
                            }
                            Err(_) => panic!("Unexpected behaviour"),
                        }
                    }
                    utils::FoundExtended::ArithmeticOperator => {
                        match operator_type::ArithmeticOperators::resolve_arithmetic_operator(
                            &(self.operator_collect.clone() + &letter_char.to_string()),
                        ) {
                            Ok(op) => {
                                self.data.operator = operator_type::Operators::ArithmeticType(op)
                            }
                            Err(_) => panic!("Unexpected behaviour"),
                        }
                    }
                    utils::FoundExtended::AssignmentOperator => {
                        match operator_type::AssignmentOperators::resolve_assignment_operator(
                            &(self.operator_collect.clone() + &letter_char.to_string()),
                        ) {
                            Ok(op) => {
                                self.data.operator = operator_type::Operators::AssignmentType(op)
                            }
                            Err(_) => panic!("Unexpected behaviour"),
                        }
                    }
                }
            } else {
                if self.operator_collect != "" {
                    self.operator_collected = true;
                }
            }
        }

        if !self.operator_collected {
            self.operator_collect += &letter_char.to_string();
        } else {
            self.itered_cache
                .iterate(errors, cursor, last_char, letter_char);
            if self.itered_cache.is_complete() {
                self.data.second = Box::new(self.itered_cache.current.clone());
            }
        }

        /*
        if self.complete {



            self.definite = self.data.clone();



            if letter_char == '3' {
                panic!("{:#?}", self.data);
            }
        }
        */
    }
}
