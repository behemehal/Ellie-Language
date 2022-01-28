use crate::syntax::types::as_keyword;
use ellie_core::{defs, error};

impl crate::processors::Processor for as_keyword::AsKeywordCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.keyword_collected {
            if self.keyword_pos == 0 {
                if letter_char == 'a' {
                    self.keyword_pos = 1;
                } else {
                    errors.push(error::error_list::ERROR_S1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        file!().to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                }
            } else if self.keyword_pos == 1 {
                if letter_char == 's' {
                    self.keyword_collected = true;
                } else {
                    errors.push(error::error_list::ERROR_S1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        file!().to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                }
            }
        } else {
            if self.data.type_pos.is_zero() {
                self.data.type_pos.range_start = cursor;
            }
            self.data
                .rtype
                .iterate(errors, cursor, last_char, letter_char);
            self.complete = self.data.rtype.complete;
            self.data.type_pos.range_end = cursor.clone().skip_char(1);
        }
        self.data.pos.range_end = cursor.clone().skip_char(1);
    }
}
