use crate::alloc::borrow::ToOwned;
use crate::alloc::string::ToString;
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::processors;
use crate::syntax;
use ellie_core::{defs, error, utils};

pub fn collect_enum<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    let parser_clone = parser.clone();
    if let parser::Collecting::Enum(ref mut enum_data) = parser.current {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if !enum_data.name_collected {
            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n") || enum_data.data.name.is_empty())
            {
                if enum_data.data.name.is_empty() {
                    enum_data.data.name_pos.range_start = parser.pos;
                }
                enum_data.data.name += letter_char;
                enum_data.data.name_pos.range_end = parser.pos.clone().skip_char(1);
            } else if letter_char == "{" {
                enum_data.data.brace_start_pos.range_start = parser.pos;
                enum_data.data.brace_start_pos.range_end = parser.pos.clone().skip_char(1);
                enum_data.name_collected = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    code: error::errorList::error_s1.code,
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    message: error::errorList::error_s1.message.clone(),
                    title: error::errorList::error_s1.title.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: letter_char.to_string(),
                        }],
                    ),
                    debug_message: "28bdd8ef9ee46b592524e8a29ac4f527".to_owned(),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else {
            let last_entry = enum_data.data.items.len();

            if last_entry == 0 {
                if current_reliability.reliable {
                    enum_data.data.items.push(syntax::enum_type::EnumItem {
                        has_type: false,
                        identifier: letter_char.to_string(),
                        identifier_pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                        ..Default::default()
                    });
                } else if letter_char == "}" {
                    parser.collected.push(parser.current.clone());
                    parser.current = parser::Collecting::None;
                } else if letter_char != " " {
                    errors.push(error::Error {
                        code: error::errorList::error_s1.code,
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        message: error::errorList::error_s1.message.clone(),
                        title: error::errorList::error_s1.title.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        debug_message: "76a898403701ecfdeb379b1b20e6c2b4".to_owned(),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                }
            } else {
                let has_dedup = enum_data.clone().has_dedup();
                let entry = &mut enum_data.data.items[last_entry - 1];

                if !enum_data.identifier_collected {
                    if current_reliability.reliable
                        && ((last_char != " " && last_char != "\n") || entry.identifier.is_empty())
                    {
                        if entry.identifier.is_empty() {
                            entry.identifier_pos.range_start = parser.pos;
                        }
                        entry.identifier += letter_char;
                        entry.identifier_pos.range_end = parser.pos.clone().skip_char(1);
                    } else if letter_char == ":" {
                        entry.has_type = true;
                        enum_data.identifier_collected = true;
                    } else if letter_char == "}" {
                        if has_dedup {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "12e6fc5ff5d273fa13fecd7cf2551743".to_owned(),
                                title: error::errorList::error_s10.title.clone(),
                                code: error::errorList::error_s10.code,
                                message: error::errorList::error_s10.message.clone(),
                                builded_message: error::BuildedError::build_from_string(
                                    error::errorList::error_s10.message.clone(),
                                ),
                                pos: entry.identifier_pos,
                            });
                        }
                        parser.collected.push(parser.current.clone());
                        parser.current = parser::Collecting::None;
                    } else if letter_char == "," {
                        if has_dedup {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "6f129c7d8281a87b70de7ad59bc6cd16".to_owned(),
                                title: error::errorList::error_s10.title.clone(),
                                code: error::errorList::error_s10.code,
                                message: error::errorList::error_s10.message.clone(),
                                builded_message: error::BuildedError::build_from_string(
                                    error::errorList::error_s10.message.clone(),
                                ),
                                pos: entry.identifier_pos,
                            });
                        }
                        enum_data
                            .data
                            .items
                            .push(syntax::enum_type::EnumItem::default());
                    } else if letter_char != " " {
                        errors.push(error::Error {
                            code: error::errorList::error_s1.code,
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            message: error::errorList::error_s1.message.clone(),
                            title: error::errorList::error_s1.title.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s1.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: letter_char.to_string(),
                                }],
                            ),
                            debug_message: "64e34ca515d6f2bad51cb94f1435421c".to_owned(),
                            pos: defs::Cursor {
                                range_start: parser.pos,
                                range_end: parser.pos.clone().skip_char(1),
                            },
                        });
                    }
                } else {
                    if entry.enum_type.is_definer_complete() && letter_char == "," {
                        if has_dedup {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "164829a19a4ded86a9def4dfcaa78855".to_owned(),
                                title: error::errorList::error_s10.title.clone(),
                                code: error::errorList::error_s10.code,
                                message: error::errorList::error_s10.message.clone(),
                                builded_message: error::BuildedError::build_from_string(
                                    error::errorList::error_s10.message.clone(),
                                ),
                                pos: entry.identifier_pos,
                            });
                        }
                        enum_data
                            .data
                            .items
                            .push(syntax::enum_type::EnumItem::default());
                    } else if entry.enum_type.is_definer_complete() && letter_char == "}" {
                        if has_dedup {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "af26c33a95b406bca24aae3e0ad8b4ad".to_owned(),
                                title: error::errorList::error_s10.title.clone(),
                                code: error::errorList::error_s10.code,
                                message: error::errorList::error_s10.message.clone(),
                                builded_message: error::BuildedError::build_from_string(
                                    error::errorList::error_s10.message.clone(),
                                ),
                                pos: entry.identifier_pos,
                            });
                        }
                        parser.collected.push(parser.current.clone());
                        parser.current = parser::Collecting::None;
                    } else {
                        processors::definer_processor::collect_definer(
                            parser_clone,
                            &mut entry.enum_type,
                            errors,
                            letter_char.to_string(),
                            next_char,
                            last_char,
                        );
                    }
                }
            }
        }
    }
}
