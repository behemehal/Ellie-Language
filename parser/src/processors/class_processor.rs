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
    if let parser::Collecting::Class(ref mut class_data) = parser.current {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if !class_data.name_collected {
            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n") || class_data.data.name.is_empty())
            {
                if class_data.data.name.is_empty() {
                    class_data.data.name_pos.range_start = parser.pos;
                }
                class_data.data.name += letter_char;
                class_data.data.name_pos.range_end = parser.pos.clone().skip_char(1);
            } else if letter_char == "<" && !class_data.data.name.is_empty() {
                if utils::is_reserved(&class_data.data.name) {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "a7745abc60ea5de65798c453bd7981b6".to_string(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: class_data.data.name.clone(),
                            }],
                        ),
                        pos: class_data.data.name_pos,
                    });
                }
                class_data.name_collected = true;
            } else if letter_char == "{" && !class_data.data.name.is_empty() {
                if utils::is_reserved(&class_data.data.name) {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "a7aacd2379316d9779f410b66533a48b".to_string(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: class_data.data.name.clone(),
                            }],
                        ),
                        pos: class_data.data.name_pos,
                    });
                }
                class_data.name_collected = true;
                class_data.has_code = true;
                class_data.generic_definings_collected = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "748e6fd160c5067911fb2768aca5b773".to_string(),
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
        } else if !class_data.generic_definings_collected {
            let mut last_entry = class_data.data.generic_definings.len();

            if last_entry == 0 && current_reliability.reliable {
                //...reliable will make sure in case of no parameter used no parameter data will be applied
                class_data
                    .data
                    .generic_definings
                    .push(class::GenericDefining::default());
                last_entry = 1;
            }

            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n")
                    || class_data.data.generic_definings[last_entry - 1]
                        .name
                        .is_empty())
            {
                if class_data.data.generic_definings[last_entry - 1]
                    .name
                    .is_empty()
                {
                    class_data.data.generic_definings[last_entry - 1]
                        .pos
                        .range_start = parser.pos;
                }
                class_data.at_comma = false;
                class_data.data.generic_definings[last_entry - 1]
                    .pos
                    .range_end = parser.pos.clone().skip_char(1);
                class_data.data.generic_definings[last_entry - 1].name += letter_char;
            } else if letter_char == ">" && !class_data.at_comma {
                if class_data.has_dedup() {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "0a68ad4b58a1b02edd076cedd2947173".to_string(),
                        title: error::errorList::error_s10.title.clone(),
                        code: error::errorList::error_s10.code,
                        message: error::errorList::error_s10.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s10.message.clone(),
                        ),
                        pos: class_data.data.generic_definings[last_entry - 1].pos,
                    });
                }
                class_data.generic_definings_collected = true;
            } else if letter_char == "," && !class_data.at_comma {
                if class_data.has_dedup() {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "cf867d9fd252079128e7d2af27cb2498".to_string(),
                        title: error::errorList::error_s10.title.clone(),
                        code: error::errorList::error_s10.code,
                        message: error::errorList::error_s10.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s10.message.clone(),
                        ),
                        pos: class_data.data.generic_definings[last_entry - 1].pos,
                    });
                }
                class_data.at_comma = true;
                class_data
                    .data
                    .generic_definings
                    .push(class::GenericDefining::default());
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "1ae861ca61816896838e0d9bf038aaa5".to_string(),
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
        } else if !class_data.has_code {
            if letter_char == "{" {
                class_data.has_code = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "b13efbf02f1e3af7e3dec3a9ba856c74".to_string(),
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
        } else if class_data.brace_count == 0 && letter_char == "}" {
            for i in class_data.code.collected.clone() {
                match i {
                    parser::Collecting::Variable(e) => {
                        class_data.data.properties.push(e.data);
                    }
                    parser::Collecting::Function(e) => {
                        class_data.data.methods.push(e.data);
                    }
                    parser::Collecting::Constructor(e) => {
                        if e.data.name != class_data.data.name {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "4a987c2a232fed66abd089efc0f454f3".to_string(),
                                title: error::errorList::error_s22.title.clone(),
                                code: error::errorList::error_s22.code,
                                message: error::errorList::error_s22.message.clone(),
                                builded_message: error::BuildedError::build_from_string(
                                    error::errorList::error_s22.message.clone(),
                                ),
                                pos: e.data.name_pos,
                            });
                        }
                        class_data.data.constructor = e.data;
                    }
                    _ => {}
                };
            }
            class_data.code = Box::new(parser::RawParser::default()); //Empty the cache
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if letter_char == "{" {
                class_data.brace_count += 1;
            } else if letter_char == "}" && class_data.brace_count != 0 {
                class_data.brace_count -= 1;
            }
            let mut child_parser = class_data.code.clone().to_no_resolver_parser();
            child_parser.collected = parser
                .collected
                .clone()
                .into_iter()
                .filter(|x| {
                    if let parser::Collecting::ImportItem(_) = x {
                        true
                    } else {
                        false
                    }
                })
                .collect();
            child_parser.options = parser.options.clone();
            child_parser.options.parser_type = defs::ParserType::ClassParser;
            child_parser.generic_variables = class_data.data.generic_definings.clone();
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
            class_data.code = Box::new(child_parser.to_raw());
        }
    }
}
