use crate::syntax::types::byte_type;
use ellie_core::{defs, error};

impl crate::processors::Processor for byte_type::ByteType {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        match letter_char.to_string().parse::<i8>() {
            Ok(num) => {
                if last_char == ' ' {
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
                    let full_value = self.value as isize;
                    let new_value = format!("{}{}", full_value, num).parse::<isize>().unwrap();
                    if new_value > 255 || new_value < -255 {
                        errors.push(error::error_list::ERROR_S16.clone().build(
                            vec![error::ErrorBuildField {
                                key: "val".to_owned(),
                                value: new_value.to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            defs::Cursor::build_from_cursor(cursor),
                        ));
                    } else {
                        self.value = new_value as i8;
                    }
                }
            }
            Err(_) => {
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
