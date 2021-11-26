use crate::syntax::types::collective_type;
use ellie_core::{defs, error};

impl super::Processor for collective_type::CollectiveTypeCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.brace_started {
            if letter_char == '{' {
                self.brace_started = true;
            } else if letter_char != ' ' {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "val".to_owned(),
                        value: letter_char.to_string(),
                    }],
                    "collective_0x21".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else if self.itered_cache.is_complete()
            && self.key_collected
            && (letter_char == ',' || letter_char == '}')
        {
            if letter_char == ',' {
                self.key_collected = false;
                self.data
                    .entries
                    .push(collective_type::CollectiveEntry::default());
                self.itered_cache = Box::new(super::TypeProcessor::default())
            } else {
                self.complete = true;
            }
        } else if self.itered_cache.is_complete() && !self.key_collected && letter_char == ':' {
            self.key_collected = true;
            self.itered_cache = Box::new(super::TypeProcessor::default())
        } else if !self.complete {
            self.itered_cache
                .iterate(errors, cursor, last_char, letter_char);
            let param_len = self.data.entries.len();

            if !self.key_collected {
                if param_len == 0 {
                    self.data.entries.push(collective_type::CollectiveEntry {
                        key: self.itered_cache.current.clone(),
                        key_pos: defs::Cursor::build_with_skip_char(cursor),
                        ..Default::default()
                    });
                } else {
                    self.data.entries[param_len - 1].key = self.itered_cache.current.clone();
                    self.data.entries[param_len - 1].key_pos.range_end =
                        cursor.clone().skip_char(1);
                }
            } else {
                self.data.entries[param_len - 1].value = self.itered_cache.current.clone();
                self.data.entries[param_len - 1].value_pos.range_end = cursor.clone().skip_char(1);
            }
        } else if letter_char != ' ' {
            errors.push(error::errorList::error_s1.clone().build(
                vec![error::ErrorBuildField {
                    key: "val".to_owned(),
                    value: letter_char.to_string(),
                }],
                "collective_0x68".to_owned(),
                defs::Cursor::build_with_skip_char(cursor),
            ));
        }
    }
}
