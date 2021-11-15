use crate::processors::Processor;
use crate::syntax::types::float_type;
use ellie_core::{defs, error, utils::reliable_name_range};

impl Processor for float_type::FloatTypeCollector {
    fn new() -> Self {
        float_type::FloatTypeCollector::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.at_point {
            if letter_char == '.' {
                self.at_point = true;
            } else {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "0x34".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else {
            if letter_char.to_string().parse::<i8>().is_ok() {
                self.point += &letter_char.to_string();
                self.data.raw += &letter_char.to_string();
                let f32_parse = self.data.raw.parse::<f32>();
                if f32_parse.is_ok() && self.data.raw.len() < 9 {
                    if f32_parse.clone().unwrap().is_infinite() {
                        errors.push(error::errorList::error_s17.clone().build(
                            vec![error::ErrorBuildField {
                                key: "val".to_owned(),
                                value: (self.point.clone() + &letter_char.to_string()),
                            }],
                            "0x35".to_owned(),
                            defs::Cursor::build_with_skip_char(cursor),
                        ));
                    } else {
                        self.data.value = float_type::FloatSize::F32(f32_parse.unwrap());
                        self.data.rtype = float_type::FloatTypes::F32;
                        self.complete = true;
                    }
                } else if let Ok(flt) = self.data.raw.parse::<f64>() {
                    if flt.is_infinite() {
                        errors.push(error::errorList::error_s17.clone().build(
                            vec![error::ErrorBuildField {
                                key: "val".to_owned(),
                                value: (self.point.clone() + &letter_char.to_string()),
                            }],
                            "0x50".to_owned(),
                            defs::Cursor::build_with_skip_char(cursor),
                        ));
                    } else {
                        self.data.value = float_type::FloatSize::F64(flt);
                        self.data.rtype = float_type::FloatTypes::F64;
                        self.complete = true;
                    }
                } else {
                    errors.push(error::errorList::error_s17.clone().build(
                        vec![error::ErrorBuildField {
                            key: "val".to_owned(),
                            value: self.data.raw.clone(),
                        }],
                        "0x64".to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                }
            } else {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "0x74".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        }
    }
}
