use crate::syntax::types::brace_reference_type;
use ellie_core::{defs, error};

impl crate::processors::Processor for brace_reference_type::BraceReferenceTypeCollector {
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
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "val".to_owned(),
                        value: letter_char.to_string(),
                    }],
                    "brace_refence_0x34".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else if !self.complete {
            if self.itered_cache.is_complete() && letter_char == ']' {
                self.complete = true;
                self.data.value = Box::new(self.itered_cache.current.clone());
            } else {
                self.itered_cache
                    .iterate(errors, cursor, last_char, letter_char);
            }
        } else if letter_char != ' ' {
            errors.push(error::error_list::ERROR_S1.clone().build(
                vec![error::ErrorBuildField {
                    key: "val".to_owned(),
                    value: letter_char.to_string(),
                }],
                "array_0x21".to_owned(),
                defs::Cursor::build_with_skip_char(cursor),
            ));
        }
    }
}
