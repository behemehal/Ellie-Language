use crate::syntax::types::operator_type::{self, ComparisonOperators};
use ellie_core::{definite::Converter, defs, error, utils};

impl crate::processors::Processor for operator_type::OperatorTypeCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;
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
                self.data.pos.range_end = cursor;
            } else {
                if self.operator_collect != "" {
                    self.operator_collected = true;
                }
            }
        }

        if !self.operator_collected {
            self.operator_collect += &letter_char.to_string();
        } else {
            hang = self
                .itered_cache
                .iterate(errors, cursor, last_char, letter_char);

            if letter_char != ' ' && self.data.second_pos.range_start.is_zero() {
                self.data.second_pos.range_start = cursor.clone();
            }

            if self.itered_cache.is_complete() {
                self.data.second_pos.range_end = cursor.clone();
                self.data.second = Box::new(self.itered_cache.current.clone());
                match *self.data.first.clone() {
                    crate::processors::types::Processors::Operator(e) => {
                        if !utils::is_operators_chainable(
                            e.data.operator.clone().to_definite(),
                            self.data.operator.clone().to_definite(),
                        ) {
                            if matches!(
                                *e.data.first,
                                crate::processors::types::Processors::Variable(_)
                            ) && e.data.operator
                                == operator_type::Operators::ComparisonType(
                                    ComparisonOperators::LessThan,
                                )
                                && self.data.operator
                                    == operator_type::Operators::ComparisonType(
                                        ComparisonOperators::GreaterThan,
                                    )
                            {
                                errors.push(error::error_list::ERROR_S41.clone().build(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: "Functions with generics are not supported yet. See progress here https://github.com/behemehal/Ellie-Language/issues/60".to_string(),
                                    }],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    defs::Cursor { range_start: e.data.first_pos.range_start, range_end: self.data.second_pos.range_end.clone().skip_char(1) },
                                ));
                                hang = true;
                            } else {
                                errors.push(error::error_list::ERROR_S53.clone().build(
                                    vec![error::ErrorBuildField {
                                        key: "opType".to_string(),
                                        value: e.data.operator.clone().name_of_group().to_string(),
                                    }],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    defs::Cursor {
                                        range_start: e.data.pos.range_start,
                                        range_end: e.data.pos.range_end.clone().skip_char(1),
                                    },
                                ));
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        hang
    }
}
