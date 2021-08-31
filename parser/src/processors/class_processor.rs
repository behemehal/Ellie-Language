/*

    DEPRECATED

*/

use crate::syntax::class;
use crate::{parser, parser::Collecting, syntax, syntax::import_item};

use ellie_core::{defs, error, utils};

use crate::alloc::string::{String, ToString};

use crate::alloc::boxed::Box;
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn collect_class<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
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
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "b33c40fb1f6ae4432f337dd4f0dc9179".to_string(),
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
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "e4b63a35b27453778bb0fc44fa9403f1".to_string(),
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
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "b5a2ec8c7a226bd97600b4965375fae0".to_string(),
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
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "a1cfaf7dd22b04c4fa62b2034f2b698d".to_string(),
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
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "c5aae4d50849cd914cdc6c17d376d745".to_string(),
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
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "51fdda34a826a52708a8d2cc5b80d92d".to_string(),
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
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "aeac9a03307346a2141d1e6f1b960d35".to_string(),
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
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "64b747d23bb15ea8c411fdd1504ecea8".to_string(),
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
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "007eb0f5fb419a562c27c9d7faf10102".to_string(),
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
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "1ca09346cd8fff8fc4002bc52bfd8baf".to_string(),
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
                    path: parser.options.path.clone(),
                    scope: "definer_processor".to_string(),
                    debug_message: "577421d55b3e9f9fe9c956f73b6a371d".to_string(),
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
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "43789e1fd1abc940f768cf6aa57cefe2".to_string(),
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
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "fa6f67881fcff5b64cf892fdeb4aa4fd".to_string(),
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
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "cb2c4a647b3725afd4e0c0e46465cd1d".to_string(),
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

            if class_data.code.pos.is_zero() {
                //Make sure upper scope imported once

                for item in parser.collected.clone() {
                    //Import variables as temporary for syntax support, we will remove them after collecting complete
                    child_parser.collected.push(parser::Collecting::ImportItem(
                        import_item::ImportItem {
                            from_path: "<temporary>".to_string(),
                            public: true,
                            item: Box::new(item),
                        },
                    ));
                }
            }

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
