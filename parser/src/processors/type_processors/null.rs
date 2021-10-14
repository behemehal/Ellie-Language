use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::processors::type_processors;
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::{defs, error};

pub fn collect_null<F, E>(
    parser: parser::Parser<F, E>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Sized,
{
    if let types::Types::Null = itered_data.data.value {
        if itered_data.raw_value.is_empty() {
            if letter_char == "." {
                itered_data.data.value =
                    types::Types::Float(types::float_type::FloatTypeCollector {
                        base: "0".to_owned(),
                        at_point: true,
                        ..Default::default()
                    });
            } else if letter_char == "\"" {
                itered_data.data.value =
                    types::Types::String(types::string_type::StringTypeCollector {
                        data: types::string_type::StringType {
                            comma_start_pos: defs::Cursor {
                                range_start: parser.pos.clone().pop_char(1),
                                range_end: parser.pos,
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
            } else if letter_char == "'" {
                itered_data.data.value = types::Types::Char(types::char_type::CharType::default());
            } else if letter_char.parse::<i64>().is_ok() {
                // Default integer
                itered_data.data.value =
                    types::Types::Integer(types::integer_type::IntegerTypeCollector::default());
                //TODO PERFORMANCE BUG
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
            } else if letter_char == "?" {
                itered_data.data.value =
                    types::Types::NullResolver(types::null_resolver::NullResolver::default());
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
                itered_data.data.value = types::Types::VariableType(
                    types::variable_type::VariableTypeCollector::default(),
                );
                type_processors::variable::collect_variable(
                    parser.clone(),
                    itered_data,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                )
            }
        } else if letter_char != " " {
            if (next_char == ";" || next_char == " ")
                && itered_data.raw_value.parse::<i64>().is_ok()
            {
                panic!("This should have been happened XC11");
            }
            itered_data.raw_value += &letter_char;
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
