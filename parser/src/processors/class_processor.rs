use crate::syntax::class;
use crate::{parser, parser::Collecting, syntax, syntax::import_item};
use crate::alloc::borrow::ToOwned;
use crate::alloc::string::{String, ToString};
use ellie_core::{defs, error, utils};

use crate::alloc::boxed::Box;
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn collect_class<F, E>(
    parser: &mut parser::Parser<F, E>,
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
                        debug_message: "7a2c31ce1e4b5d402eafbd789359a47e".to_owned(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
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
                        debug_message: "00f9f6893d8a669983c67d9ccce15e91".to_owned(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
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
                    debug_message: "c57ccf1f7acecb2b8ccf59f2ca055622".to_owned(),
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
                        debug_message: "a5dca62c42196d80c17f4af21d82f065".to_owned(),
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
                        debug_message: "cea7a601ccf937677227ee1ea6fff930".to_owned(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
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
                        debug_message: "dff82c4c4ae751a6a7ed95b4c14215c5".to_owned(),
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
                        debug_message: "6204b46372ab949f0f0a4185255b278b".to_owned(),
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
                        debug_message: "dafe28b11b9da8b5bafbecbf5475c855".to_owned(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
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
                    debug_message: "9a4ba246476bac9f5779c4c01bd840fb".to_owned(),
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
        } else if !class_data.has_code {
            if letter_char == "{" {
                class_data.has_code = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "c7a0619008f86492c42a839252881527".to_owned(),
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
        } else if class_data.brace_count == 0 && letter_char == "}" {
            if class_data.code.current != Collecting::None
                || !class_data.code.keyword_catch.trim().is_empty()
            {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "definer_processor".to_owned(),
                    debug_message: "dd9efd8b8c289d9ef339cd48658bf5da".to_owned(),
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
                    parser::Collecting::Getter(e) => {
                        class_data.data.getters.push(e.data);
                    }
                    parser::Collecting::Setter(e) => {
                        class_data.data.setters.push(e.data);
                    }
                    parser::Collecting::Constructor(e) => {
                        if e.data.name != class_data.data.name {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "2853f76b1e346de805ed5a2550f7c1b8".to_owned(),
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
                                debug_message: "149f8b56b03a989b76599e67e0ddfc09".to_owned(),
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
                            debug_message: "a473f8f3341fad952044872f6a0ddc27".to_owned(),
                            title: error::errorList::error_s4.title.clone(),
                            code: error::errorList::error_s4.code,
                            message: error::errorList::error_s4.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s4.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
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
                    if let parser::Collecting::ImportItem(import) = item {
                        child_parser.collected.push(parser::Collecting::ImportItem(
                            import_item::ImportItem {
                                resolution_id: 0,
                                from_import: 0,
                                from_path: "<temporary>".to_owned(),
                                public: true,
                                item: import.item,
                            },
                        ));
                    } else {
                        child_parser.collected.push(parser::Collecting::ImportItem(
                            import_item::ImportItem {
                                resolution_id: 0,
                                from_import: 0,
                                from_path: "<temporary>".to_owned(),
                                public: true,
                                item: Box::new(item),
                            },
                        ));
                    }
                }
            }

            child_parser.options = parser.options.clone();
            child_parser.options.parser_type = defs::ParserType::ClassParser;
            child_parser.generic_variables = class_data.data.generic_definings.clone();
            child_parser.pos = parser.pos;
            child_parser.scope.scope_name = "core/class_processor".to_owned();
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
    } else {
        panic!("Unexpected parser behaviour")
    }
}
