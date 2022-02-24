use crate::syntax::types::char_type;
use ellie_core::{defs, error};

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
                    "0x35".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else {
            if letter_char == '\'' && last_char != '\\' {
                self.complete = true;
                self.pos.range_end = cursor;
            } else if !self.complete {
                self.value = letter_char;
                self.complete = true;
            } else {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "0x52".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        }
        false
    }
}
