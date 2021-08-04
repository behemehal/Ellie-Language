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
    if let types::Types::ClassCall(ref mut class_call_data) = itered_data.data.value {
        if !class_call_data.keyword_collected {
            if class_call_data.keyword_index == 0 && letter_char != "n" {
                class_call_data.keyword_index = 1;
            } else if class_call_data.keyword_index == 1 && letter_char != "e" {
                class_call_data.keyword_index = 2
            } else if class_call_data.keyword_index == 2 && letter_char != "w" {
                class_call_data.keyword_collected = true;
                class_call_data.ignore_space = true;
            } else if (letter_char == " " && class_call_data.keyword_index == 0)
                || letter_char != " "
            {
                errors.push(error::Error {
                    scope: "function_call_processor".to_string(),
                    debug_message: "4f0fc7b0def70759390e812989882b40".to_string(),
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
        } else if !class_call_data.name_collected {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable
                && (class_call_data.ignore_space
                    || ((last_char != " " && last_char != "\n")
                        && !class_call_data.data.name.is_empty()))
            {
                class_call_data.ignore_space = false;
                class_call_data.data.name_pos.range_end = parser.pos;
                class_call_data.data.name += letter_char;
            } else if letter_char == "(" {
                if class_call_data.data.name.is_empty() {
                    errors.push(error::Error {
                        scope: "function_call_processor".to_string(),
                        debug_message: "d3fbc7e3cf53db467f778a8640d724e6".to_string(),
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
                    class_call_data.name_collected = true;
                }
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: "function_call_processor".to_string(),
                    debug_message: "aa5ed75a413911a3b370ef1bb4b3d546".to_string(),
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
        } else if !class_call_data.complete {
            let last_entry = class_call_data.data.params.clone().len();
            let is_s_n = last_entry == 0
                || class_call_data.data.params[last_entry - 1]
                    .value
                    .is_type_complete();

            if letter_char == "," && is_s_n && last_entry != 0 {
                if class_call_data.complete {
                    errors.push(error::Error {
                        scope: "function_call_processor".to_string(),
                        debug_message: "3009c14c8ab234df0f3cea265c27aac9".to_string(),
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
                } else if class_call_data.comma {
                    errors.push(error::Error {
                        scope: "function_call_processor".to_string(),
                        debug_message: "c626ae3fce6e1921ac5983d4f488f995".to_string(),
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
                        class_call_data.data.params[last_entry - 1]
                            .value
                            .make_complete();
                    }
                    class_call_data.comma = true;
                    class_call_data
                        .data
                        .params
                        .push(types::class_call::ClassCallParameter::default());
                }
            } else if letter_char == ")" && is_s_n {
                if last_entry != 0 {
                    class_call_data.data.params[last_entry - 1].pos.range_end = parser.pos;
                }

                let fn_exists = parser.resolve_class_call(class_call_data.clone());
                if let Some(type_errors) = fn_exists {
                    for error in type_errors {
                        errors.push(error);
                    }
                }
                class_call_data.complete = true;
            } else {
                if letter_char != " " {
                    //TODO IS THIS SAFE ?
                    class_call_data.comma = false;
                }

                //TODO FIX THIS with function after resolving complete
                let mut will_be_itered: variable::VariableCollector;
                if let definers::DefinerCollecting::Cloak(cloak_data) =
                    itered_data.data.rtype.clone()
                {
                    will_be_itered = if class_call_data.data.params.is_empty() {
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
                                value: class_call_data.data.params
                                    [class_call_data.data.params.len() - 1]
                                    .value
                                    .clone(),
                                rtype: cloak_data.rtype[class_call_data.data.params.len() - 1]
                                    .clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    };
                } else {
                    will_be_itered = if class_call_data.data.params.is_empty() {
                        variable::VariableCollector::default()
                    } else {
                        variable::VariableCollector {
                            data: variable::Variable {
                                value: class_call_data.data.params
                                    [class_call_data.data.params.len() - 1]
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
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Float(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Float(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Operator(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Operator(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Bool(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Bool(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::String(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::String(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Char(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Char(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Collective(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Collective(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Reference(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Reference(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::BraceReference(match_data) => {
                        types::class_call::ClassCallParameter {
                            value: types::Types::BraceReference(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                class_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Array(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Array(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Cloak(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Cloak(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::ArrowFunction(match_data) => {
                        types::class_call::ClassCallParameter {
                            value: types::Types::ArrowFunction(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                class_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::FunctionCall(match_data) => {
                        types::class_call::ClassCallParameter {
                            value: types::Types::FunctionCall(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                class_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::ClassCall(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::ClassCall(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::Negative(match_data) => types::class_call::ClassCallParameter {
                        value: types::Types::Negative(match_data),
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },

                    types::Types::Void => types::class_call::ClassCallParameter {
                        value: types::Types::Void,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::VariableType(match_data) => {
                        types::class_call::ClassCallParameter {
                            value: types::Types::VariableType(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                class_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Null => types::class_call::ClassCallParameter {
                        value: types::Types::Null,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            class_call_data.data.params[last_entry - 1].pos
                        },
                    },
                };

                if !itered_fcall_vector.errors.is_empty() {
                    errors.extend(itered_fcall_vector.errors);
                }
                if class_call_data.data.params.is_empty() {
                    class_call_data.data.params.push(itered_entry);

                    if class_call_data.data.params[0].pos.is_zero() {
                        class_call_data.data.params[0].pos.range_start = parser.pos;
                    }
                    class_call_data.data.params[0].pos.range_end = parser.pos;
                } else {
                    class_call_data.data.params[last_entry - 1] = itered_entry;
                    if class_call_data.data.params[last_entry - 1].pos.is_zero() {
                        class_call_data.data.params[last_entry - 1].pos.range_start = parser.pos;
                    }
                    class_call_data.data.params[last_entry - 1].pos.range_end = parser.pos;
                }
            }
        } else if letter_char == "." {
            itered_data.data.value =
                types::Types::Reference(types::reference_type::ReferenceType {
                    reference: Box::new(itered_data.data.value.clone()),
                    chain: Vec::new(),
                    on_dot: false,
                });
            type_processors::reference::collect_reference(
                parser,
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
            )
        } else if types::logical_type::LogicalOperators::is_logical_operator(letter_char)
            || types::logical_type::LogicalOperators::is_logical_operator(
                &(letter_char.to_string() + &next_char),
            )
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
        } else if types::comparison_type::ComparisonOperators::is_comparison_operator(letter_char)
            || types::comparison_type::ComparisonOperators::is_comparison_operator(
                &(letter_char.to_string() + &next_char),
            )
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
        } else if types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(letter_char)
            || types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(
                &(letter_char.to_string() + &next_char),
            )
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
        }
    }
}
