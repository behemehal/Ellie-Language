use crate::{syntax::types::array_type, processors::EscapeCharEmitter};
use ellie_core::{defs, error};

impl crate::processors::Processor for array_type::ArrayTypeCollector {
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
        } else if (self.itered_cache.is_complete()
            || self.itered_cache.current.is_not_initialized())
            && letter_char == ','
        {
            self.data.collective.push(array_type::ArrayEntry::default());
            self.itered_cache = Box::new(super::TypeProcessor::default())
        } else if (self.itered_cache.is_complete() || self.data.collective.is_empty())
            && letter_char == ']'
        {
            self.complete = true;
            self.itered_cache = Box::new(super::TypeProcessor::default())
        } else if !self.complete {
            hang = self
                .itered_cache
                .iterate(errors, cursor, last_char, letter_char);

            let param_len = self.data.collective.len();
            if param_len == 0 {
                self.data.collective.push(array_type::ArrayEntry {
                    value: self.itered_cache.current.clone(),
                    location: defs::Cursor::build_from_cursor(cursor),
                });
            } else {
                self.data.collective[param_len - 1].value = self.itered_cache.current.clone();
                self.data.collective[param_len - 1].location.range_end =
                    cursor.clone().skip_char(1);
            }
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
        hang
    }
}
