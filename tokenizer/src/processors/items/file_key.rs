pub use crate::syntax::items::file_key::FileKey;
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for FileKey {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.keyword_collected {
            if letter_char == '@' {
                self.keyword_collected = true;
            } else {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "filek_0x24".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else if !self.name_collected {
            if utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
                .reliable
            {
                if self.key_name == "" {
                    self.key_name_location.range_start = cursor;
                } else if last_char == ' ' {
                    errors.push(error::error_list::ERROR_S1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "var_0x23".to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                }
                self.key_name_location.range_end = cursor;
                self.key_name += &letter_char.to_string();
            } else if letter_char == '=' {
                self.value_location.range_start = cursor;
                self.name_collected = true;
            } else if letter_char != ' ' {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "var_0x40".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else {
            if self.value_cache.is_complete() && letter_char == ';' {
                self.complete = true;
                self.pos.range_end = cursor.clone().skip_char(1);
                self.value_location.range_end = cursor;
                self.value = self.value_cache.current.clone();
            } else {
                self.value_cache
                    .iterate(errors, cursor, last_char, letter_char)
            }
        }
        self.pos.range_end = cursor;
    }
}
