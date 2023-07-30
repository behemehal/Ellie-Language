use crate::{
    processors::EscapeCharEmitter,
    syntax::{items::definers::DefinerCollector, types::class_call_type},
};
use ellie_core::{defs, error};

use super::TypeProcessor;
impl crate::processors::Processor for class_call_type::ClassCallCollector {
    fn emits_line_endings(&self) -> EscapeCharEmitter {
        self.itered_cache.emits_line_endings()
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;
        if !self.base_collected {
            if (letter_char == '(' || letter_char == '<') && self.itered_cache.is_complete() {
                self.base_collected = true;
                self.data.target = Box::new(self.itered_cache.current.clone());
                if letter_char == '(' {
                    self.param_collected = true;
                    self.generic_collected = true;
                } else {
                    self.data
                        .generic_parameters
                        .push(class_call_type::ClassCallGenericParameter::default());
                }
                self.itered_cache = Box::new(TypeProcessor::default());
            } else {
                if self.data.target_pos.range_start.is_zero() && letter_char != ' ' {
                    self.data.target_pos.range_start = cursor;
                }
                self.data.target_pos.range_end = cursor;
                hang = self
                    .itered_cache
                    .iterate(errors, cursor, last_char, letter_char);
            }
        } else if !self.generic_collected {
            let len = self.data.generic_parameters.len();
            if (letter_char == ',' || letter_char == '>') && self.generic_cache.complete {
                self.data.generic_parameters[len - 1].pos.range_end = cursor;
                self.data.generic_parameters[len - 1].value =
                    self.generic_cache.definer_type.clone();
                if letter_char == '>' {
                    self.generic_collected = true;
                    self.generic_cache = DefinerCollector::default();
                } else {
                    self.data.generic_parameters[len - 1].pos.range_start = cursor;
                    self.data
                        .generic_parameters
                        .push(class_call_type::ClassCallGenericParameter {
                            pos: defs::Cursor {
                                range_start: cursor,
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    self.generic_cache = DefinerCollector::default();
                }
            } else {
                if self.data.generic_parameters[len - 1]
                    .pos
                    .range_start
                    .is_zero()
                    && letter_char == ' '
                {
                    self.data.generic_parameters[len - 1].pos.range_start = cursor;
                }
                hang = self
                    .generic_cache
                    .iterate(errors, cursor, last_char, letter_char);
            }
        } else if !self.param_collected {
            if letter_char == '(' {
                self.param_collected = true;
            } else if letter_char != ' ' {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: letter_char.to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            }
        } else if !self.complete {
            if self.itered_cache.is_complete() && letter_char == ',' {
                self.data
                    .parameters
                    .push(class_call_type::ClassCallParameter::default());
                self.itered_cache = Box::new(super::TypeProcessor::default());
            } else if (self.itered_cache.is_complete() || self.data.parameters.is_empty())
                && letter_char == ')'
            {
                self.itered_cache = Box::new(super::TypeProcessor::default());
                self.complete = true;
            } else {
                hang = self
                    .itered_cache
                    .iterate(errors, cursor, last_char, letter_char);

                let param_len = self.data.parameters.len();
                if param_len == 0 {
                    self.data
                        .parameters
                        .push(class_call_type::ClassCallParameter {
                            value: self.itered_cache.current.clone(),
                            pos: defs::Cursor::build_from_cursor(cursor),
                        });
                } else {
                    if self.data.parameters[param_len - 1]
                        .pos
                        .range_start
                        .is_zero()
                        && letter_char != ' '
                    {
                        self.data.parameters[param_len - 1].pos.range_start = cursor;
                    }
                    self.data.parameters[param_len - 1].value = self.itered_cache.current.clone();
                    self.data.parameters[param_len - 1].pos.range_end = cursor;
                }
            }
        } else if letter_char != ' ' {
            errors.push(error::error_list::ERROR_S1.clone().build(
                vec![error::ErrorBuildField {
                    key: "token".to_owned(),
                    value: letter_char.to_string(),
                }],
                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                defs::Cursor::build_from_cursor(cursor),
            ));
        }
        hang
    }
}
