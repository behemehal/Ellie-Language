use crate::processors::{reliable_char, Processor};
use ellie_core::{definite::types::integer, defs, error};

#[derive(Default, Clone, Debug)]
pub struct IntegerProcessor {
    pub raw_size: String,
    pub size: integer::IntegerSize,
    pub errors: Vec<error::Error>,
    pub cursor: defs::CursorPosition,
    pub forward: Option<ellie_core::definite::types::Types>,
    pub complete: bool,
}

impl Processor<ellie_core::definite::types::Types> for IntegerProcessor {
    fn new() -> Self {
        IntegerProcessor::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(&mut self, cursor: defs::CursorPosition, last_char: char, letter_char: char) {
        let is_num = letter_char.to_string().parse::<i8>().is_ok();

        if is_num {
            self.raw_size += &letter_char.to_string();
            if let Ok(nm) = self.raw_size.parse::<i8>() {
                self.size = integer::IntegerSize::I8(nm);
            } else if let Ok(nm) = self.raw_size.parse::<i16>() {
                self.size = integer::IntegerSize::I16(nm);
            } else if let Ok(nm) = self.raw_size.parse::<i32>() {
                self.size = integer::IntegerSize::I32(nm);
            } else if let Ok(nm) = self.raw_size.parse::<i64>() {
                self.size = integer::IntegerSize::I64(nm);
            } else if let Ok(nm) = self.raw_size.parse::<i128>() {
                self.size = integer::IntegerSize::I128(nm);
            } else if let Ok(nm) = self.raw_size.parse::<isize>() {
                self.size = integer::IntegerSize::Isize(nm);
            } else {
                self.errors.push(error::errorList::error_s16.clone().build(
                    vec![error::ErrorBuildField {
                        key: "val".to_owned(),
                        value: self.raw_size.clone(),
                    }],
                    "0x36".to_owned(),
                    defs::Cursor {
                        range_start: cursor,
                        range_end: cursor.clone().skip_char(1),
                    },
                ));
            }
            self.complete = true;
        } else {
            if letter_char == '-' && self.raw_size == "" {
                self.raw_size = "-".to_string();
            } else if letter_char != '\0' {
                self.errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "0x36".to_owned(),
                    defs::Cursor {
                        range_start: cursor,
                        range_end: cursor.clone().skip_char(1),
                    },
                ));
            }
        }
    }

    fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }

    fn errors(&self) -> Vec<error::Error> {
        self.errors.clone()
    }

    fn is_complete(&self) -> bool {
        self.complete
    }

    fn is_forwarded(&self) -> Option<ellie_core::definite::types::Types> {
        self.forward.clone()
    }
}
