use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{definers, types, variable};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error, utils};

pub fn collect_reference(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::Reference(ref mut data) = itered_data.data.value {
        if letter_char == "."
            && (data.chain.len() == 0
                || matches!(&data.chain[data.chain.len() - 1].value, types::reference_type::ChainType::Setter(setter_data) if setter_data.value.is_type_complete())
                    && matches!(&data.chain[data.chain.len() - 1].value, types::reference_type::ChainType::FunctionCall(function_call_data) if function_call_data.complete))
        {
            if data.on_dot {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "reference_processor".to_string(),
                    debug_message: "e22c23735e19d1fb8e4fb0bf115d5708".to_string(),
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
                if data.chain.len() == 0 {
                    data.on_dot = true;
                } else {
                    data.on_dot = true;
                }
            }
        } else {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );
            let last_chain = data.chain.clone().len();

            if last_chain == 0 {
                data.chain.push(types::reference_type::Chain {
                    pos: defs::Cursor {
                        range_start: parser.pos.clone(),
                        ..Default::default()
                    },
                    value: types::reference_type::ChainType::Getter(
                        types::reference_type::GetterChain {
                            value: letter_char.to_string(),
                        },
                    ),
                });
            } else {
                let clone_ref_data = data.clone();
                match &mut data.chain[last_chain - 1].value {
                    types::reference_type::ChainType::Getter(getter_data) => {
                        if current_reliability.reliable {
                            getter_data.value += letter_char;
                        } else if letter_char == "(" && getter_data.value != "" {
                            data.chain[last_chain - 1].value =
                                types::reference_type::ChainType::FunctionCall(
                                    types::function_call::FunctionCallCollector {
                                        data: types::function_call::FunctionCall {
                                            name: getter_data.value.clone(),
                                            name_pos: data.chain[last_chain - 1].pos,
                                            ..Default::default()
                                        },
                                        name_collected: true,
                                        ..Default::default()
                                    },
                                );
                        } else if letter_char == "=" && getter_data.value != "" {
                            data.chain[last_chain - 1].value =
                                types::reference_type::ChainType::Setter(
                                    types::reference_type::SetterChain {
                                        name: getter_data.value.clone(),
                                        name_set: true,
                                        ..Default::default()
                                    },
                                );
                        } else if letter_char != " " {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: "reference_processor".to_string(),
                                debug_message: "0a316625cca267908704a5e63bc5c43d".to_string(),
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
                    }
                    types::reference_type::ChainType::Setter(setter_data) => {
                        //panic!("SETTER NOT IMPLEMENTED");
                        let itered_setter_vector = Box::new(value_processor::collect_value(
                            parser.clone(),
                            &mut variable::VariableCollector {
                                data: variable::Variable {
                                    value: setter_data.value.clone(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            letter_char,
                            next_char.clone(),
                            last_char,
                        ));

                        if !itered_setter_vector.errors.is_empty() {
                            errors.extend(itered_setter_vector.errors);
                        }
                        setter_data.value = itered_setter_vector.itered_data.data.value;

                        if setter_data.value.is_type_complete() {}
                    }
                    types::reference_type::ChainType::FunctionCall(functioncalldata) => {
                        if itered_data.data.dynamic {
                            itered_data.data.rtype =
                                definers::DefinerCollecting::Generic(definers::GenericType {
                                    rtype: "functionCall".to_string(),
                                });
                        }

                        if !functioncalldata.name_collected {
                            panic!("This should never happen")
                        } else if !functioncalldata.complete {
                            let last_entry = functioncalldata.data.params.clone().len();
                            let is_s_n = last_entry == 0
                                || functioncalldata.data.params[last_entry - 1]
                                    .value
                                    .is_type_complete();

                            if letter_char == "," && is_s_n && last_entry != 0 {
                                if functioncalldata.complete {
                                    errors.push(error::Error {
                                        path: parser.options.path.clone(),
                                        scope: "function_call_processor".to_string(),
                                        debug_message: "2db4b8f4124945a260471da1c21591f2"
                                            .to_string(),
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
                                } else if functioncalldata.comma {
                                    errors.push(error::Error {
                                        path: parser.options.path.clone(),
                                        scope: "function_call_processor".to_string(),
                                        debug_message: "17609f94825bd20cf786d03a4287c29a"
                                            .to_string(),
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
                                        functioncalldata.data.params[last_entry - 1]
                                            .value
                                            .make_complete();
                                    }
                                    functioncalldata.comma = true;
                                    functioncalldata.data.params.push(
                                        types::function_call::FunctionCallParameter::default(),
                                    );
                                }
                            } else if letter_char == ")" && is_s_n {
                                if last_entry != 0 {
                                    functioncalldata.data.params[last_entry - 1].pos.range_end =
                                        parser.pos;
                                }

                                let fn_exists = parser.resolve_reference_function_call(
                                    clone_ref_data,
                                    functioncalldata.clone(),
                                );
                                if let Some(type_errors) = fn_exists {
                                    for error in type_errors {
                                        errors.push(error);
                                    }
                                }
                                functioncalldata.complete = true;
                            } else {
                                if letter_char != " " {
                                    //TODO IS THIS SAFE ?
                                    functioncalldata.comma = false;
                                }

                                //TODO FIX THIS with function after resolving complete
                                let mut will_be_itered: variable::VariableCollector;
                                if let definers::DefinerCollecting::Cloak(cloak_data) =
                                    itered_data.data.rtype.clone()
                                {
                                    will_be_itered = if functioncalldata.data.params.is_empty() {
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
                                                value: functioncalldata.data.params
                                                    [functioncalldata.data.params.len() - 1]
                                                    .value
                                                    .clone(),
                                                rtype: cloak_data.rtype
                                                    [functioncalldata.data.params.len() - 1]
                                                    .clone(),
                                                ..Default::default()
                                            },
                                            ..variable::VariableCollector::default()
                                        }
                                    };
                                } else {
                                    will_be_itered = if functioncalldata.data.params.is_empty() {
                                        variable::VariableCollector::default()
                                    } else {
                                        variable::VariableCollector {
                                            data: variable::Variable {
                                                value: functioncalldata.data.params
                                                    [functioncalldata.data.params.len() - 1]
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
                                    next_char.clone(),
                                    last_char,
                                ));

                                let itered_entry = match itered_fcall_vector.itered_data.data.value
                                {
                                    types::Types::Integer(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Integer(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::Float(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Float(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::Operator(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Operator(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::Bool(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Bool(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::String(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::String(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::Char(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Char(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::Collective(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Collective(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::Reference(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Reference(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::BraceReference(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::BraceReference(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::Array(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Array(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::Cloak(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Cloak(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::ArrowFunction(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::ArrowFunction(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::FunctionCall(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::FunctionCall(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::ConstructedClass(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::ConstructedClass(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::Void => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Void,
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::Negative(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Negative(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::VariableType(match_data) => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::VariableType(match_data),
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                    types::Types::Null => {
                                        types::function_call::FunctionCallParameter {
                                            value: types::Types::Null,
                                            pos: if last_entry == 0 {
                                                defs::Cursor::default()
                                            } else {
                                                functioncalldata.data.params[last_entry - 1].pos
                                            },
                                        }
                                    }
                                };

                                if !itered_fcall_vector.errors.is_empty() {
                                    errors.extend(itered_fcall_vector.errors);
                                }
                                if functioncalldata.data.params.is_empty() {
                                    functioncalldata.data.params.push(itered_entry);

                                    if functioncalldata.data.params[0].pos.is_zero() {
                                        functioncalldata.data.params[0].pos.range_start =
                                            parser.pos;
                                    }
                                    functioncalldata.data.params[0].pos.range_end = parser.pos;
                                } else {
                                    functioncalldata.data.params[last_entry - 1] = itered_entry;
                                    if functioncalldata.data.params[last_entry - 1].pos.is_zero() {
                                        functioncalldata.data.params[last_entry - 1]
                                            .pos
                                            .range_start = parser.pos;
                                    }
                                    functioncalldata.data.params[last_entry - 1].pos.range_end =
                                        parser.pos;
                                }
                            }
                        } else if letter_char == "." {
                            data.on_dot = true;
                        }
                    }
                };
            }
        }
    }
}
