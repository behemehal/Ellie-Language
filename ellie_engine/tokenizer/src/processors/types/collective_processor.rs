use crate::{processors::EscapeCharEmitter, syntax::types::collective_type};
use ellie_core::{defs, error};

impl crate::processors::Processor for collective_type::CollectiveTypeCollector {
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
            if letter_char == '{' {
                self.brace_started = true;
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
        } else if !self.key_collected {
            if letter_char == '"' && (!self.key_started || !self.key_ended) {
                if self.key_started {
                    self.key_pos.range_end = cursor;
                    self.key_ended = true;
                } else {
                    self.key_pos.range_start = cursor;
                    self.key_started = true;
                }
            } else if letter_char == ':' && self.key_ended {
                self.key_collected = true;
            } else if self.key_started && !self.key_ended {
                self.key_collect.push(letter_char);
            } else if !self.key_started && letter_char == '}' {
                self.complete = true;
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
        } else {
            if self.itered_cache.is_complete() && letter_char == ',' {
                self.data.entries.push(collective_type::CollectiveEntry {
                    key: self.key_collect.clone(),
                    key_pos: self.key_pos,
                    value: self.itered_cache.current.clone(),
                    value_pos: self.itered_cache.current.get_pos(),
                });
                self.itered_cache = Box::new(super::TypeProcessor::default());
            } else if (self.itered_cache.is_complete()
                || self.itered_cache.current.is_not_initialized())
                && letter_char == '}'
            {
                self.complete = true;
            } else if !self.complete {
                hang = self
                    .itered_cache
                    .iterate(errors, cursor, last_char, letter_char);
            }
        }
        hang
    }
}
