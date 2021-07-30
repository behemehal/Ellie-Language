use crate::parser;
use crate::syntax::class;

use ellie_core::{defs, error, utils};

use crate::alloc::string::{String, ToString};

use crate::alloc::boxed::Box;
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn collect_class(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let parser::Collecting::Class(ref mut classdata) = parser.current {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if !classdata.name_collected {
            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n") || classdata.data.name.is_empty())
            {
                if classdata.data.name.is_empty() {
                    classdata.data.name_pos.range_start = parser.pos;
                }
                classdata.data.name += letter_char;
                classdata.data.name_pos.range_end = parser.pos.clone().skip_char(1);
            } else if letter_char == "<" && !classdata.data.name.is_empty() {
                if utils::is_reserved(&classdata.data.name) {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "a0fcb9d9947c40da15de0db77e25fd91".to_string(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: classdata.data.name.clone(),
                            }],
                        ),
                        pos: classdata.data.name_pos,
                    });
                }
                classdata.name_collected = true;
            } else if letter_char == "{" && !classdata.data.name.is_empty() {
                if utils::is_reserved(&classdata.data.name) {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "10cb11b8de3948358be21d0d1f9c375d".to_string(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: classdata.data.name.clone(),
                            }],
                        ),
                        pos: classdata.data.name_pos,
                    });
                }
                classdata.name_collected = true;
                classdata.has_code = true;
                classdata.generic_definings_collected = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "5583cc10e16c4aa005b8e953904f8ecf".to_string(),
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
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if !classdata.generic_definings_collected {
            let mut last_entry = classdata.data.generic_definings.len();

            if last_entry == 0 && current_reliability.reliable {
                //...reliable will make sure in case of no parameter used no parameter data will be applied
                classdata
                    .data
                    .generic_definings
                    .push(class::GenericDefining::default());
                last_entry = 1;
            }

            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n")
                    || classdata.data.generic_definings[last_entry - 1]
                        .name
                        .is_empty())
            {
                if classdata.data.generic_definings[last_entry - 1]
                    .name
                    .is_empty()
                {
                    classdata.data.generic_definings[last_entry - 1]
                        .pos
                        .range_start = parser.pos;
                }
                classdata.at_comma = false;
                classdata.data.generic_definings[last_entry - 1]
                    .pos
                    .range_end = parser.pos.clone().skip_char(1);
                classdata.data.generic_definings[last_entry - 1].name += letter_char;
            } else if letter_char == ">" && !classdata.at_comma {
                if classdata.has_dedup() {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "558d9cbc18781888e17095062e699ffc".to_string(),
                        title: error::errorList::error_s10.title.clone(),
                        code: error::errorList::error_s10.code,
                        message: error::errorList::error_s10.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s10.message.clone(),
                        ),
                        pos: classdata.data.generic_definings[last_entry - 1].pos,
                    });
                }
                classdata.generic_definings_collected = true;
            } else if letter_char == "," && !classdata.at_comma {
                if classdata.has_dedup() {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "abae6b99cbb762bf94f7f09b2d197836".to_string(),
                        title: error::errorList::error_s10.title.clone(),
                        code: error::errorList::error_s10.code,
                        message: error::errorList::error_s10.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s10.message.clone(),
                        ),
                        pos: classdata.data.generic_definings[last_entry - 1].pos,
                    });
                }
                classdata.at_comma = true;
                classdata
                    .data
                    .generic_definings
                    .push(class::GenericDefining::default());
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "d49b546c4361073818e82c1fbf3db75d".to_string(),
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
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if !classdata.has_code {
            if letter_char == "{" {
                classdata.has_code = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "e0bb1faef55491f1221c2430a38974b2".to_string(),
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
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if classdata.brace_count == 0 && letter_char == "}" {
            for i in classdata.code.collected.clone() {
                match i {
                    parser::Collecting::Variable(e) => {
                        classdata.data.properties.push(e.data);
                    }
                    parser::Collecting::Function(e) => {
                        classdata.data.methods.push(e.data);
                    }
                    parser::Collecting::Constructor(e) => {
                        if e.data.name != classdata.data.name {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "b577a61bb98102e72f74d9cadf84f7b6".to_string(),
                                title: error::errorList::error_s22.title.clone(),
                                code: error::errorList::error_s22.code,
                                message: error::errorList::error_s22.message.clone(),
                                builded_message: error::BuildedError::build_from_string(
                                    error::errorList::error_s22.message.clone(),
                                ),
                                pos: e.data.name_pos,
                            });
                        }
                        classdata.data.constructor = e.data;
                    }
                    _ => {}
                };
            }
            classdata.code = Box::new(parser::Parser::default()); //Empty the cache
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if letter_char == "{" {
                classdata.brace_count += 1;
            } else if letter_char == "}" && classdata.brace_count != 0 {
                classdata.brace_count -= 1;
            }
            let mut child_parser = classdata.code.clone();
            child_parser.options = parser.options.clone();
            child_parser.options.parser_type = defs::ParserType::ClassParser;
            child_parser.generic_variables = classdata.data.generic_definings.clone();
            child_parser.pos = parser.pos;
            child_parser.scope.scope_name = "core/class_processor".to_string();
            let mut child_parser_errors: Vec<error::Error> = Vec::new();
            parser::iterator::iter(
                &mut child_parser,
                &mut child_parser_errors,
                letter_char,
                next_char,
                last_char,
            );

            for i in child_parser_errors {
                errors.push(i);
            }
            classdata.code = child_parser;
        }
    }
}
