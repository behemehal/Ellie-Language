use crate::syntax::items::function;
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for function::FunctionCollector {
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
                if self.data.name == "" {
                    self.data.name_pos.range_start = cursor;
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
                self.data.name_pos.range_end = cursor;
                self.data.name += &letter_char.to_string();
            } else if letter_char == '(' && self.data.name != "" {
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
            let param_len = self.data.parameters.len();
            if !self.key_collected {
                if utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
                    .reliable
                {
                    if param_len == 0 {
                        self.data.parameters.push(function::FunctionParameter {
                            name: letter_char.to_string(),
                            pos: defs::Cursor {
                                range_start: cursor,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                    } else {
                        if self.data.parameters[param_len - 1].name == "" {
                            self.data.parameters[param_len - 1].pos.range_start = cursor;
                        }
                        self.data.parameters[param_len - 1].pos.range_end = cursor;
                        self.data.parameters[param_len - 1].name += &letter_char.to_string();
                    }
                } else if letter_char == '*'
                    && (param_len == 0 || self.data.parameters[param_len - 1].name == "")
                {
                    if param_len == 0 {
                        self.data.parameters.push(function::FunctionParameter {
                            multi_capture: true,
                            pos: defs::Cursor {
                                range_start: cursor,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                    } else {
                        self.data.parameters[param_len - 1].pos.range_start = cursor;
                        self.data.parameters[param_len - 1].multi_capture = true;
                    }
                } else if letter_char == ':'
                    && self.data.parameters.len() != 0
                    && self.data.parameters[param_len - 1].name != ""
                {
                    self.data.parameters[param_len - 1].pos.range_end = cursor;
                    self.key_collected = true;
                } else if letter_char == ')' && self.data.parameters.len() == 0 {
                    self.key_collected = true;
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
                if self.data.parameters[param_len - 1].rtype.complete && letter_char == ',' {
                    self.key_collected = false;
                    self.data
                        .parameters
                        .push(function::FunctionParameter::default());
                } else if self.data.parameters[param_len - 1].rtype.complete && letter_char == ')' {
                    self.key_collected = true;
                    self.parameters_collected = true;
                } else {
                    hang = self.data.parameters[param_len - 1].rtype.iterate(
                        errors,
                        cursor,
                        last_char,
                        letter_char,
                    );
                }
            }
        } else if !self.return_keyword_collected {
            if letter_char == ':' {
                self.return_keyword_collected = true;
            } else if letter_char == '{' {
                self.data.no_return = true;
                self.return_keyword_collected = true;
                self.data.return_type.definer_type =
                    crate::syntax::items::definers::DefinerTypes::Generic(
                        crate::syntax::items::definers::GenericType {
                            rtype: "void".to_string(),
                            pos: self.data.name_pos,
                        },
                    );
                self.data.no_return = true;
                self.return_collected = true;
            } else if letter_char == ';' {
                self.data.defining = true;
                self.return_keyword_collected = true;
                self.return_collected = true;
                self.complete = true;
                self.data.no_return = true;
                self.data.return_type.definer_type =
                    crate::syntax::items::definers::DefinerTypes::Generic(
                        crate::syntax::items::definers::GenericType {
                            rtype: "void".to_string(),
                            pos: self.data.name_pos,
                        },
                    );
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
        } else if !self.return_collected {
            if self.data.return_type.complete && letter_char == '{' {
                self.return_collected = true;
                self.data.body_pos.range_start = cursor;
            } else if self.data.return_type.complete && letter_char == ';' {
                self.data.hash = ellie_core::utils::generate_hash_u64();
                self.data.defining = true;
                self.return_collected = true;
                self.complete = true;
            } else {
                if self.data.return_pos.range_start.is_zero() && letter_char != ' ' {
                    self.data.return_pos.range_start = cursor;
                }
                if letter_char != ' ' {
                    self.data.return_pos.range_end = cursor;
                }
                hang = self
                    .data
                    .return_type
                    .iterate(errors, cursor, last_char, letter_char);
            }
        } else if letter_char == '}' && self.brace_count == 0 {
            self.complete = true;
            self.data.body_pos.range_end = cursor;
            self.data.pos.range_end = cursor;
            self.data.hash = ellie_core::utils::generate_hash_u64();
            self.iterator.finalize();
            errors.extend(self.iterator.errors.clone());
            self.data.body = self.iterator.collected.clone();
            let contains_ret = self.data.body.iter().any(|x| match x {
                super::Processors::Ret(_) => true,
                _ => false,
            });

            if !self.data.no_return && !contains_ret {
                let mut error = error::error_list::ERROR_S2.clone().build(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    self.data.body_pos,
                );

                error.reference_message = "Defined here".to_string();
                error.reference_block = Some((self.data.return_pos, "<fill>".to_string()));

                errors.push(error);
            }
        } else {
            if letter_char == '{' {
                self.brace_count += 1;
            } else if letter_char == '}' && self.brace_count != 0 {
                self.brace_count -= 1;
            }
            self.iterator.pos = cursor;
            hang = self.iterator.iterate(last_char, letter_char);
        }
        hang
    }
}

/*

*/
