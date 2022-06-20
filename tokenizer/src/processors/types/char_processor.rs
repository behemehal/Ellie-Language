use crate::syntax::types::char_type;
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for char_type::CharType {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        if !self.comma_started {
            if letter_char == '\'' {
                self.comma_started = true;
                self.pos.range_start = cursor;
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
            if letter_char == '\'' && last_char != '\\' {
                self.complete = true;
                self.pos.range_end = cursor;
                if self.is_escaped {
                    if utils::is_escape(self.value) {
                        self.value = match self.value {
                            '\'' => '\'',
                            '"' => '\"',
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            '0' => '\0',
                            '\\' => '\\',
                            _ => ' ',
                        }
                    } else {
                        errors.push(error::error_list::ERROR_S5.clone().build(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: self.value.to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            self.pos,
                        ));
                    }
                }
            } else if !self.complete {
                if letter_char == '\\' {
                    self.is_escaped = true;
                } else if self.value == '\0' {
                    self.value = letter_char;
                } else {
                    errors.push(error::error_list::ERROR_S54.clone().build(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        defs::Cursor::build_from_cursor(cursor),
                    ));
                }
            }
        }
        false
    }
}
