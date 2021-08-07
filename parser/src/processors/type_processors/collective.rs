use crate::parser;
use crate::processors::value_processor;
use crate::syntax;
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::syntax::types::collective_type;

use ellie_core::error;

pub fn collect_collective(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    let clone_parser = parser.clone();
    if let types::Types::Collective(ref mut collective_data) = itered_data.data.value {
        let mut last_entry_ind = collective_data.data.entries.len(); //Get the last entry

        if last_entry_ind == 0 {
            //If there is no entry available, create a entry
            collective_data
                .data
                .entries
                .push(collective_type::CollectiveEntryCollector::default());
            last_entry_ind = 1;
        }

        if itered_data.data.dynamic {
            itered_data.data.rtype =
                definers::DefinerCollecting::Collective(definers::CollectiveType {
                    complete: true,
                    key: Box::new(definers::DefinerCollecting::Dynamic),
                    value: Box::new(definers::DefinerCollecting::Dynamic),
                    ..Default::default()
                })
        }

        let has_dedup = collective_data.clone().has_dedup();
        let ref mut last_entry = collective_data.data.entries[last_entry_ind - 1];

        if !last_entry.key_collected {
            //If last entry's key is not yet collected

            if letter_char != " " && last_entry.data.key_pos.range_start.is_zero() {
                //If current char is not empty and range_start position is not yet initialized
                last_entry.data.key_pos.range_start = clone_parser.pos.clone();
            }
            last_entry.data.key_pos.range_end = clone_parser.pos.clone(); //Set the range end

            if letter_char == "}" && last_entry.data.key.get_type() == "null" {
                collective_data.complete = true;
                collective_data.data.entries = vec![];
            } else if letter_char == ":" && last_entry.data.key.is_type_complete() {
                //If current char is splitter and collected key is complete

                last_entry.key_collected = true;
                if !itered_data.data.dynamic {
                    if let definers::DefinerCollecting::Collective(collective_defining) =
                        itered_data.data.rtype.clone()
                    {
                        let entry_type = parser.resolve_variable(
                            *collective_data.data.entries[last_entry_ind - 1]
                                .data
                                .key
                                .clone(),
                        );

                        if collective_defining.key.raw_name() != entry_type
                            && collective_defining.value.raw_name() != "dyn"
                        {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "f7a93a3e00887ce225e3767fa789101b".to_string(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: collective_defining.key.raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: entry_type,
                                        },
                                    ],
                                ),
                                pos: collective_data.data.entries[last_entry_ind - 1]
                                    .data
                                    .key_pos,
                            });
                        }
                    }
                }
            } else {
                let mut will_be_itered = syntax::variable::VariableCollector {
                    data: syntax::variable::Variable {
                        value: *last_entry.data.key.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                let itered_key_vector = Box::new(value_processor::collect_value(
                    clone_parser,
                    &mut will_be_itered,
                    letter_char,
                    next_char,
                    last_char,
                ));

                if !itered_key_vector.errors.is_empty() {
                    errors.extend(itered_key_vector.errors);
                }

                last_entry.data.key = Box::new(itered_key_vector.itered_data.data.value);
            }
        } else {
            //Collecting last entry's value
            if letter_char != " " && last_entry.data.value_pos.range_start.is_zero() {
                //If current char is not empty and range_start position is not yet initialized
                last_entry.data.value_pos.range_start = clone_parser.pos.clone();
            }

            if (letter_char == "," || letter_char == "}")
                && last_entry.data.value.is_type_complete()
            {
                //If current char is a comma and collected value is complete

                if has_dedup {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "06127ffc30f40b277155790eb5abd7ce".to_string(),
                        title: error::errorList::error_s10.title.clone(),
                        code: error::errorList::error_s10.code,
                        message: error::errorList::error_s10.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s10.message.clone(),
                        ),
                        pos: last_entry.data.key_pos,
                    });
                }

                if !itered_data.data.dynamic {
                    if let definers::DefinerCollecting::Collective(collective_defining) =
                        itered_data.data.rtype.clone()
                    {
                        let entry_type = parser.resolve_variable(
                            *collective_data.data.entries[last_entry_ind - 1]
                                .data
                                .value
                                .clone(),
                        );

                        if collective_defining.value.raw_name() != entry_type
                            && collective_defining.value.raw_name() != "dyn"
                        {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "dbbb5353ec2862251e4364d260bb4abb".to_string(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: collective_defining.value.raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: entry_type,
                                        },
                                    ],
                                ),
                                pos: collective_data.data.entries[last_entry_ind - 1]
                                    .data
                                    .value_pos,
                            });
                        }
                    }
                }

                if letter_char == "," {
                    collective_data
                        .data
                        .entries
                        .push(collective_type::CollectiveEntryCollector::default());
                } else if letter_char == "}" {
                    collective_data.complete = true;
                }
            } else {
                let mut will_be_itered = if itered_data.data.dynamic {
                    syntax::variable::VariableCollector {
                        data: syntax::variable::Variable {
                            value: *last_entry.data.value.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                } else {
                    if let definers::DefinerCollecting::Collective(q) =
                        itered_data.data.rtype.clone()
                    {
                        syntax::variable::VariableCollector {
                            data: syntax::variable::Variable {
                                value: *last_entry.data.value.clone(),
                                rtype: *q.value,
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    } else {
                        syntax::variable::VariableCollector {
                            data: syntax::variable::Variable {
                                value: *last_entry.data.value.clone(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    }
                };

                let itered_key_vector = Box::new(value_processor::collect_value(
                    clone_parser.clone(),
                    &mut will_be_itered,
                    letter_char,
                    next_char,
                    last_char,
                ));

                if !itered_key_vector.errors.is_empty() {
                    errors.extend(itered_key_vector.errors);
                }

                last_entry.data.value = Box::new(itered_key_vector.itered_data.data.value);
                last_entry.data.value_pos.range_end = clone_parser.pos.clone().skip_char(1);
                //Set the range end
            }
        }
    }
}

/*
errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "5994a97417a82577312b4cbcfb20b396".to_string(),
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
*/
