/*

    DEPRECATED

*/

use crate::syntax::class;
use crate::{parser, parser::Collecting, syntax};

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
                        debug_message: "29119e52c0325ad36726b9abf00d54ac".to_string(),
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
                        debug_message: "ae7cfb1b034bf4ec642acb9dd0489c01".to_string(),
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
                    debug_message: "2c604165ab27aef1c1382ccc66ebb014".to_string(),
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
                        debug_message: "daa5d667a759f4c544923dbaf5a7cb1e".to_string(),
                        title: error::errorList::error_s10.title.clone(),
                        code: error::errorList::error_s10.code,
                        message: error::errorList::error_s10.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s10.message.clone(),
                        ),
                        pos: class_data.data.generic_definings[last_entry - 1].pos,
                    });
                }
                if class_data.data.generic_definings.len() > 0
                    && utils::is_reserved(&class_data.data.generic_definings[last_entry - 1].name)
                {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "6970f9acb422aac820d5d0e308a6f78a".to_string(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: class_data.data.generic_definings[last_entry - 1]
                                    .name
                                    .clone(),
                            }],
                        ),
                        pos: class_data.data.generic_definings[last_entry - 1].pos,
                    });
                }
                if class_data.data.generic_definings.len() == 0 {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "b7628fd1fa92e7b4ab90f0806d9829fb".to_string(),
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
                            range_start: parser.pos.clone(),
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                }
                class_data.generic_definings_collected = true;
            } else if letter_char == "," && !class_data.at_comma {
                if class_data.has_dedup() {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "4376cd3667571f4d72fcfdabd04c73b3".to_string(),
                        title: error::errorList::error_s10.title.clone(),
                        code: error::errorList::error_s10.code,
                        message: error::errorList::error_s10.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s10.message.clone(),
                        ),
                        pos: class_data.data.generic_definings[last_entry - 1].pos,
                    });
                }
                if utils::is_reserved(&class_data.data.generic_definings[last_entry - 1].name) {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "9e663b5755e4757b790b96e319944823".to_string(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: class_data.data.generic_definings[last_entry - 1]
                                    .name
                                    .clone(),
                            }],
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
                    debug_message: "e3f1836572aa7303804610284177b4af".to_string(),
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
                    debug_message: "422193dd0fc04e6491caae1557ae101f".to_string(),
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
            if class_data.code.current != Collecting::None
                || !class_data.code.keyword_catch.trim().is_empty()
            {
                errors.push(error::Error {
                    scope: "definer_processor".to_string(),
                    debug_message: "616c43b554832c1c4c6074e26a623d5f".to_string(),
                    title: error::errorList::error_s26.title.clone(),
                    code: error::errorList::error_s26.code,
                    message: error::errorList::error_s26.message.clone(),
                    builded_message: error::BuildedError::build_from_string(
                        error::errorList::error_s26.message.clone(),
                    ),
                    pos: class_data.code.keyword_pos,
                });
            }
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
                                debug_message: "b27f7fd496404e9803e44f45d4f2241c".to_string(),
                                title: error::errorList::error_s22.title.clone(),
                                code: error::errorList::error_s22.code,
                                message: error::errorList::error_s22.message.clone(),
                                builded_message: error::BuildedError::build_from_string(
                                    error::errorList::error_s22.message.clone(),
                                ),
                                pos: e.data.name_pos,
                            });
                        } else if class_data.data.constructor.name != "" {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "fbe2cb884ab6c706cd3cba4084891bab".to_string(),
                                title: error::errorList::error_s30.title.clone(),
                                code: error::errorList::error_s30.code,
                                message: error::errorList::error_s30.message.clone(),
                                builded_message: error::BuildedError::build_from_string(
                                    error::errorList::error_s30.message.clone(),
                                ),
                                pos: e.data.pos,
                            });
                        }
                        class_data.data.constructor = e.data;
                    }
                    _ => {}
                };
            }
            if class_data.data.constructor.parameters.len() > 0 {
                for element in class_data.data.constructor.parameters.clone() {
                    if class_data
                        .data
                        .properties
                        .iter()
                        .filter(|e| e.name == element.name)
                        .collect::<Vec<&syntax::variable::Variable>>()
                        .is_empty()
                    {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "f2dfdfb79f6b99aae8a0c5d7f7e947fe".to_string(),
                            title: error::errorList::error_s4.title.clone(),
                            code: error::errorList::error_s4.code,
                            message: error::errorList::error_s4.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s4.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: element.name.clone(),
                                }],
                            ),
                            pos: element.pos,
                        });
                    }
                }
            }
            class_data.code = Box::new(parser::RawParser::default()); //Empty the cache
            class_data.data.pos.range_end = parser.pos.clone().skip_char(1);
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if letter_char == "{" {
                class_data.brace_count += 1;
            } else if letter_char == "}" && class_data.brace_count != 0 {
                class_data.brace_count -= 1;
            }
            let mut child_parser = class_data.code.clone().to_no_resolver_parser();

            // Import scope's modules
            child_parser.collected = parser
                .collected
                .clone()
                .into_iter()
                .filter(|x| matches!(x, parser::Collecting::ImportItem(_)))
                .collect();

            // Import previous iteration data
            child_parser
                .collected
                .extend(class_data.code.collected.clone());
            child_parser.options = parser.options.clone();
            child_parser.options.parser_type = defs::ParserType::ClassParser;
            child_parser.generic_variables = class_data.data.generic_definings.clone();
            child_parser.pos = parser.pos;
            child_parser.scope.scope_name = "core/class_processor".to_string();
            child_parser.current = class_data.code.current.clone();
            child_parser.keyword_catch = class_data.code.keyword_catch.clone();
            child_parser.keyword_cache = class_data.code.keyword_cache.clone();

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
