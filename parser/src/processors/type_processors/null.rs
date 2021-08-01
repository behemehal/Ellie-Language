use crate::parser;
use crate::processors::type_processors;
use crate::syntax::{definers, types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn collect_null(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::Null = itered_data.data.value {
        //let is_num = itered_data.raw_value.parse::<usize>().is_ok();
        if itered_data.raw_value.is_empty() {
            if letter_char == "." {
                itered_data.data.value =
                    types::Types::Float(types::float_type::FloatTypeCollector {
                        base: "0".to_string(),
                        at_point: true,
                        ..Default::default()
                    });
            } else if letter_char == "\"" {
                if itered_data.data.dynamic {
                    itered_data.data.rtype =
                        definers::DefinerCollecting::Generic(definers::GenericType {
                            rtype: "string".to_string(),
                        });
                }
                itered_data.data.value =
                    types::Types::String(types::string_type::StringType::default());
            } else if letter_char == "'" {
                if itered_data.data.dynamic {
                    itered_data.data.rtype =
                        definers::DefinerCollecting::Generic(definers::GenericType {
                            rtype: "char".to_string(),
                        });
                }
                itered_data.data.value = types::Types::Char(types::char_type::CharType::default());
            } else if (itered_data.raw_value.clone() + letter_char)
                .parse::<i64>()
                .is_ok()
            {
                // Default integer
                itered_data.data.value =
                    types::Types::Integer(types::integer_type::IntegerType::default());
                type_processors::integer::collect_integer(
                    parser.clone(),
                    itered_data,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                )
            } else if letter_char == "[" {
                if itered_data.data.dynamic {
                    itered_data.data.rtype =
                        definers::DefinerCollecting::GrowableArray(definers::GrowableArrayType {
                            rtype: Box::new(crate::syntax::definers::DefinerCollecting::Dynamic),
                            ..Default::default()
                        });
                }
                itered_data.data.value =
                    types::Types::Array(types::array_type::ArrayTypeCollector {
                        data: types::array_type::ArrayType {
                            layer_size: 0,
                            collective: Vec::new(),
                        },
                        child_start: false,
                        complete: false,
                        comma: false,
                    });
            } else if letter_char == "@" {
                itered_data.data.value = types::Types::ArrowFunction(
                    types::arrow_function::ArrowFunctionCollector::default(),
                );
            } else if letter_char == "{" {
                itered_data.data.value = types::Types::Collective(
                    types::collective_type::CollectiveCollector::default(),
                );
            } else if letter_char == "!" {
                itered_data.data.value =
                    types::Types::Negative(types::negative_type::Negative::default());
            } else if letter_char == "(" {
                itered_data.data.value =
                    types::Types::Cloak(types::cloak_type::CloakTypeCollector {
                        child_start: false,
                        complete: false,
                        comma: false,
                        data: types::cloak_type::CloakType {
                            layer_size: 0,
                            collective: Vec::new(),
                        },
                    });
            } else if letter_char != " " {
                itered_data.data.value =
                    types::Types::VariableType(types::variable_type::VariableType {
                        value_complete: false,
                        value: itered_data.raw_value.clone() + letter_char,
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            ..Default::default()
                        },
                    });
            }
        } else if letter_char != " " {
            if (next_char == ";" || next_char == " ")
                && itered_data.raw_value.parse::<i64>().is_ok()
            {
                panic!("This should have been happened XC11");
            }
            itered_data.raw_value += &letter_char;
        }
    }
}
