use crate::syntax::types::as_keyword;
use ellie_core::{defs, error};

impl crate::processors::Processor for as_keyword::AsKeywordCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;
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
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                }
            }
        } else {
            if self.data.type_pos.range_start.is_zero() && letter_char != ' ' {
                self.data.type_pos.range_start = cursor;
            }
            hang = self
                .data
                .rtype
                .iterate(errors, cursor, last_char, letter_char);
            self.complete = self.data.rtype.complete;
            self.data.type_pos.range_end = cursor;
        }
        self.data.pos.range_end = cursor;
        hang
    }
}
