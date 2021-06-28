use crate::parser;
use crate::syntax::constructor;
use ellie_core::{defs, error, utils};

use crate::alloc::boxed::Box;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn collect_constructor(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    options: defs::ParserOptions,
) {
    if let parser::Collecting::Constructor(ref mut constructordata) = parser.current {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if !constructordata.named {
            if current_reliability.reliable
                && (last_char != " " || constructordata.data.name.is_empty())
            {
                if constructordata.data.name.is_empty() {
                    constructordata.data.name_pos.range_start = parser.pos;
                }

                constructordata.data.name += letter_char;
                constructordata.data.name_pos.range_end = parser.pos;
            } else if letter_char == "(" {
                constructordata.named = true;
                constructordata.data.parameters_pos.range_start = parser.pos;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.clone() + "/function_processor",
                    debug_message: "9841eff3bfeba5c42edc6ec6bd6168be".to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skipChar(1),
                    },
                });
            }
        } else if !constructordata.parameter_wrote {
            let mut last_entry = constructordata.data.parameters.len();

            if last_entry == 0 && current_reliability.reliable {
                //...reliable will make sure in case of no parameter used no parameter data will be applied
                constructordata
                    .data
                    .parameters
                    .push(constructor::ConstructorParameter::default());
                last_entry = 1;
            }

            if current_reliability.reliable
                && (last_char != " "
                    || constructordata.data.parameters[last_entry - 1]
                        .name
                        .is_empty())
            {
                if constructordata.data.parameters[last_entry - 1]
                    .name
                    .is_empty()
                {
                    constructordata.data.parameters[last_entry - 1]
                        .pos
                        .range_start = parser.pos;
                }
                constructordata.data.parameters[last_entry - 1].name += letter_char;
            } else if letter_char == ")" && !constructordata.at_comma {
                constructordata.parameter_wrote = true;
            } else if letter_char == "," && !constructordata.at_comma {
                constructordata.at_comma = true;
                constructordata
                    .data
                    .parameters
                    .push(constructor::ConstructorParameter::default());
            } else if !constructordata.has_code {
                if letter_char == "{" {
                    constructordata.has_code = true;
                } else if letter_char == ";" {
                    parser.collected.push(parser.current.clone());
                    parser.current = parser::Collecting::None;
                } else if letter_char != " " {
                    errors.push(error::Error {
                        scope: parser.scope.clone() + "/function_processor",
                        debug_message: "9841eff3bfeba5c42edc6ec6bd6168be".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                }
            } else if constructordata.brace_count == 0 && letter_char == "}" {
                constructordata.data.inside_code = constructordata.code.collected.clone();
                constructordata.code = Box::new(parser::Parser::default()); //Empty the cache
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else {
                if letter_char == "{" {
                    constructordata.brace_count += 1;
                } else if letter_char == "}" && constructordata.brace_count != 0 {
                    constructordata.brace_count -= 1;
                }

                let mut child_parser = constructordata.code.clone();
                child_parser.options = parser.options.clone();
                let mut child_parser_errors: Vec<error::Error> = Vec::new();
                parser::iterator::iter(
                    &mut child_parser,
                    &mut child_parser_errors,
                    letter_char,
                    next_char,
                    last_char,
                );

                for i in child_parser_errors {
                    let mut edited = i;
                    edited.pos.range_start.0 += parser.pos.0;
                    edited.pos.range_start.1 += parser.pos.1;
                    edited.pos.range_end.0 += parser.pos.0;
                    edited.pos.range_end.1 += parser.pos.1;
                    errors.push(edited);
                }
                constructordata.code = child_parser;
            }
        }
    }
}
