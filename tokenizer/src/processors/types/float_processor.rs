use crate::syntax::types::float_type;
use ellie_core::{defs, error};

impl crate::processors::Processor for float_type::FloatTypeCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        _last_char: char,
        letter_char: char,
    ) -> bool {
        if !self.at_point {
            if letter_char == '.' {
                self.at_point = true;
            } else {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "0x34".to_owned(),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            }
        } else {
            if letter_char.to_string().parse::<i8>().is_ok() {
                self.point += &letter_char.to_string();
                self.data.raw += &letter_char.to_string();
                if let Ok(nm) = self.data.raw.parse::<f32>() {
                    self.data.value = nm;
                } else {
                    errors.push(error::error_list::ERROR_S16.clone().build(
                        vec![error::ErrorBuildField {
                            key: "val".to_owned(),
                            value: self.data.raw.clone(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        defs::Cursor::build_from_cursor(cursor),
                    ));
                }
                self.data.pos.range_end = cursor;
            } else {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            }
        }
        false
    }
}
