use crate::syntax::types::integer_type;
use ellie_core::{defs, error};

impl crate::processors::Processor for integer_type::IntegerTypeCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let is_num = letter_char.to_string().parse::<i8>().is_ok();

        if is_num {
            if self.raw == "" {
                self.data.pos.range_start = cursor;
            } else if last_char == ' ' {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            }
            self.raw += &letter_char.to_string();
            if let Ok(nm) = self.raw.parse::<i8>() {
                self.data.value = integer_type::IntegerSize::I8(nm);
                self.data.rtype = integer_type::IntegerTypes::I8;
            } else if let Ok(nm) = self.raw.parse::<i16>() {
                self.data.value = integer_type::IntegerSize::I16(nm);
                self.data.rtype = integer_type::IntegerTypes::I16;
            } else if let Ok(nm) = self.raw.parse::<i32>() {
                self.data.value = integer_type::IntegerSize::I32(nm);
                self.data.rtype = integer_type::IntegerTypes::I32;
            } else if let Ok(nm) = self.raw.parse::<i64>() {
                self.data.value = integer_type::IntegerSize::I64(nm);
                self.data.rtype = integer_type::IntegerTypes::I64;
            } else if let Ok(nm) = self.raw.parse::<i128>() {
                self.data.value = integer_type::IntegerSize::I128(nm);
                self.data.rtype = integer_type::IntegerTypes::I128;
            } else if let Ok(nm) = self.raw.parse::<isize>() {
                self.data.value = integer_type::IntegerSize::Isize(nm);
                self.data.rtype = integer_type::IntegerTypes::Isize;
            } else {
                errors.push(error::error_list::ERROR_S16.clone().build(
                    vec![error::ErrorBuildField {
                        key: "val".to_owned(),
                        value: self.raw.clone(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            }
            self.data.pos.range_end = cursor;
            self.complete = true;
        } else {
            if letter_char == '-' && self.raw == "" {
                self.raw = "-".to_string();
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
