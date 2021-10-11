use crate::alloc::borrow::ToOwned;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::syntax;
use crate::syntax::definers::{DefinerCollecting, GenericType};
use alloc::boxed::Box;
use ellie_core::{defs, error, utils};

pub fn collect_definer<F>(
    parser: parser::Parser<F>,
    type_data: &mut DefinerCollecting,
    errors: &mut Vec<error::Error>,
    letter_char: String,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    match type_data {
        DefinerCollecting::GrowableArray(ref mut data) => {
            if letter_char == "(" && !data.bracket_inserted {
                data.bracket_inserted = true;
            } else if letter_char == ")" && data.rtype.is_definer_complete() {
                data.complete = true;
            } else {
                collect_definer(
                    parser,
                    &mut data.rtype,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                )
            }
        }
        DefinerCollecting::Future(ref mut data) => {
            if letter_char == ")" && data.value.is_definer_complete() {
                data.complete = true;
            } else {
                collect_definer(
                    parser,
                    &mut data.value,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                )
            }
        }
        DefinerCollecting::Nullable(ref mut data) => collect_definer(
            parser,
            &mut data.value,
            errors,
            letter_char,
            next_char,
            last_char,
        ),
        DefinerCollecting::Array(ref mut data) => {
            if !data.typed {
                if letter_char == "(" && !data.bracket_inserted {
                    data.bracket_inserted = true;
                } else if letter_char == "," && data.rtype.is_definer_complete() {
                    data.typed = true;
                } else {
                    collect_definer(
                        parser,
                        &mut data.rtype,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                }
            } else if letter_char == ")" && data.len.complete {
                data.complete = true;
            } else {
                let mut emulated_collector_data = syntax::variable::VariableCollector {
                    data: syntax::variable::Variable {
                        value: syntax::types::Types::Integer(data.len.clone()),
                        rtype: syntax::definers::DefinerCollecting::Generic(
                            syntax::definers::GenericType {
                                rtype: "int".to_owned(),
                            },
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                crate::processors::value_processor::collect_value(
                    parser.clone(),
                    &mut emulated_collector_data,
                    errors,
                    &letter_char,
                    next_char,
                    last_char,
                );

                if !emulated_collector_data.data.value.is_integer() && letter_char != " " {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "definer_processor".to_owned(),
                        debug_message: "1988bcd93b28a6545db61f03cdeae024".to_owned(),
                        title: error::errorList::error_s20.title.clone(),
                        code: error::errorList::error_s20.code,
                        message: error::errorList::error_s20.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s20.message.clone(),
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                }

                if emulated_collector_data.data.value.is_type_complete() {
                    data.complete = true;
                }

                if let syntax::types::Types::Integer(e) = emulated_collector_data.data.value {
                    data.len = e;
                }
            }
        }
        DefinerCollecting::Generic(data) => {
            if letter_char == "(" && data.rtype.trim() == "fn" {
                *type_data = DefinerCollecting::Function(syntax::definers::FunctionType {
                    bracket_inserted: true,
                    returning: alloc::boxed::Box::new(DefinerCollecting::Generic(GenericType {
                        rtype: "void".to_owned(),
                    })),
                    ..Default::default()
                });
            } else if letter_char == "(" && data.rtype == "array" {
                *type_data = DefinerCollecting::Array(syntax::definers::ArrayType {
                    bracket_inserted: true,
                    ..Default::default()
                });
            } else if letter_char == "(" && data.rtype == "future" {
                *type_data = DefinerCollecting::Future(syntax::definers::FutureType {
                    brace_started: true,
                    ..Default::default()
                });
            } else if letter_char == "(" && data.rtype == "cloak" {
                *type_data = DefinerCollecting::Cloak(syntax::definers::CloakType {
                    bracket_inserted: true,
                    rtype: vec![DefinerCollecting::Generic(
                        syntax::definers::GenericType::default(),
                    )],
                    ..Default::default()
                });
            } else if letter_char == "_" && data.rtype == "" {
                *type_data = DefinerCollecting::Nullable(syntax::definers::NullableType::default())
            } else if letter_char == "(" && data.rtype == "collective" {
                *type_data =
                    DefinerCollecting::Collective(syntax::definers::CollectiveType::default());
            } else if letter_char == "(" && data.rtype == "growableArray" {
                *type_data =
                    DefinerCollecting::GrowableArray(syntax::definers::GrowableArrayType {
                        bracket_inserted: true,
                        ..Default::default()
                    });
            } else if (data.rtype.clone() + &letter_char) == "dy" {
                *type_data = DefinerCollecting::Dynamic
            } else if letter_char != " "
                && (last_char == " " || last_char == "\n")
                && data.rtype.trim() != ""
            {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "definer_processor".to_owned(),
                    debug_message: "3a170a2846902e2e75c029cfb7a002d6".to_owned(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: letter_char,
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            } else {
                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );
                if current_reliability.reliable {
                    data.rtype += &letter_char;
                    data.rtype = utils::trim_good(data.rtype.trim().to_string());
                } else if letter_char != " " {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "definer_processor".to_owned(),
                        debug_message: "03e19af0b1bfa59ae657dce8956187ba".to_owned(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: letter_char,
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                }
            }
        }
        DefinerCollecting::Function(data) => {
            if !data.parameter_collected {
                if letter_char == "(" && !data.bracket_inserted {
                    data.bracket_inserted = true;
                } else if letter_char == ")" && data.bracket_inserted {
                    data.parameter_collected = true;
                } else if letter_char == "," && !data.params.is_empty() && !data.at_comma {
                    data.params.push(DefinerCollecting::Generic(
                        syntax::definers::GenericType::default(),
                    ));
                    data.at_comma = true;
                } else if data.params.is_empty() && data.bracket_inserted {
                    //This should have been filled If everything were right
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "definer_processor".to_owned(),
                        debug_message: "7ed204706ec9e038b8cc96cb96b40138".to_owned(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: letter_char,
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else if data.bracket_inserted {
                    data.at_comma = false;
                    if data.params.clone().len() == 0 {
                        data.params.push(DefinerCollecting::Generic(
                            syntax::definers::GenericType::default(),
                        ));
                    }
                    let len = data.params.clone().len();
                    collect_definer(
                        parser,
                        &mut data.params[if len == 0 { 0 } else { len - 1 }],
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    );

                    if data.params[if len == 0 { 0 } else { len - 1 }].is_definer_complete() {
                        data.complete = true;
                    }
                }
            } else if !data.return_typed {
                if data.return_keyword != 2 {
                    if letter_char != ":" {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: "definer_processor".to_owned(),
                            debug_message: "48fe22c8c92da9401ea1a5f11396d79d".to_owned(),
                            title: error::errorList::error_s1.title.clone(),
                            code: error::errorList::error_s1.code,
                            message: error::errorList::error_s1.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s1.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: letter_char,
                                }],
                            ),
                            pos: defs::Cursor {
                                range_start: parser.pos,
                                range_end: parser.pos.clone().skip_char(1),
                            },
                        });
                    }
                    data.return_keyword += 1;
                    data.returning = Box::new(DefinerCollecting::Generic(GenericType::default()));
                } else {
                    data.complete = true;
                    collect_definer(
                        parser,
                        &mut data.returning,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                }
            }
        }
        DefinerCollecting::Cloak(data) => {
            let length_of_childs = data.rtype.len();
            let is_complete = if length_of_childs == 0 {
                false
            } else {
                data.rtype[if length_of_childs == 1 {
                    0
                } else {
                    length_of_childs - 1
                }]
                .is_definer_complete()
            };

            if letter_char == "," && is_complete {
                data.rtype.push(DefinerCollecting::Generic(
                    syntax::definers::GenericType::default(),
                ));
            } else if letter_char == ")" && is_complete {
                data.complete = true;
            } else {
                collect_definer(
                    parser,
                    &mut data.rtype[if length_of_childs == 1 {
                        0
                    } else {
                        length_of_childs - 1
                    }],
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                )
            }
        }
        DefinerCollecting::Collective(data) => {
            if !data.has_key {
                if letter_char == "," && data.key.is_definer_complete() {
                    data.has_key = true;
                } else {
                    collect_definer(
                        parser,
                        &mut data.key,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                }
            } else {
                if letter_char == ")" && data.value.is_definer_complete() {
                    data.complete = true;
                } else {
                    collect_definer(
                        parser,
                        &mut data.value,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                }
            }
        }
        DefinerCollecting::Dynamic => {}
    }
}
