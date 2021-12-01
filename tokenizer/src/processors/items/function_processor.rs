use crate::syntax::items::function;
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for function::FunctionCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.name_collected {
            if utils::reliable_name_range(utils::ReliableNameRanges::Type, letter_char).reliable {
                if self.data.name == "" {
                    self.data.name_pos.range_start = cursor;
                } else if last_char == ' ' {
                    errors.push(error::errorList::error_s1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "var_0x23".to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                }
                self.data.name_pos.range_end = cursor;
                self.data.name += &letter_char.to_string();
            } else if letter_char == '(' && self.data.name != "" {
                self.name_collected = true;
            } else if letter_char != ' ' {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "var_0x40".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else if !self.parameters_collected {
            let param_len = self.data.parameters.len();
            if !self.key_collected {
                if utils::reliable_name_range(utils::ReliableNameRanges::Type, letter_char).reliable
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
                    errors.push(error::errorList::error_s1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "var_0x40".to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
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
                    self.data.parameters[param_len - 1].rtype.iterate(
                        errors,
                        cursor,
                        last_char,
                        letter_char,
                    )
                }
            }
        } else if !self.return_keyword_collected {
            if letter_char == ':' {
                self.return_keyword_collected = true;
            } else if letter_char == '{' {
                self.return_keyword_collected = true;
                self.return_collected = true;
            } else if letter_char == ';' {
                self.data.defining = true;
                self.return_keyword_collected = true;
                self.return_collected = true;
                self.complete = true;
            } else if letter_char != ' ' {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "var_0x102".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else if !self.return_collected {
            if self.data.return_type.complete && letter_char == '{' {
                self.return_collected = true;
            } else if self.data.return_type.complete && letter_char == ';' {
                self.data.defining = true;
                self.return_collected = true;
                self.complete = true;
            } else {
                self.data
                    .return_type
                    .iterate(errors, cursor, last_char, letter_char);
            }
        } else if letter_char == '}' && self.brace_count == 0 {
            self.complete = true;
            self.iterator.finalize();
            errors.extend(self.iterator.errors.clone());
            self.data.body = self.iterator.collected.clone();
        } else {
            if letter_char == '{' {
                self.brace_count += 1;
            } else if letter_char == '}' && self.brace_count != 0 {
                self.brace_count -= 1;
            }
            self.iterator.iterate(last_char, letter_char);
        }
    }
}

/*

*/
