use ellie_core::{defs, error};

use crate::processors::Processor;
use crate::syntax::types::negative_type;

impl Processor for negative_type::Negative {
    fn new() -> Self {
        negative_type::Negative::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<ellie_core::error::Error>,
        cursor: ellie_core::defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.char_available {
            if letter_char == '!' {
                self.char_available = true;
            } else {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "neg_0x33".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else {
            self.itered_cache
                .iterate(errors, cursor, last_char, letter_char);
            if self.itered_cache.is_complete() {
                self.value = Box::new(self.itered_cache.current.clone());
            }
        }
    }
}
