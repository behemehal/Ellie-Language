use crate::syntax::types::function_call_type;
use ellie_core::{defs, error};

impl crate::processors::Processor for function_call_type::FunctionCallCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;
        if !self.param_started {
            if letter_char == '(' {
                self.param_started = true;
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
                    .push(function_call_type::FunctionCallParameter::default());
                self.itered_cache = Box::new(super::TypeProcessor::default())
            } else if (self.itered_cache.is_complete() || self.data.parameters.is_empty())
                && letter_char == ')'
            {
                self.data.pos.range_end = cursor.clone();
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
                        .push(function_call_type::FunctionCallParameter {
                            value: self.itered_cache.current.clone(),
                            pos: defs::Cursor::build_from_cursor(cursor),
                        });
                } else {
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
