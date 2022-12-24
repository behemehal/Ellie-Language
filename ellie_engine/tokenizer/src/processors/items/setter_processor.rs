use crate::syntax::items::{function, setter};
use ellie_core::{
    defs::{self, Cursor},
    error, utils,
};

impl crate::processors::Processor for setter::Setter {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;

        if !self.name_collected {
            if utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
                .reliable
            {
                if self.name == "" {
                    self.name_pos.range_start = cursor;
                } else if last_char == ' ' {
                    errors.push(error::error_list::ERROR_S1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        defs::Cursor::build_from_cursor(cursor),
                    ));
                }
                self.name_pos.range_end = cursor;
                self.name += &letter_char.to_string();
            } else if letter_char == '(' && self.name != "" {
                self.parameters_pos.range_start = cursor;
                self.name_collected = true;
            } else if letter_char != ' ' {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            }
        } else if !self.parameters_collected {
            let param_len = self.parameters.len();
            if !self.key_collected {
                if utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
                    .reliable
                {
                    if param_len == 0 {
                        self.parameters.push(function::FunctionParameter {
                            name: letter_char.to_string(),
                            name_pos: defs::Cursor {
                                range_start: cursor,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                    } else {
                        if self.parameters[param_len - 1].name == "" {
                            self.parameters[param_len - 1].name_pos.range_start = cursor;
                        }
                        self.parameters[param_len - 1].name_pos.range_end = cursor;
                        self.parameters[param_len - 1].name += &letter_char.to_string();
                    }
                } else if letter_char == '*'
                    && (param_len == 0 || self.parameters[param_len - 1].name == "")
                {
                    if param_len == 0 {
                        self.parameters.push(function::FunctionParameter {
                            multi_capture: true,
                            name_pos: defs::Cursor {
                                range_start: cursor,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                    } else {
                        self.parameters[param_len - 1].name_pos.range_start = cursor;
                        self.parameters[param_len - 1].multi_capture = true;
                    }
                } else if letter_char == ':'
                    && self.parameters.len() != 0
                    && self.parameters[param_len - 1].name != ""
                {
                    self.parameters[param_len - 1].name_pos.range_end = cursor;
                    self.key_collected = true;
                } else if letter_char == ')' && self.parameters.len() == 0 {
                    self.key_collected = true;
                    self.parameters_pos.range_end = cursor;
                    self.parameters_collected = true;
                } else if letter_char != ' ' {
                    errors.push(error::error_list::ERROR_S1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        defs::Cursor::build_from_cursor(cursor),
                    ));
                }
            } else {
                if self.parameters[param_len - 1].rtype_pos.is_zero() {
                    self.parameters[param_len - 1].rtype_pos.range_start = cursor;
                }
                if self.parameters[param_len - 1].rtype.complete && letter_char == ',' {
                    self.key_collected = false;
                    self.parameters.push(function::FunctionParameter::default());
                } else if self.parameters[param_len - 1].rtype.complete && letter_char == ')' {
                    self.key_collected = true;
                    self.parameters_collected = true;
                } else {
                    self.parameters[param_len - 1].rtype_pos.range_end = cursor;
                    hang = self.parameters[param_len - 1].rtype.iterate(
                        errors,
                        cursor,
                        last_char,
                        letter_char,
                    );
                }
            }
        } else if !self.code_start_collected {
            if letter_char == '{' {
                self.code_start_collected = true;
                self.body_pos.range_start = cursor;
            } else if letter_char != ' ' {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            }
        } else if letter_char == '}' && self.brace_count == 0 {
            self.complete = true;
            self.body_pos.range_end = cursor;
            self.pos.range_end = cursor;
            self.hash = ellie_core::utils::generate_hash_usize();
            self.iterator.finalize();
            errors.extend(self.iterator.errors.clone());
            self.body = self.iterator.collected.clone();
            let contains_ret = self.body.iter().any(|x| match x {
                super::Processors::Ret(_) => true,
                _ => false,
            });

            if self.parameters.len() != 1 {
                errors.push(error::error_list::ERROR_S12.clone().build(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    self.parameters_pos,
                ));
            } else if self.parameters[0].multi_capture {
                let mut error = error::error_list::ERROR_S12.clone().build(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    self.parameters_pos,
                );
                error.reference_block = Some((
                    Cursor {
                        range_start: self.parameters[0].name_pos.range_start,
                        range_end: self.parameters[0].rtype_pos.range_end,
                    },
                    "<fill>".to_string(),
                ));
                error.reference_message = "This is a multi capture parameter".to_string();

                errors.push(error);
            }

            if contains_ret {
                errors.push(error::error_list::ERROR_S14.clone().build(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    self.body_pos,
                ));
            }
        } else {
            if letter_char == '{' {
                self.brace_count += 1;
            } else if letter_char == '}' && self.brace_count != 0 {
                self.brace_count -= 1;
            }
            self.iterator.pos = cursor;
            if cursor.0 != self.iterator.comment_pos.range_start.0 && self.iterator.line_comment {
                self.iterator.line_comment = false;
            }
            hang = self.iterator.iterate(last_char, letter_char);
        }

        hang
    }
}
