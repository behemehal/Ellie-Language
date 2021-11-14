use crate::processors::Processor;
use crate::syntax::types::string_type;
use ellie_core::{
    defs, error,
    utils::{is_escape, reliable_name_range},
};

impl Processor for string_type::StringTypeCollector {
    fn new() -> Self {
        string_type::StringTypeCollector::default()
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
            if letter_char == '"' {
                self.comma_started = true;
                self.data.comma_start_pos = defs::Cursor::build_with_skip_char(cursor);
            } else if letter_char != ' ' {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "0x38".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else {
            if letter_char == '"' && last_char != '\\' {
                self.complete = true;
                self.data.comma_end_pos = defs::Cursor::build_with_skip_char(cursor);
            } else {
                if self.data.value == "" {
                    self.data.value_pos.range_start = cursor;
                }
                self.data.value_pos.range_end = cursor;

                if last_char == '\\' && !is_escape(letter_char) {
                    errors.push(error::errorList::error_s1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "0x38".to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                } else {
                    self.data.value += &letter_char.to_string();
                }
            }
        }
    }
}
