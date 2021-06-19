use crate::processors::{type_processors, value_processor};
use crate::syntax::{definers, types, variable};
use ellie_core::{defs, error, utils};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_array(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    options: defs::ParserOptions,
) {
    if let types::Types::Array(ref mut data) = itered_data.data.value {
        /*
            Don't look right to it, it's dangerously complicated
            Here is the story,

            I assume you as a person that doesn't have a programming experience. In a loop you can process a data
            and if a same data applied you can use the same function to process the data, This program uses millions of same pattern,
            I experienced this a million times, Created programs that runs through loops processing big data. But this time I got stuck at this
            function. It took almost 2 months, Thank god I got it.

            A Weird way to stop a letter,

            Sincerely

            Ahmetcan Aksu 🦀
        */

        let last_entry = data.clone().collective.len();
        //let mut value: types::Types = types::Types::Null;

        let is_s_n = last_entry == 0 || data.collective[last_entry - 1].value.is_type_complete();

        if letter_char == "[" && !data.child_start && is_s_n {
            if !data.comma && last_entry != 0 {
                errors.push(error::Error {
                    debug_message: "abdcae832dd0ad353ce76d557bc04afa".to_string(),
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
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            } else {
                data.child_start = true;
                if last_entry == 0 {
                    data.collective.push(types::array_type::ArrayEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Array(
                            types::array_type::ArrayType::default(),
                        )),
                    });
                } else {
                    data.collective[last_entry - 1] = types::array_type::ArrayEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Array(
                            types::array_type::ArrayType::default(),
                        )),
                    };
                }
            }
        } else if letter_char == "," && !data.child_start && is_s_n {
            if data.complete {
                errors.push(error::Error {
                    debug_message: "dfeb5de8f2a29d9cf1c4c7169828b34d".to_string(),
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
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            } else if data.comma {
                errors.push(error::Error {
                    debug_message: "6a32204fe280b9c0704a0dc74cdc826f".to_string(),
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
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            } else {
                if last_entry != 0 {
                    data.collective[last_entry - 1].value.make_complete();
                    data.collective[last_entry - 1].value_complete = true;
                }
                data.comma = true;
                data.layer_size += 1;
                data.collective
                    .push(types::array_type::ArrayEntry::default());
            }
        } else if letter_char == "]" && !data.child_start && is_s_n {
            if data.comma {
                errors.push(error::Error {
                    debug_message: "71cb40281b114da0c3e797ec01779ee5".to_string(),
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
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            } else if data.complete {
                errors.push(error::Error {
                    debug_message: "90b696fe81536bf18b43857a0e57b19d".to_string(),
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
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            } else {
                if last_entry != 0 {
                    if data.collective[last_entry - 1].value == Box::new(types::Types::Null) {
                        data.collective.pop();
                    } else {
                        data.collective[last_entry - 1].value_complete = true;
                        data.collective[last_entry - 1].value.make_complete();
                    }
                }
                data.layer_size += 1;
                data.complete = true;
                itered_data.value_complete = true;
            }
        } else if data.complete && letter_char == "." && is_s_n {
            itered_data.data.value =
                types::Types::Refference(types::refference_type::RefferenceType {
                    refference: Box::new(itered_data.data.value.clone()),
                    chain: Vec::new(),
                    on_dot: false,
                });
            type_processors::refference::collect_refference(
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
                pos,
                options,
            )
        } else if data.complete
            && types::logical_type::LogicalOpearators::is_logical_opearator(letter_char)
            && is_s_n
        {
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorTypeCollector {
                    data: types::operator_type::OperatorType {
                        first: Box::new(itered_data.data.value.clone()),
                        operator: types::operator_type::Operators::LogicalType(
                            types::logical_type::LogicalOpearators::Null,
                        ),
                        ..Default::default()
                    },
                    first_filled: true,
                    ..Default::default()
                });
            type_processors::operator::collect_operator(
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
                pos,
                options,
            )
        } else if data.complete
            && types::comparison_type::ComparisonOperators::is_comparison_opearator(letter_char)
            && is_s_n
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
                    first_filled: true,
                    ..Default::default()
                });
            type_processors::operator::collect_operator(
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
                pos,
                options,
            )
        } else if data.complete
            && types::arithmetic_type::ArithmeticOperators::is_arithmetic_opearator(letter_char)
            && is_s_n
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
                    first_filled: true,
                    ..Default::default()
                });
            type_processors::operator::collect_operator(
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
                pos,
                options,
            )
        } else {
            if letter_char != " " {
                //TODO IS THIS SAFE ?
                data.comma = false;
            }

            let mut will_be_itered: variable::VariableCollector;
            if let definers::DefinerCollecting::Array(array_data) = itered_data.data.rtype.clone() {
                if data.collective.len() > *array_data.len.value.as_usize().unwrap() {
                    //Check if array size is overflowed
                    errors.push(error::Error {
                        debug_message: "327eac4b838ea2aed7f5c41dc850ac71".to_string(),
                        title: error::errorList::error_s19.title.clone(),
                        code: error::errorList::error_s19.code,
                        message: error::errorList::error_s19.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s19.message.clone(),
                            vec![
                                error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: array_data.len.value.as_usize().unwrap().to_string(),
                                },
                                error::ErrorBuildField {
                                    key: "token2".to_string(),
                                    value: data.collective.len().to_string(),
                                },
                            ],
                        ),
                        pos: defs::Cursor {
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message: "fe887b5b94882b95ea2e48764f899959".to_string(),
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
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }

                will_be_itered = if data.collective.is_empty() {
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
                            value: *data.collective[data.collective.len() - 1].value.clone(),
                            rtype: *array_data.rtype.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                };
            } else if let definers::DefinerCollecting::GrowableArray(array_data) =
                itered_data.data.rtype.clone()
            {
                will_be_itered = if data.collective.is_empty() {
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
                            value: *data.collective[data.collective.len() - 1].value.clone(),
                            rtype: *array_data.rtype.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                };
            } else {
                will_be_itered = if data.collective.is_empty() {
                    variable::VariableCollector {
                        ..variable::VariableCollector::default()
                    }
                } else {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *data.collective[data.collective.len() - 1].value.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                };
                #[cfg(feature = "std")]
                std::println!(
                    "{}[ParserError:0x1]{}: This shouldn't have happened",
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Red),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                );
            }

            let itered_array_vector = Box::new(value_processor::collect_value(
                &mut will_be_itered,
                letter_char,
                next_char,
                last_char,
                defs::CursorPosition(0, 0),
                options,
            ));

            if let types::Types::Array(ref adata) = itered_array_vector.itered_data.data.value {
                if adata.complete {
                    data.child_start = false;
                }
            }

            let itered_entry = match itered_array_vector.itered_data.data.value {
                types::Types::Integer(match_data) => types::array_type::ArrayEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Integer(match_data)),
                },
                types::Types::Float(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Float(match_data)),
                },
                types::Types::Operator(match_data) => types::array_type::ArrayEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Operator(match_data)),
                },
                types::Types::Bool(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Bool(match_data)),
                },
                types::Types::String(match_data) => types::array_type::ArrayEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::String(match_data)),
                },
                types::Types::Char(match_data) => types::array_type::ArrayEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Char(match_data)),
                },
                types::Types::Collective => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::Refference(_) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::Array(match_data) => types::array_type::ArrayEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Array(match_data)),
                },
                types::Types::Cloak(match_data) => types::array_type::ArrayEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Cloak(match_data)),
                },
                types::Types::ArrowFunction(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::ArrowFunction(match_data)),
                },
                types::Types::FunctionCall(_) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::Void => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::VariableType(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::VariableType(match_data)),
                },
                types::Types::Null => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
            };

            if !itered_array_vector.errors.is_empty() {
                for returned_error in itered_array_vector.errors {
                    //errors.extend(itered_array_vector.errors);
                    let mut edited = returned_error;
                    edited.pos.range_start.0 += pos.0;
                    edited.pos.range_start.1 += pos.1;
                    edited.pos.range_end.0 += pos.0;
                    edited.pos.range_end.1 += pos.1;
                    errors.push(edited);
                }
            }

            if data.collective.is_empty() {
                data.collective.push(itered_entry);
            } else {
                data.collective[last_entry - 1] = itered_entry;
            }
        }
    }
}



