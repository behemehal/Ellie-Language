use crate::syntax::types::array_type;
use ellie_core::{defs, error};

impl crate::processors::Processor for array_type::ArrayTypeCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.brace_started {
            if letter_char == '[' {
                self.brace_started = true;
            } else if letter_char != ' ' {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "val".to_owned(),
                        value: letter_char.to_string(),
                    }],
                    "array_0x21".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else if self.itered_cache.is_complete() && letter_char == ',' {
            self.data.collective.push(array_type::ArrayEntry::default());
            self.itered_cache = Box::new(super::TypeProcessor::default())
        } else if (self.itered_cache.is_complete() || self.data.collective.is_empty())
            && letter_char == ']'
        {
            self.complete = true;
            self.itered_cache = Box::new(super::TypeProcessor::default())
        } else if !self.complete {
            self.itered_cache
                .iterate(errors, cursor, last_char, letter_char);

            let param_len = self.data.collective.len();
            if param_len == 0 {
                self.data.collective.push(array_type::ArrayEntry {
                    value: self.itered_cache.current.clone(),
                    location: defs::Cursor::build_with_skip_char(cursor),
                });
            } else {
                self.data.collective[param_len - 1].value = self.itered_cache.current.clone();
                self.data.collective[param_len - 1].location.range_end =
                    cursor.clone().skip_char(1);
            }
        } else if letter_char != ' ' {
            errors.push(error::errorList::error_s1.clone().build(
                vec![error::ErrorBuildField {
                    key: "val".to_owned(),
                    value: letter_char.to_string(),
                }],
                "array_0x54".to_owned(),
                defs::Cursor::build_with_skip_char(cursor),
            ));
        }
    }
}
