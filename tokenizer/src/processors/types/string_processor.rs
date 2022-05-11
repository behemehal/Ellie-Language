use crate::syntax::types::string_type;
use ellie_core::{defs, error, utils::is_escape};

impl crate::processors::Processor for string_type::StringTypeCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        if !self.comma_started {
            if letter_char == '"' {
                self.comma_started = true;
                self.data.pos.range_start = cursor;
            } else if letter_char != ' ' {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "0x38".to_owned(),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            }
        } else {
            if letter_char == '"' && last_char != '\\' {
                self.complete = true;
                self.data.pos.range_end = cursor;
            } else {
                if last_char == '\\' && !is_escape(letter_char) {
                    errors.push(error::error_list::ERROR_S1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "0x38".to_owned(),
                        defs::Cursor::build_from_cursor(cursor),
                    ));
                } else {
                    self.data.value += &letter_char.to_string();
                }
            }
        }
        false
    }
}
