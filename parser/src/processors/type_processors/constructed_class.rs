use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::defs;
use ellie_core::error;

pub fn collect_new_call<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    if let types::Types::ConstructedClass(ref mut new_call_data) = itered_data.data.value {
        if !new_call_data.keyword_collected {
            if new_call_data.keyword_index == 0 && letter_char == "n" {
                new_call_data.data.keyword_pos.range_start = parser.pos.clone();
                new_call_data.keyword_index = 1;
            } else if new_call_data.keyword_index == 1 && letter_char == "e" {
                new_call_data.keyword_index = 2
            } else if new_call_data.keyword_index == 2 && letter_char == "w" {
                new_call_data.keyword_index = 3;
                new_call_data.data.keyword_pos.range_end = parser.pos.clone();
            } else if new_call_data.keyword_index == 3 && letter_char == " " {
                new_call_data.keyword_collected = true;
            } else if (letter_char == " " && new_call_data.keyword_index == 0) || letter_char != " "
            {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "function_call_processor".to_owned(),
                    debug_message: "166d25697eaa7119cba059a5edd0abb3".to_owned(),
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
        } else if !new_call_data.value_collected {
            if new_call_data.data.value.is_type_complete() && letter_char == "(" {
                new_call_data.value_collected = true;
            } else {
                if new_call_data.data.value_pos.is_zero() {
                    new_call_data.data.value_pos.range_start = parser.pos.clone();
                }

                let mut will_be_itered = variable::VariableCollector {
                    data: variable::Variable {
                        value: *new_call_data.data.value.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                value_processor::collect_value(
                    parser.clone(),
                    &mut will_be_itered,
                    errors,
                    letter_char,
                    next_char.clone(),
                    last_char,
                );

                new_call_data.raw_value += letter_char;
                new_call_data.data.value = Box::new(will_be_itered.data.value);
                new_call_data.data.value_pos.range_end = parser.pos.clone().skip_char(1);
            }
        } else if !new_call_data.complete {
            let last_entry = new_call_data.data.params.clone().len();
            let is_s_n = last_entry == 0
                || new_call_data.data.params[last_entry - 1]
                    .value
                    .is_type_complete();

            if letter_char == "," && is_s_n && last_entry != 0 {
                if new_call_data.complete {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "function_call_processor".to_owned(),
                        debug_message: "803571a755c67ec57f078f98ca675894".to_owned(),
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
                } else if new_call_data.comma {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "function_call_processor".to_owned(),
                        debug_message: "c8263770d363007c3e24a65a21a35e4d".to_owned(),
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
                        new_call_data.data.params[last_entry - 1]
                            .value
                            .make_complete();
                    }
                    new_call_data.comma = true;
                    new_call_data
                        .data
                        .params
                        .push(types::constructed_class::ConstructedClassParameter::default());
                }
            } else if letter_char == ")" && is_s_n {
                if last_entry != 0 {
                    new_call_data.data.params[last_entry - 1].pos.range_end = parser.pos;
                }
                let class_call_resolve = parser.resolve_new_call(new_call_data.clone());
                match class_call_resolve {
                    Ok(resolved) => {
                        if let parser::Collecting::Class(class_collector) = resolved {
                            if class_collector.data.constructor.parameters.len()
                                != new_call_data.data.params.len()
                            {
                                errors.push(error::Error {
                                    path: parser.options.path.clone(),
                                    scope: parser.scope.scope_name.clone(),
                                    debug_message: "replace_parser_577".to_owned(),
                                    title: error::errorList::error_s19.title.clone(),
                                    code: error::errorList::error_s19.code,
                                    message: error::errorList::error_s19.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s19.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token".to_owned(),
                                                value: class_collector
                                                    .data
                                                    .constructor
                                                    .parameters
                                                    .len()
                                                    .to_string(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_owned(),
                                                value: new_call_data.data.params.len().to_string(),
                                            },
                                        ],
                                    ),
                                    pos: new_call_data.data.value_pos,
                                });
                            } else {
                                let mut has_faulty_param = false;
                                for (pos, param) in
                                    new_call_data.data.params.clone().into_iter().enumerate()
                                {
                                    let constructor_param =
                                        &class_collector.data.constructor.parameters[pos];
                                    let properties = class_collector
                                        .data
                                        .properties
                                        .iter()
                                        .filter(|e| e.name == constructor_param.name)
                                        .collect::<Vec<&variable::Variable>>();

                                    if properties.len() != 0 {
                                        let property = properties[0];
                                        if property.rtype.raw_name() != param.value.get_type() {
                                            errors.push(error::Error {
                                                path: parser.options.path.clone(),
                                                scope: parser.scope.scope_name.clone(),
                                                debug_message: "replace_parser_640".to_owned(),
                                                title: error::errorList::error_s3.title.clone(),
                                                code: error::errorList::error_s3.code,
                                                message: error::errorList::error_s3.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s3.message.clone(),
                                                    vec![
                                                        error::ErrorBuildField {
                                                            key: "token1".to_owned(),
                                                            value: property.rtype.raw_name(),
                                                        },
                                                        error::ErrorBuildField {
                                                            key: "token2".to_owned(),
                                                            value: param.value.get_type(),
                                                        },
                                                    ],
                                                ),
                                                pos: param.pos,
                                            });
                                        }
                                    }
                                }

                                for param in class_collector.data.constructor.parameters {
                                    let properties = class_collector
                                        .data
                                        .properties
                                        .iter()
                                        .filter(|e| e.name == param.name)
                                        .collect::<Vec<&variable::Variable>>();

                                    if properties.len() != 0 {
                                        let property = properties[0];
                                    }
                                }
                            }
                        } else {
                            panic!("Unexpected parser behaviour")
                        }
                    }
                    Err(e) => errors.extend(e),
                }
                new_call_data.complete = true;
            } else {
                if letter_char != " " {
                    //TODO IS THIS SAFE ?
                    new_call_data.comma = false;
                }

                //TODO FIX THIS with function after resolving complete
                let mut will_be_itered: variable::VariableCollector;
                if let definers::DefinerCollecting::Cloak(cloak_data) =
                    itered_data.data.rtype.clone()
                {
                    will_be_itered = if new_call_data.data.params.is_empty() {
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
                                value: new_call_data.data.params
                                    [new_call_data.data.params.len() - 1]
                                    .value
                                    .clone(),
                                rtype: cloak_data.rtype[new_call_data.data.params.len() - 1]
                                    .clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    };
                } else {
                    will_be_itered = if new_call_data.data.params.is_empty() {
                        variable::VariableCollector::default()
                    } else {
                        variable::VariableCollector {
                            data: variable::Variable {
                                value: new_call_data.data.params
                                    [new_call_data.data.params.len() - 1]
                                    .value
                                    .clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    };
                }

                value_processor::collect_value(
                    parser.clone(),
                    &mut will_be_itered,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                );

                let itered_entry = match will_be_itered.data.value {
                    types::Types::Integer(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Integer(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Float(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Float(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Operator(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Operator(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Bool(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Bool(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::String(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::String(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Char(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Char(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Collective(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Collective(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Reference(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Reference(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::NullResolver(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::NullResolver(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Negative(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Negative(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }

                    types::Types::Array(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Array(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Cloak(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Cloak(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::ArrowFunction(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::ArrowFunction(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::FunctionCall(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::FunctionCall(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::ConstructedClass(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::ConstructedClass(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Void => types::constructed_class::ConstructedClassParameter {
                        value: types::Types::Void,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            new_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::VariableType(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::VariableType(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Null => types::constructed_class::ConstructedClassParameter {
                        value: types::Types::Null,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            new_call_data.data.params[last_entry - 1].pos
                        },
                    },
                };

                if new_call_data.data.params.is_empty() {
                    new_call_data.data.params.push(itered_entry);

                    if new_call_data.data.params[0].pos.is_zero() {
                        new_call_data.data.params[0].pos.range_start = parser.pos;
                    }
                    new_call_data.data.params[0].pos.range_end = parser.pos;
                } else {
                    new_call_data.data.params[last_entry - 1] = itered_entry;
                    if new_call_data.data.params[last_entry - 1]
                        .pos
                        .range_start
                        .is_zero()
                        && letter_char != " "
                    {
                        new_call_data.data.params[last_entry - 1].pos.range_start = parser.pos;
                    }
                    new_call_data.data.params[last_entry - 1].pos.range_end = parser.pos;
                }
            }
        } else if letter_char == "." {
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
                });
        }
    }
}
