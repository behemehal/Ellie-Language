use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::processors::{type_processors, value_processor};
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error};

pub fn collect_array<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    if let types::Types::Array(ref mut data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype = crate::syntax::definers::DefinerCollecting::GrowableArray(
                crate::syntax::definers::GrowableArrayType {
                    rtype: Box::new(crate::syntax::definers::DefinerCollecting::Dynamic),
                    ..Default::default()
                },
            );
        }

        let mut last_entry = data.clone().data.collective.len();

        let is_s_n = last_entry == 0
            || data.data.collective[last_entry - 1]
                .value
                .is_type_complete();

        if letter_char == "[" && !data.child_start && is_s_n {
            if !data.comma && last_entry != 0 {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "array_processor".to_owned(),
                    debug_message: "a6fe6c0bd9679430ad8112f0db692a39".to_owned(),
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
            } else {
                data.child_start = true;
                if last_entry == 0 {
                    data.data.collective.push(types::array_type::ArrayEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Array(
                            types::array_type::ArrayTypeCollector::default(),
                        )),
                        location: defs::Cursor {
                            range_start: parser.pos,
                            ..Default::default()
                        },
                    });
                } else {
                    data.data.collective[last_entry - 1] = types::array_type::ArrayEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Array(
                            types::array_type::ArrayTypeCollector::default(),
                        )),
                        location: defs::Cursor {
                            range_start: parser.pos,
                            ..Default::default()
                        },
                    };
                }
            }
        } else if letter_char == "," && !data.child_start && is_s_n {
            if data.complete {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "array_processor".to_owned(),
                    debug_message: "b896692eab597bc65c3fe22a0d5e88c6".to_owned(),
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
            } else if data.comma {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "array_processor".to_owned(),
                    debug_message: "beb8e99e2b7db373a1abc2e2f06464d6".to_owned(),
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
            } else {
                if last_entry != 0 {
                    data.data.collective[last_entry - 1].value.make_complete();
                    data.data.collective[last_entry - 1].value_complete = true;
                }

                if !itered_data.data.dynamic {
                    if let definers::DefinerCollecting::Array(array_defining) =
                        itered_data.data.rtype.clone()
                    {
                        let entry_type_option = parser
                            .resolve_variable(*data.data.collective[last_entry - 1].value.clone());

                        if let Ok(entry_type) = entry_type_option {
                            if *array_defining.rtype != entry_type {
                                errors.push(error::Error {
                                    path: parser.options.path.clone(),
                                    scope: parser.scope.scope_name.clone(),
                                    debug_message: "15b268c471e09a3cf793958a7fa35980".to_owned(),
                                    title: error::errorList::error_s3.title.clone(),
                                    code: error::errorList::error_s3.code,
                                    message: error::errorList::error_s3.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s3.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token1".to_owned(),
                                                value: array_defining.rtype.raw_name(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_owned(),
                                                value: entry_type.raw_name_with_extensions(),
                                            },
                                        ],
                                    ),
                                    pos: data.data.collective[last_entry - 1].location,
                                });
                            }
                        } else {
                            panic!("Unexpected parser error");
                        }
                    }
                } else {
                    //let mut available_types: Vec<String> = data.data.collective.into_iter().map(|x| x.value.get_type()).collect();
                    //if available_types.dedup().len() == 1 {
                    //    itered_data.data.rtype = definers::DefinerCollecting::Array(
                    //        rtype: definers::DefinerCollecting::Array()
                    //    );
                    //}
                }

                data.comma = true;
                data.data.layer_size += 1;
                data.data
                    .collective
                    .push(types::array_type::ArrayEntry::default());
            }
        } else if letter_char == "]" && !data.child_start && is_s_n {
            if data.comma {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "array_processor".to_owned(),
                    debug_message: "27fab0d8588d1244589bb005e84aaabf".to_owned(),
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
            } else if data.complete {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "array_processor".to_owned(),
                    debug_message: "2b1a3d7fe6f04c51b7271f9d8710c202".to_owned(),
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
            } else {
                if last_entry != 0 {
                    if data.data.collective[last_entry - 1].value == Box::new(types::Types::Null) {
                        data.data.collective.pop();
                    } else {
                        data.data.collective[last_entry - 1].value_complete = true;
                        data.data.collective[last_entry - 1].value.make_complete();
                    }
                }
                if !itered_data.data.dynamic {
                    if let definers::DefinerCollecting::Array(array_defining) =
                        itered_data.data.rtype.clone()
                    {
                        let entry_type_option = parser
                            .resolve_variable(*data.data.collective[last_entry - 1].value.clone());

                        if let Ok(entry_type) = entry_type_option {
                            if *array_defining.rtype != entry_type {
                                errors.push(error::Error {
                                    path: parser.options.path.clone(),
                                    scope: parser.scope.scope_name.clone(),
                                    debug_message: "abce163873d769c215f01382aef67aba".to_owned(),
                                    title: error::errorList::error_s3.title.clone(),
                                    code: error::errorList::error_s3.code,
                                    message: error::errorList::error_s3.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s3.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token1".to_owned(),
                                                value: array_defining.rtype.raw_name(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_owned(),
                                                value: entry_type.raw_name_with_extensions(),
                                            },
                                        ],
                                    ),
                                    pos: data.data.collective[last_entry - 1].location,
                                });
                            }
                        } else {
                            panic!("Unexpected parser error");
                        }
                    }
                }

                data.data.layer_size += 1;
                data.complete = true;
                itered_data.value_complete = true;
            }
        } else if data.complete && letter_char == "." && is_s_n {
            itered_data.data.value =
                types::Types::Reference(types::reference_type::ReferenceTypeCollector {
                    data: types::reference_type::ReferenceType {
                        reference_pos: itered_data.data.value_pos,
                        reference: Box::new(itered_data.data.value.clone()),
                        chain: Vec::new(),
                    },
                    root_available: false,
                    on_dot: false,
                    complete: false,
                    last_entry: itered_data.data.value.clone().to_definer(),
                });

            type_processors::reference::collect_reference(
                parser.clone(),
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
            )
        } else if data.complete
            && types::logical_type::LogicalOperators::is_logical_operator(letter_char)
            || types::logical_type::LogicalOperators::is_logical_operator(
                &(letter_char.to_string() + &next_char),
            ) && is_s_n
        {
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorTypeCollector {
                    data: types::operator_type::OperatorType {
                        first: Box::new(itered_data.data.value.clone()),
                        operator: types::operator_type::Operators::LogicalType(
                            types::logical_type::LogicalOperators::Null,
                        ),
                        ..Default::default()
                    },
                    operator_collect: letter_char.to_string(),
                    first_filled: true,
                    ..Default::default()
                });
        } else if data.complete
            && types::comparison_type::ComparisonOperators::is_comparison_operator(letter_char)
            || types::comparison_type::ComparisonOperators::is_comparison_operator(
                &(letter_char.to_string() + &next_char),
            ) && is_s_n
        {
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorTypeCollector {
                    data: types::operator_type::OperatorType {
                        first: Box::new(itered_data.data.value.clone()),
                        operator: types::operator_type::Operators::ComparisonType(
                            types::comparison_type::ComparisonOperators::Null,
                        ),
                        ..Default::default()
                    },
                    operator_collect: letter_char.to_string(),
                    first_filled: true,
                    ..Default::default()
                });
        } else if data.complete
            && types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(letter_char)
            || types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(
                &(letter_char.to_string() + &next_char),
            ) && is_s_n
        {
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorTypeCollector {
                    data: types::operator_type::OperatorType {
                        first: Box::new(itered_data.data.value.clone()),
                        operator: types::operator_type::Operators::ArithmeticType(
                            types::arithmetic_type::ArithmeticOperators::Null,
                        ),
                        ..Default::default()
                    },
                    operator_collect: letter_char.to_string(),
                    first_filled: true,
                    ..Default::default()
                });
        } else {
            if letter_char != " " {
                //TODO IS THIS SAFE ?
                data.comma = false;
            }

            let mut will_be_itered: variable::VariableCollector;
            if let definers::DefinerCollecting::Array(array_data) = itered_data.data.rtype.clone() {
                will_be_itered = if data.data.collective.is_empty() {
                    variable::VariableCollector {
                        data: variable::Variable {
                            rtype: *array_data.rtype.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                } else {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *data.data.collective[data.data.collective.len() - 1]
                                .value
                                .clone(),
                            rtype: *array_data.rtype.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                };
            } else if let definers::DefinerCollecting::GrowableArray(array_data) =
                itered_data.data.rtype.clone()
            {
                will_be_itered = if data.data.collective.is_empty() {
                    variable::VariableCollector {
                        data: variable::Variable {
                            rtype: *array_data.rtype.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                } else {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *data.data.collective[data.data.collective.len() - 1]
                                .value
                                .clone(),
                            rtype: *array_data.rtype.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                };
            } else {
                will_be_itered = if data.data.collective.is_empty() {
                    variable::VariableCollector {
                        ..variable::VariableCollector::default()
                    }
                } else {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *data.data.collective[data.data.collective.len() - 1]
                                .value
                                .clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                };
                //#[cfg(feature = "std")]
                //std::println!("[ParserError:0x1]: This shouldn't have happened");
            }

            value_processor::collect_value(
                parser.clone(),
                &mut will_be_itered,
                errors,
                letter_char,
                next_char,
                last_char,
            );

            if let types::Types::Array(ref array_data) = will_be_itered.data.value {
                if array_data.complete {
                    data.child_start = false;
                }
            }

            let itered_entry = match will_be_itered.data.value {
                types::Types::Integer(match_data) => types::array_type::ArrayEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Integer(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                            && letter_char != " "
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Float(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Float(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Operator(match_data) => types::array_type::ArrayEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Operator(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Bool(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Bool(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::String(match_data) => types::array_type::ArrayEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::String(match_data.clone())),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Char(match_data) => types::array_type::ArrayEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Char(match_data.clone())),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                            && match_data.value.clone() != '\0'
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Collective(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Collective(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Reference(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Reference(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Array(match_data) => types::array_type::ArrayEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Array(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Cloak(match_data) => types::array_type::ArrayEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Cloak(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::ArrowFunction(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::ArrowFunction(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::FunctionCall(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::FunctionCall(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::ConstructedClass(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::ConstructedClass(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Negative(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Negative(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::NullResolver(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::NullResolver(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Void => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Void),
                    location: defs::Cursor {
                        range_start: parser.pos,
                        ..Default::default()
                    },
                },
                types::Types::VariableType(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::VariableType(match_data)),
                    location: defs::Cursor {
                        range_start: if data.data.collective.len() != 0
                            && !data.data.collective[last_entry - 1].location.is_zero()
                        {
                            data.data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Null => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                    location: defs::Cursor {
                        range_start: parser.pos.clone().skip_char(1),
                        ..Default::default()
                    },
                },
            };

            if data.data.collective.is_empty() {
                data.data.collective.push(itered_entry);
                last_entry += 1;
            } else {
                data.data.collective[last_entry - 1] = itered_entry;
            }
            data.data.collective[last_entry - 1].location.range_end =
                parser.pos.clone().skip_char(1);

            if let definers::DefinerCollecting::Array(array_def) = itered_data.data.rtype.clone() {
                if array_def
                    .len
                    .data
                    .value
                    .greater_than(data.data.collective.len() as isize)
                    && letter_char != " "
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "array_processor".to_owned(),
                        debug_message: "9e23bb1264de0c461104297f24bce1e8".to_owned(),
                        title: error::errorList::error_s19.title.clone(),
                        code: error::errorList::error_s19.code,
                        message: error::errorList::error_s19.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s19.message.clone(),
                            vec![
                                error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: array_def.len.data.value.get_val(),
                                },
                                error::ErrorBuildField {
                                    key: "token2".to_owned(),
                                    value: data.data.collective.len().to_string(),
                                },
                            ],
                        ),
                        pos: data.data.collective[last_entry - 1].location,
                    });
                }
            }
        }
    }
}
