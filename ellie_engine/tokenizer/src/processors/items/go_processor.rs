use crate::syntax::items::go::Go;
use ellie_core::{defs, error};

impl crate::processors::Processor for Go {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        _last_char: char,
        letter_char: char,
    ) -> bool {
        if letter_char == ';' {
            self.pos.range_end = cursor;
            self.complete = true;
        } else if letter_char != ' ' {
            errors.push(error::error_list::ERROR_S1.clone().build(
                vec![error::ErrorBuildField {
                    key: "token".to_string(),
                    value: letter_char.to_string(),
                }],
                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                defs::Cursor::build_from_cursor(cursor),
            ));
        }
        true
    }
}
