use crate::processors::Processor;
use crate::syntax::types::char_type;
use ellie_core::{definite::types::integer, defs, error, utils::reliable_name_range};

impl Processor for char_type::CharType {
    fn new() -> Self {
        char_type::CharType::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.comma_started {
            if letter_char == '\'' {
                self.comma_started = true;
                self.comma_start_pos = defs::Cursor::build_with_skip_char(cursor);
            } else if letter_char != ' ' {
                errors.push(error::errorList::error_s1.clone().build(
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
                self.comma_end_pos = defs::Cursor::build_with_skip_char(cursor);
            } else if !self.complete {
                self.value = letter_char;
                self.complete = true;
            } else {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "0x52".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        }
    }
}
