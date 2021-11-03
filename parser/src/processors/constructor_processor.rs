use crate::alloc::borrow::ToOwned;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::syntax::constructor;
use ellie_core::{defs, error, utils};

pub fn collect_constructor<F, E>(
    parser: &mut parser::Parser<F, E>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Sized,
{
    if let parser::Collecting::Constructor(ref mut constructor_data) = parser.current {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if !constructor_data.named {
            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n")
                    || constructor_data.data.name.is_empty())
            {
                if constructor_data.data.name.is_empty() {
                    constructor_data.data.name_pos.range_start = parser.pos;
                }

                constructor_data.data.name += letter_char;
                constructor_data.data.name_pos.range_end = parser.pos.clone().skip_char(1);
            } else if letter_char == "(" {
                constructor_data.named = true;
                constructor_data.data.parameters_pos.range_start = parser.pos;
            } else if letter_char != " " {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "109463cbb59a6b5ec1579556d39f32b0".to_owned(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: letter_char.to_string(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if !constructor_data.parameter_wrote {
            let mut last_entry = constructor_data.data.parameters.len();

            if last_entry == 0 && current_reliability.reliable {
                //...reliable will make sure in case of no parameter used no parameter data will be applied
                constructor_data
                    .data
                    .parameters
                    .push(constructor::ConstructorParameter::default());
                last_entry = 1;
            }

            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n")
                    || constructor_data.data.parameters[last_entry - 1]
                        .name
                        .is_empty())
            {
                if constructor_data.data.parameters[last_entry - 1]
                    .name
                    .is_empty()
                {
                    constructor_data.data.parameters[last_entry - 1]
                        .pos
                        .range_start = parser.pos;
                }
                constructor_data.data.parameters[last_entry - 1]
                    .pos
                    .range_end = parser.pos.skip_char(1);
                constructor_data.at_comma = false;
                constructor_data.data.parameters[last_entry - 1].name += letter_char;
            } else if letter_char == ")" && !constructor_data.at_comma {
                constructor_data.parameter_wrote = true;
            } else if letter_char == "," && !constructor_data.at_comma {
                constructor_data.at_comma = true;
                constructor_data
                    .data
                    .parameters
                    .push(constructor::ConstructorParameter::default());
            } else if letter_char != " " {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "d455295175c034d2acb182d328caf2a2".to_owned(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: letter_char.to_string(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if !constructor_data.has_code {
            if letter_char == "{" {
                constructor_data.has_code = true;
            } else if letter_char == ";" {
                constructor_data.data.pos.range_end = parser.pos.clone().skip_char(1);
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char != " " {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "0c774d1458e85950679da570dac75520".to_owned(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: letter_char.to_string(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if constructor_data.brace_count == 0 && letter_char == "}" {
            constructor_data.data.pos.range_end = parser.pos.clone().skip_char(1);
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if letter_char == "{" {
                constructor_data.brace_count += 1;
            } else if letter_char == "}" && constructor_data.brace_count != 0 {
                constructor_data.brace_count -= 1;
            }

            //let code_letter = if last_char.clone() == "\n" || last_char.clone() == "\r" {
            //    last_char + letter_char //Make sure we get the lines correctly
            //} else {
            //    letter_char.to_string()
            //};
            //constructor_data.code += &code_letter;
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
