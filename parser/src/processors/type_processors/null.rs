use crate::processors::type_processors;
use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn collect_null(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
) {
    if let types::Types::Null = itered_data.data.value {
        //let is_num = itered_data.raw_value.parse::<usize>().is_ok();
        if itered_data.raw_value.is_empty() {
            if letter_char == "\"" {
                if itered_data.data.dynamic {
                    itered_data.rtype = crate::syntax::definers::DefinerCollecting::Generic(
                        crate::syntax::definers::GenericType {
                            rtype: "string".to_string(),
                        },
                    );
                }
                itered_data.data.value =
                    types::Types::String(types::string_type::StringType::default());
            } else if letter_char == "'" {
                if itered_data.data.dynamic {
                    itered_data.rtype = crate::syntax::definers::DefinerCollecting::Generic(
                        crate::syntax::definers::GenericType {
                            rtype: "char".to_string(),
                        },
                    );
                }
                itered_data.data.value = types::Types::Char(types::char_type::CharType::default());
            } else if (itered_data.raw_value.clone() + letter_char)
                .parse::<i64>()
                .is_ok()
            {
                itered_data.data.value =
                    types::Types::Number(types::number_type::NumberType::default());
                type_processors::number::collect_number(
                    itered_data,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                    pos,
                )
            } else if letter_char == "[" {
                if itered_data.data.dynamic {
                    itered_data.rtype = crate::syntax::definers::DefinerCollecting::GrowableArray(
                        crate::syntax::definers::GrowableArrayType {
                            rtype: Box::new(crate::syntax::definers::DefinerCollecting::Dynamic),
                            ..Default::default()
                        },
                    );
                }
                itered_data.data.value = types::Types::Array(types::array_type::ArrayType {
                    layer_size: 0,
                    child_start: false,
                    complete: false,
                    comma: false,
                    collective: Vec::new(),
                });
            } else if letter_char == "@" {
                itered_data.data.value =
                    types::Types::ArrowFunction(types::arrow_function::ArrowFunctionCollector {
                        complete: false,
                        ..Default::default()
                    });
            } else if letter_char == "{" {
                panic!("Collective is not complete");
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
                    rtype: types::number_type::NumberTypes::I64,
                    raw: itered_data.raw_value.clone() + letter_char,
                    ..Default::default()
                })
            }
            itered_data.raw_value += &letter_char;
        }
    }
}
