use crate::{syntax::items::variable::VariableCollector, processors::EscapeCharEmitter};
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for VariableCollector {
    fn emits_line_endings(&self) -> EscapeCharEmitter {
        self.value_cache.emits_line_endings()
    }

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
            } else if letter_char == ':' {
                self.data.has_type = true;
                self.name_collected = true;
            } else if letter_char == '=' {
                self.data.has_value = true;
                self.name_collected = true;
                self.type_collected = true;
            } else if letter_char == ';' {
                errors.push(error::error_list::ERROR_S8.clone().build(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_from_cursor(cursor),
                ));
                self.data.pos.range_end = cursor;
                self.complete = true;
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
        } else if !self.type_collected {
            if self.type_cache.complete && letter_char == ';' {
                self.data.pos.range_end = cursor;
                self.data.hash = ellie_core::utils::generate_hash_usize();
                self.type_collected = true;
                self.data.rtype = self.type_cache.clone();
                self.complete = true;
            } else if self.type_cache.complete && letter_char == '=' {
                self.data.has_value = true;
                self.type_collected = true;
                self.data.rtype = self.type_cache.clone();
            } else {
                if self.data.type_pos.range_start.is_zero() && letter_char != ' ' {
                    self.data.type_pos.range_start = cursor;
                }
                if letter_char != ' ' {
                    self.data.type_pos.range_end = cursor;
                }
                hang = self
                    .type_cache
                    .iterate(errors, cursor, last_char, letter_char);
            }
        } else if !self.value_collected {
            if self.value_cache.is_complete() && letter_char == ';' {
                self.data.pos.range_end = cursor;
                self.data.hash = ellie_core::utils::generate_hash_usize();
                self.complete = true;
                self.data.value = self.value_cache.current.clone();
            } else {
                if self.data.value_pos.range_start.is_zero() && letter_char != ' ' {
                    self.data.value_pos.range_start = cursor;
                }
                self.data.value_pos.range_end = cursor;
                hang = self
                    .value_cache
                    .iterate(errors, cursor, last_char, letter_char);
            }
        }
        hang
    }
}
