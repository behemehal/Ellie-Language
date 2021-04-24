use crate::error;
use crate::mapper;
use crate::syntax::{types, variable};

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    _errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    _last_char: String,
    _pos: mapper::defs::CursorPosition,
) {
    if let types::Types::Null = itered_data.data.value {
        //let is_num = itered_data.raw_value.parse::<usize>().is_ok();
        if itered_data.raw_value.is_empty() {
            if letter_char == "\"" || letter_char == "'" {
                itered_data.data.value = types::Types::String(types::string_type::StringType {
                    quote_type: letter_char.to_string(),
                    ..Default::default()
                })
            } else if (itered_data.raw_value.clone() + letter_char)
                .parse::<i32>()
                .is_ok()
            {
                itered_data.data.value = types::Types::Number(types::number_type::NumberType {
                    value: (itered_data.raw_value.clone() + letter_char)
                        .parse::<usize>()
                        .unwrap(),
                    complete: false,
                })
            } else if letter_char == "[" {
                println!("Array Started");
                itered_data.data.value = types::Types::Array(types::array_type::ArrayType {
                    layer_size: 0,
                    child_start: false,
                    complete: false,
                    comma: false,
                    collective: Vec::new(),
                });
            } else if letter_char == "{" {
                panic!("Collective is deprecated");
            } else if letter_char == "(" {
                println!("Cloak Started");
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
                && itered_data.raw_value.parse::<i32>().is_ok()
            {
                itered_data.data.value = types::Types::Number(types::number_type::NumberType {
                    value: (itered_data.raw_value.clone() + letter_char)
                        .parse::<usize>()
                        .unwrap(),
                    complete: false,
                })
            }
            itered_data.raw_value += &letter_char;
        }
    }
}
