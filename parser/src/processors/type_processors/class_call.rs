use crate::parser;
use crate::processors::{type_processors, value_processor};
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::error;
use ellie_core::{defs, utils};

pub fn collect_class_call(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::ClassCall(ref mut classcalldata) = itered_data.data.value {
        if !classcalldata.keyword_collected {
            if classcalldata.keyword_index == 0 && letter_char != "n" {
                classcalldata.keyword_index = 1;
            } else if classcalldata.keyword_index == 1 && letter_char != "e" {
                classcalldata.keyword_index = 2
            } else if classcalldata.keyword_index == 2 && letter_char != "w" {
                classcalldata.keyword_collected = true;
            } else if (letter_char == " " && classcalldata.keyword_index == 0) || letter_char != " "
            {
                errors.push(error::Error {
                    scope: "function_call_processor".to_string(),
                    debug_message: "b67cd63a3552a23651e1c3e6fe24e9a5".to_string(),
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
        } else if !classcalldata.name_collected {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n") && !classcalldata.data.name.is_empty())
            {
                classcalldata.data.name_pos.range_end = parser.pos;
                classcalldata.data.name += letter_char;
            } else if letter_char == "(" {
                if classcalldata.data.name.is_empty() {
                    errors.push(error::Error {
                        scope: "function_call_processor".to_string(),
                        debug_message: "a10376b928d50f748066c78611c31a21".to_string(),
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
                } else {
                    classcalldata.name_collected = true;
                }
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: "function_call_processor".to_string(),
                    debug_message: "ed3315ba2dcbf551b0ede95aa7258d9a".to_string(),
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
        } else if !classcalldata.complete {
            let last_entry = classcalldata.data.params.clone().len();
            let is_s_n = last_entry == 0
                || classcalldata.data.params[last_entry - 1]
                    .value
                    .is_type_complete();

            if letter_char == "," && is_s_n && last_entry != 0 {
                if classcalldata.complete {
                    errors.push(error::Error {
                        scope: "function_call_processor".to_string(),
                        debug_message: "26f9afeda4c7067a204b48cae9115085".to_string(),
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
                } else if classcalldata.comma {
                    errors.push(error::Error {
                        scope: "function_call_processor".to_string(),
                        debug_message: "70e07e4aa968a36330fd7828a50121e9".to_string(),
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
                } else {
                    if last_entry != 0 {
                        classcalldata.data.params[last_entry - 1]
                            .value
                            .make_complete();
                    }
                    classcalldata.comma = true;
                    classcalldata
                        .data
                        .params
                        .push(types::class_call::ClassCallParameter::default());
                }
            } else if letter_char == ")" && is_s_n {
                if last_entry != 0 {
                    classcalldata.data.params[last_entry - 1].pos.range_end = parser.pos;
                }

                let fn_exists = parser.resolve_class_call(classcalldata.clone());
                if let Some(type_errors) = fn_exists {
                    for error in type_errors {
                        errors.push(error);
                    }
                }
                classcalldata.complete = true;
            } else {
                if letter_char != " " {
                    //TODO IS THIS SAFE ?
                    classcalldata.comma = false;
                }

                //TODO FIX THIS with function after resolving complete
                let mut will_be_itered: variable::VariableCollector;
                if let definers::DefinerCollecting::Cloak(cloak_data) =
                    itered_data.data.rtype.clone()
                {
                    will_be_itered = if classcalldata.data.params.is_empty() {
                        variable::VariableCollector {
                            data: variable::Variable {
                                rtype: cloak_data.rtype[0].clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    } else {
                        variable::VariableCollector {
                            data: variable::Variable {
                                value: classcalldata.data.params
                                    [classcalldata.data.params.len() - 1]
                                    .value
                                    .clone(),
                                rtype: cloak_data.rtype[classcalldata.data.params.len() - 1]
                                    .clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    };
                } else {
                    will_be_itered = if classcalldata.data.params.is_empty() {
                        variable::VariableCollector::default()
                    } else {
                        variable::VariableCollector {
                            data: variable::Variable {
                                value: classcalldata.data.params
                                    [classcalldata.data.params.len() - 1]
                                    .value
                                    .clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    };
                }

                let itered_fcall_vector = Box::new(value_processor::collect_value(
                    parser.clone(),
                    &mut will_be_itered,
                    letter_char,
                    next_char,
                    last_char,
                ));

                let itered_entry = match itered_fcall_vector.itered_data.data.value {
                    types::Types::Integer(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Integer(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Float(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Float(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Operator(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Operator(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Bool(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Bool(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::String(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::String(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Char(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Char(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Collective(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Collective(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Refference(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Refference(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Array(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Array(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Cloak(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Cloak(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::ArrowFunction(match_data) => {
                        types::class_call::ClassCallParameter {
                            value: types::Types::ArrowFunction(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                classcalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::FunctionCall(match_data) => {
                        types::class_call::ClassCallParameter {
                            value: types::Types::FunctionCall(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                classcalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::ClassCall(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::ClassCall(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Void => types::class_call::ClassCallParameter {
                        value: types::Types::Void,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::VariableType(match_data) => {
                        types::class_call::ClassCallParameter {
                            value: types::Types::VariableType(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                classcalldata.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Null => types::class_call::ClassCallParameter {
                        value: types::Types::Null,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            classcalldata.data.params[last_entry - 1].pos
                        },
                    },
                };

                if !itered_fcall_vector.errors.is_empty() {
                    errors.extend(itered_fcall_vector.errors);
                }
                if classcalldata.data.params.is_empty() {
                    classcalldata.data.params.push(itered_entry);

                    if classcalldata.data.params[0].pos.is_zero() {
                        classcalldata.data.params[0].pos.range_start = parser.pos;
                    }
                    classcalldata.data.params[0].pos.range_end = parser.pos;
                } else {
                    classcalldata.data.params[last_entry - 1] = itered_entry;
                    if classcalldata.data.params[last_entry - 1].pos.is_zero() {
                        classcalldata.data.params[last_entry - 1].pos.range_start = parser.pos;
                    }
                    classcalldata.data.params[last_entry - 1].pos.range_end = parser.pos;
                }
            }
        } else if letter_char == "." {
            itered_data.data.value =
                types::Types::Refference(types::refference_type::RefferenceType {
                    refference: Box::new(itered_data.data.value.clone()),
                    chain: Vec::new(),
                    on_dot: false,
                });
            type_processors::refference::collect_refference(
                parser,
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
            )
        } else if types::logical_type::LogicalOpearators::is_logical_opearator(letter_char) {
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
                parser,
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
            )
        } else if types::comparison_type::ComparisonOperators::is_comparison_opearator(letter_char)
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
                parser,
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
            )
        } else if types::arithmetic_type::ArithmeticOperators::is_arithmetic_opearator(letter_char)
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
                parser,
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
            )
        }
    }
}
