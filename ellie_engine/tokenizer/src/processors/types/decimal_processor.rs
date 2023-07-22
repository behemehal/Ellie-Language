use crate::syntax::types::decimal_type;
use ellie_core::{definite::types::decimal::DecimalTypeEnum, defs, error};

impl crate::processors::Processor for decimal_type::DecimalTypeCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
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
            if last_char == ' ' {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            } else if letter_char.to_string().parse::<i8>().is_ok() {
                self.point += &letter_char.to_string();
                self.data.raw += &letter_char.to_string();
                if let Ok(nm) = self.data.raw.parse::<f64>() {
                    self.data.value = DecimalTypeEnum::Float(nm);
                    self.complete = true;
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
            } else if letter_char == 'f' {
                self.data.pos.range_end = cursor;
                if let Ok(nm) = self.data.raw.parse::<f64>() {
                    self.data.value = DecimalTypeEnum::Float(nm);
                    self.data.is_double = false;
                    self.complete = true;
                } else {
                    errors.push(error::error_list::ERROR_S17.clone().build(
                        vec![error::ErrorBuildField {
                            key: "val".to_owned(),
                            value: self.data.raw.clone(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        defs::Cursor::build_from_cursor(cursor),
                    ));
                }
            } else if letter_char == 'd' {
                self.data.pos.range_end = cursor;
                if let Ok(nm) = self.data.raw.parse::<f32>() {
                    self.data.value = DecimalTypeEnum::Double(nm);
                    self.complete = false;
                } else {
                    errors.push(error::error_list::ERROR_S17.clone().build(
                        vec![error::ErrorBuildField {
                            key: "val".to_owned(),
                            value: self.data.raw.clone(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        defs::Cursor::build_from_cursor(cursor),
                    ));
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
        }
        false
    }
}
