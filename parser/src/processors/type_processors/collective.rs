use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::processors::value_processor;
use crate::syntax;
use crate::syntax::types::collective_type;
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use ellie_core::{defs, error};

pub fn collect_collective<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
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
                        debug_message: "8004916a36ba3b6d2b65594c9839a590".to_owned(),
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
                collective_data.complete = true;
                collective_data.data.entries = vec![];

                if itered_data.data.dynamic {
                    let mut keys = collective_data
                        .data
                        .entries
                        .clone()
                        .into_iter()
                        .map(|x| x.data.key.to_definer())
                        .collect::<Vec<_>>();
                    keys.dedup();
                    let mut values = collective_data
                        .data
                        .entries
                        .clone()
                        .into_iter()
                        .map(|x| x.data.value.to_definer())
                        .collect::<Vec<_>>();
                    values.dedup();

                    let key_type = if keys.len() > 1 || keys.len() == 0 {
                        crate::syntax::definers::DefinerCollecting::Dynamic
                    } else {
                        keys[0].clone()
                    };

                    let value_type = if values.len() > 1 || values.len() == 0 {
                        crate::syntax::definers::DefinerCollecting::Dynamic
                    } else {
                        values[0].clone()
                    };
                    itered_data.data.rtype = crate::syntax::definers::DefinerCollecting::Collective(
                        crate::syntax::definers::CollectiveType {
                            key: Box::new(key_type),
                            value: Box::new(value_type),
                            ..Default::default()
                        },
                    );
                }
            } else if letter_char == ":" && last_entry.data.key.is_type_complete() {
                //If current char is splitter and collected key is complete

                last_entry.key_collected = true;
                if !itered_data.data.dynamic {
                    if let definers::DefinerCollecting::Collective(collective_defining) =
                        itered_data.data.rtype.clone()
                    {
                        let entry_type_option =
                            parser.resolve_variable(*last_entry.data.key.clone(), false);

                        if let Ok(entry_type) = entry_type_option {
                            if *collective_defining.key != entry_type
                                && !collective_defining.value.is_dynamic()
                            {
                                errors.push(error::Error {
                                    path: parser.options.path.clone(),
                                    scope: parser.scope.scope_name.clone(),
                                    debug_message: "9671e41150005198aeb3715e079e4bfb".to_owned(),
                                    title: error::errorList::error_s3.title.clone(),
                                    code: error::errorList::error_s3.code,
                                    message: error::errorList::error_s3.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s3.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token1".to_owned(),
                                                value: collective_defining.key.raw_name(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_owned(),
                                                value: entry_type.raw_name_with_extensions(),
                                            },
                                        ],
                                    ),
                                    pos: last_entry.data.key_pos,
                                });
                            }
                        } else {
                            panic!("Unexpected parser error");
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
                        debug_message: "402262d7843320e1f89155b277683478".to_owned(),
                        title: error::errorList::error_s36.title.clone(),
                        code: error::errorList::error_s36.code,
                        message: error::errorList::error_s36.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s36.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
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

                value_processor::collect_value(
                    clone_parser,
                    &mut will_be_itered,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                );
                last_entry.data.key = Box::new(will_be_itered.data.value);
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
                        debug_message: "f0d1296ee8ebf00bba5cb6b19bdfe955".to_owned(),
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
                        let entry_type_option = parser.resolve_variable(
                            *collective_data.data.entries[last_entry_ind - 1]
                                .data
                                .value
                                .clone(),
                            false,
                        );

                        if let Ok(entry_type) = entry_type_option {
                            if *collective_defining.value != entry_type
                                && !collective_defining.value.is_dynamic()
                            {
                                errors.push(error::Error {
                                    path: parser.options.path.clone(),
                                    scope: parser.scope.scope_name.clone(),
                                    debug_message: "1a6fd33f1c446310a1ba1287c9197d9a".to_owned(),
                                    title: error::errorList::error_s3.title.clone(),
                                    code: error::errorList::error_s3.code,
                                    message: error::errorList::error_s3.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s3.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token1".to_owned(),
                                                value: collective_defining.value.raw_name(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_owned(),
                                                value: entry_type.raw_name_with_extensions(),
                                            },
                                        ],
                                    ),
                                    pos: collective_data.data.entries[last_entry_ind - 1]
                                        .data
                                        .value_pos,
                                });
                            }
                        } else {
                            panic!("Unexpected parser error");
                        }

                        if itered_data.data.dynamic {
                            let mut keys = collective_data
                                .data
                                .entries
                                .clone()
                                .into_iter()
                                .map(|x| x.data.key.to_definer())
                                .collect::<Vec<_>>();
                            keys.dedup();
                            let mut values = collective_data
                                .data
                                .entries
                                .clone()
                                .into_iter()
                                .map(|x| x.data.value.to_definer())
                                .collect::<Vec<_>>();
                            values.dedup();

                            let key_type = if keys.len() > 1 || keys.len() == 0 {
                                crate::syntax::definers::DefinerCollecting::Dynamic
                            } else {
                                keys[0].clone()
                            };

                            let value_type = if values.len() > 1 || values.len() == 0 {
                                crate::syntax::definers::DefinerCollecting::Dynamic
                            } else {
                                values[0].clone()
                            };
                            itered_data.data.rtype =
                                crate::syntax::definers::DefinerCollecting::Collective(
                                    crate::syntax::definers::CollectiveType {
                                        key: Box::new(key_type),
                                        value: Box::new(value_type),
                                        ..Default::default()
                                    },
                                );
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
                            debug_message: "858330d352cf3b994a91d8e6736252a5".to_owned(),
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
                    collective_data.complete = true;

                    if itered_data.data.dynamic {
                        let mut keys = collective_data
                            .data
                            .entries
                            .clone()
                            .into_iter()
                            .map(|x| x.data.key.to_definer())
                            .collect::<Vec<_>>();
                        keys.dedup();
                        let mut values = collective_data
                            .data
                            .entries
                            .clone()
                            .into_iter()
                            .map(|x| x.data.value.to_definer())
                            .collect::<Vec<_>>();
                        values.dedup();

                        let key_type = if keys.len() > 1 || keys.len() == 0 {
                            crate::syntax::definers::DefinerCollecting::Dynamic
                        } else {
                            keys[0].clone()
                        };

                        let value_type = if values.len() > 1 || values.len() == 0 {
                            crate::syntax::definers::DefinerCollecting::Dynamic
                        } else {
                            values[0].clone()
                        };
                        itered_data.data.rtype =
                            crate::syntax::definers::DefinerCollecting::Collective(
                                crate::syntax::definers::CollectiveType {
                                    key: Box::new(key_type),
                                    value: Box::new(value_type),
                                    ..Default::default()
                                },
                            );
                    }
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

                value_processor::collect_value(
                    clone_parser.clone(),
                    &mut will_be_itered,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                );

                last_entry.data.value = Box::new(will_be_itered.data.value);
                last_entry.data.value_pos.range_end = clone_parser.pos.clone().skip_char(1);
                //Set the range end
            }
        }
    }
}
