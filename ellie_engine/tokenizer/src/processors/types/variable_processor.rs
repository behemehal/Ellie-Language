use crate::syntax::types::variable_type;
use ellie_core::{
    defs, error,
    utils::{reliable_name_range, ReliableNameRanges},
};

impl crate::processors::Processor for variable_type::VariableTypeCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        if reliable_name_range(ReliableNameRanges::VariableName, letter_char).reliable {
            if last_char == ' ' && self.data.value != "" {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            } else {
                self.complete = true;
                if self.data.value == "" {
                    self.data.pos.range_start = cursor;
                }
                self.data.pos.range_end = cursor;
                self.data.value += &letter_char.to_string();
            }
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
        false
    }
}
