use crate::{syntax::types::brace_reference_type, processors::EscapeCharEmitter};
use ellie_core::{defs, error};

impl crate::processors::Processor for brace_reference_type::BraceReferenceTypeCollector {
    fn emits_line_endings(&self) -> EscapeCharEmitter {
        self.itered_cache.emits_line_endings()
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;
        if !self.brace_started {
            if letter_char == '[' {
                self.brace_started = true;
                self.data.brace_pos.range_start = cursor;
            } else if letter_char != ' ' {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: letter_char.to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            }
        } else if !self.complete {
            if self.itered_cache.is_complete() && letter_char == ']' {
                self.data.brace_pos.range_end = cursor;
                self.data.pos.range_end = cursor;
                self.complete = true;
                self.data.value = Box::new(self.itered_cache.current.clone());
            } else {
                hang = self
                    .itered_cache
                    .iterate(errors, cursor, last_char, letter_char);
            }
        } else if letter_char != ' ' {
            errors.push(error::error_list::ERROR_S1.clone().build(
                vec![error::ErrorBuildField {
                    key: "token".to_owned(),
                    value: letter_char.to_string(),
                }],
                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                defs::Cursor {
                    range_start: cursor,
                    range_end: cursor,
                },
            ));
        }
        hang
    }
}
