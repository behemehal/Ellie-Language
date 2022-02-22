use crate::syntax::types::null_resolver;
use ellie_core::{defs, error};

impl crate::processors::Processor for null_resolver::NullResolver {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        _last_char: char,
        letter_char: char,
    ) {
        if self.pos.range_end.is_zero() {
            self.pos.range_end = cursor;
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
}
