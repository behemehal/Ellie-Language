use crate::parser;
use crate::processors::value_processor;
use crate::syntax;
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::syntax::types::collective_type;

use ellie_core::{defs, error};

pub fn collect_collective<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
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

        let collective_data_clone = collective_data.clone();
        let has_dedup = collective_data.clone().has_dedup();
        let ref mut last_entry = collective_data.data.entries[last_entry_ind - 1];

        if !last_entry.key_collected {
            //If last entry's key is not yet collected

            collective_data.at_comma = false;
            if letter_char != " " && last_entry.data.key_pos.range_start.is_zero() {
                //If current char is not empty and range_start position is not yet initialized
                last_entry.data.key_pos.range_start = clone_parser.pos.clone();
            }
            last_entry.data.key_pos.range_end = clone_parser.pos.clone(); //Set the range end

            if letter_char == "}" && last_entry.data.key.get_type() == "null" {
                if collective_data.at_comma {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "ad09d7bf3aed2011472bd31833a1ec53".to_string(),
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
                collective_data.complete = true;
                collective_data.data.entries = vec![];
            } else if letter_char == ":" && last_entry.data.key.is_type_complete() {
                //If current char is splitter and collected key is complete

                last_entry.key_collected = true;
                if !itered_data.data.dynamic {
                    if let definers::DefinerCollecting::Collective(collective_defining) =
                        itered_data.data.rtype.clone()
                    {
                        let entry_type = parser.resolve_variable(*last_entry.data.key.clone());

                        if collective_defining.key.raw_name() != entry_type
                            && !collective_defining.value.is_dynamic()
                        {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "60b7deece2a66268a17ed6421e1703b6".to_string(),
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
                                pos: last_entry.data.key_pos,
                            });
                        }
                    }
                }

                if &*last_entry.data.key.get_type() != "string"
                    && &*last_entry.data.key.get_type() != "char"
                    && &*last_entry.data.key.get_type() != "int"
                {
                    #[cfg(feature = "std")]
                    std::println!("\u{001b}[31m[ParserError]\u{001b}[0m: Not all types supported as collective key. Only strings are allowed for now");
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "858fbd233de885db5bb322557a0b1fe0".to_string(),
                        title: error::errorList::error_s36.title.clone(),
                        code: error::errorList::error_s36.code,
                        message: error::errorList::error_s36.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s36.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: (*last_entry.data.key.get_type().clone()).to_string(),
                            }],
                        ),
                        pos: collective_data.data.entries[last_entry_ind - 1]
                            .data
                            .key_pos,
                    });
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
                std::println!("{:#?}", collective_data_clone);
                if has_dedup {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "89e22ee6936eb0a29c51220810674456".to_string(),
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
                                debug_message: "04003377c30416fa0e8f497b38b18734".to_string(),
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
                    collective_data.at_comma = true;
                } else if letter_char == "}" {
                    if collective_data.at_comma {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "ea4e65f3463cdb6b1426c66dc45bfe8c".to_string(),
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
