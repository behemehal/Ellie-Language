use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::string::String;
use alloc::vec::Vec;

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    _last_char: String,
    pos: defs::CursorPosition,
) {
    if let types::Types::Null = itered_data.data.value {
        //let is_num = itered_data.raw_value.parse::<usize>().is_ok();
        if itered_data.raw_value.is_empty() {
            if letter_char == "\"" {
                if itered_data.data.dynamic {
                    itered_data.r#type = crate::syntax::r#type::Collecting::Generic(crate::syntax::r#type::GenericType { 
                        r#type: "string".to_string()
                    });
                } else if !matches!(&itered_data.r#type, crate::syntax::r#type::Collecting::Generic(x) if x.r#type == "string") {
                    errors.push(error::Error {
                        debug_message: "Nulity".to_string(),
                        title: error::errorList::error_s3.title.clone(),
                        code: error::errorList::error_s3.code,
                        message: error::errorList::error_s3.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s3.message.clone(),
                            vec![
                                error::ErrorBuildField {
                                    key: "token1".to_string(),
                                    value: itered_data.r#type.raw_name()
                                },
                                error::ErrorBuildField {
                                    key: "token2".to_string(),
                                    value: "string".to_string(),
                                },
                            ],
                        ),
                        pos: defs::Cursor {
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
                itered_data.data.value = types::Types::String(types::string_type::StringType::default());
            } else if letter_char == "'" {
                if itered_data.data.dynamic {
                    itered_data.r#type = crate::syntax::r#type::Collecting::Generic(crate::syntax::r#type::GenericType { 
                        r#type: "char".to_string()
                    });
                } else if !matches!(&itered_data.r#type, crate::syntax::r#type::Collecting::Generic(x) if x.r#type == "char") {
                    errors.push(error::Error {
                        debug_message: "Heliport".to_string(),
                        title: error::errorList::error_s3.title.clone(),
                        code: error::errorList::error_s3.code,
                        message: error::errorList::error_s3.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s3.message.clone(),
                            vec![
                                error::ErrorBuildField {
                                    key: "token1".to_string(),
                                    value: itered_data.r#type.raw_name()
                                },
                                error::ErrorBuildField {
                                    key: "token2".to_string(),
                                    value: "char".to_string(),
                                },
                            ],
                        ),
                        pos: defs::Cursor {
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
                itered_data.data.value = types::Types::Char(types::char_type::CharType::default());
            } else if (itered_data.raw_value.clone() + letter_char)
                .parse::<i64>()
                .is_ok()
            {
                itered_data.data.value = types::Types::Number(types::number_type::NumberType {
                    r#type: types::number_type::NumberTypes::I64,
                    value: types::number_type::NumberSize::I64(letter_char.parse::<i64>().unwrap()),
                    raw: itered_data.raw_value.clone() + letter_char,
                    ..Default::default()
                });
            } else if letter_char == "[" {
                itered_data.data.value = types::Types::Array(types::array_type::ArrayType {
                    layer_size: 0,
                    child_start: false,
                    complete: false,
                    comma: false,
                    collective: Vec::new(),
                });
            } else if letter_char == "@" {
                itered_data.data.value = types::Types::ArrowFunction(types::arrow_function::ArrowFunctionCollector {
                    complete: false,
                    ..Default::default()
                });
            } else if letter_char == "{" {
                panic!("Collective is deprecated");
            } else if letter_char == "(" {
                itered_data.data.value = types::Types::Cloak(types::cloak_type::CloakType {
                    layer_size: 0,
                    child_start: false,
                    complete: false,
                    comma: false,
                    collective: Vec::new(),
                });
            } else if letter_char != " " {
                itered_data.data.value =
                    types::Types::VariableType(types::variable_type::VariableType {
                        value_complete: false,
                        value: itered_data.raw_value.clone() + letter_char,
                    });
            }
        } else if letter_char != " " {
            if (next_char == ";" || next_char == " ")
                && itered_data.raw_value.parse::<i64>().is_ok()
            {
                itered_data.data.value = types::Types::Number(types::number_type::NumberType {
                    r#type: types::number_type::NumberTypes::I64,
                    raw: itered_data.raw_value.clone() + letter_char,
                    ..Default::default()
                })
            }
            itered_data.raw_value += &letter_char;
        }
    }
}
