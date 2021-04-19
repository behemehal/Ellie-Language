use crate::error;
use crate::mapper;
use crate::syntax::types;
use crate::syntax::variable;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct CollectorResponse {
    pub itered_data: variable::VariableCollector,
    pub errors: Vec<error::Error>,
}

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: mapper::defs::CursorPosition,
) -> CollectorResponse {
    let mut errors: Vec<error::Error> = Vec::new();
    match &mut itered_data.data.value {
        types::Types::Number(data) => {
            let is_num = letter_char.parse::<usize>().is_ok();
            if is_num {
                if data.complete {
                    errors.push(error::Error {
                        debug_message: "Caria".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.complete = true;
                    data.value = (data.value.to_string() + &letter_char)
                        .parse::<usize>()
                        .unwrap();
                }
            } else {
                if letter_char == "." {
                    // String prototype
                    itered_data.data.value =
                        types::Types::Refference(types::refference_type::RefferenceType {
                            refference: Box::new(itered_data.data.value.clone()),
                            on_dot: true,
                            chain: Vec::new(),
                        });
                } else if types::logical_type::LogicalOpearators::is_opearator(letter_char) {
                    itered_data.data.value =
                        types::Types::Operator(types::operator_type::OperatorType {
                            first: Box::new(itered_data.data.value.clone()),
                            first_filled: true,
                            operator: types::operator_type::Operators::LogicalType(
                                types::logical_type::LogicalOpearators::Null,
                            ),
                            operator_collect: letter_char.to_string(),
                            ..Default::default()
                        });
                } else if types::comparison_type::ComparisonOperators::is_opearator(letter_char) {
                    itered_data.data.value =
                        types::Types::Operator(types::operator_type::OperatorType {
                            first: Box::new(itered_data.data.value.clone()),
                            first_filled: true,
                            operator_collect: letter_char.to_string(),
                            operator: types::operator_type::Operators::ComparisonType(
                                types::comparison_type::ComparisonOperators::Null,
                            ),
                            ..Default::default()
                        });
                } else if letter_char == " " || letter_char == ")" {
                    data.complete = true;
                } else {
                    errors.push(error::Error {
                        debug_message: "mRNA".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
            }
            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Double(_) => CollectorResponse {
            itered_data: itered_data.clone(),
            errors,
        },
        types::Types::Bool(_) => CollectorResponse {
            itered_data: itered_data.clone(),
            errors,
        },
        types::Types::String(data) => {
            if letter_char == data.quote_type && last_char != "\\" {
                if data.complete {
                    errors.push(error::Error {
                        debug_message: "Mece".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.complete = true;
                }
                CollectorResponse {
                    itered_data: itered_data.clone(),
                    errors,
                }
            } else if letter_char == "." {
                // String prototype
                itered_data.data.value =
                    types::Types::Refference(types::refference_type::RefferenceType {
                        refference: Box::new(itered_data.data.value.clone()),
                        on_dot: true,
                        chain: Vec::new(),
                    });
                CollectorResponse {
                    itered_data: itered_data.clone(),
                    errors,
                }
            } else if utils::is_opearators(letter_char) {
                //itered_data.data.value = types::Types::Operators(types::OperatorType {
                //    first: Box::new(itered_data.data.value.clone()),
                //    operator_collect: letter_char.to_string(),
                //    collecting_operator: true,
                //    ..Default::default()
                //});
                CollectorResponse {
                    itered_data: itered_data.clone(),
                    errors,
                }
            } else if letter_char != "\\" {
                data.value = data.value.clone() + &letter_char;
                CollectorResponse {
                    itered_data: itered_data.clone(),
                    errors,
                }
            } else {
                CollectorResponse {
                    itered_data: itered_data.clone(),
                    errors,
                }
            }
        }
        types::Types::Collective => CollectorResponse {
            itered_data: itered_data.clone(),
            errors,
        },
        types::Types::Refference(data) => {
            if letter_char == "." {
                if data.on_dot {
                    errors.push(error::Error {
                        debug_message: "Yugirmnoa".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.on_dot = true;
                }
            } else if letter_char != " " {
                if data.on_dot {
                    data.on_dot = false;
                    data.chain.push(letter_char.to_string());
                } else if last_char == " "
                    && data.chain.len() != 0
                    && data.chain[data.chain.len() - 1] != ""
                {
                    if utils::is_opearators(letter_char) {
                        //itered_data.data.value = types::Types::Operators(types::OperatorType {
                        //    first: Box::new(itered_data.data.value.clone()),
                        //    operator_collect: letter_char.to_string(),
                        //    collecting_operator: true,
                        //    ..Default::default()
                        //});
                    } else {
                        errors.push(error::Error {
                            debug_message: "Fsteasthialvi".to_string(),
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
                            pos: mapper::defs::Cursor {
                                range_start: pos.clone(),
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else {
                    let chain_last_element = data.chain.len() - 1;
                    data.chain[chain_last_element] =
                        data.chain[chain_last_element].clone() + &letter_char;
                }
            }
            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Operator(data) => {
            if !data.first_filled {
                //First
                //TODO same as second litte bit different
                println!("FIRST");
            } else if !data.operator_collected {
                //Operator
                let is_opearator = types::operator_type::Operators::resolve_operator(
                    data.operator.clone(),
                    &(data.operator_collect.clone() + letter_char),
                );
                if is_opearator.is_err() {
                    if letter_char == " " {
                        data.operator_collected = true;
                    } else {
                        errors.push(error::Error {
                            debug_message: "Rndelle".to_string(),
                            title: error::errorList::error_s13.title.clone(),
                            code: error::errorList::error_s13.code,
                            message: error::errorList::error_s13.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s13.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char.to_string(),
                                }],
                            ),
                            pos: mapper::defs::Cursor {
                                range_start: pos.clone(),
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if let Ok(parsed_operator) = is_opearator {
                    data.operator_collect += letter_char;
                    if let types::operator_type::Operators::ComparisonType(_) = data.operator {
                        data.operator = parsed_operator;
                    } else {
                        data.operator = parsed_operator;
                    }
                }
            } else {
                //Second
                let mut will_be_itered = data.itered_cache.clone();
                data.second_is_not_null = true;
                let itered_child = collect(
                    &mut will_be_itered,
                    letter_char.clone(),
                    next_char.to_string().clone(),
                    last_char.to_string().clone(),
                    mapper::defs::CursorPosition(0, 0),
                );
                if itered_child.errors.len() != 0 {
                    for returned_error in itered_child.errors {
                        let mut edited = returned_error;
                        edited.pos.range_start.0 += pos.0;
                        edited.pos.range_start.1 += pos.1;
                        edited.pos.range_end.0 += pos.0;
                        edited.pos.range_end.1 += pos.1;
                        errors.push(edited);
                    }
                }
                if let types::Types::Operator(child_operator) =
                    itered_child.itered_data.data.value.clone()
                {
                    /*
                    println!(
                        "Collapse: {:#?} to {:#?}",
                        child_operator, data
                    );
                    */
                    if child_operator.operator == data.operator {
                        itered_data.data.value =
                            types::Types::Operator(types::operator_type::OperatorType {
                                cloaked: child_operator.cloaked,
                                first: Box::new(types::Types::Operator(
                                    types::operator_type::OperatorType {
                                        cloaked: data.cloaked,
                                        first: data.first.clone(),
                                        first_filled: true,
                                        second: child_operator.first,
                                        operator: data.operator.clone(),
                                        operator_collect: data.operator_collect.clone(),
                                        operator_collected: true,
                                        ..Default::default()
                                    },
                                )),
                                first_filled: true,
                                operator: child_operator.operator,
                                operator_collect: child_operator.operator_collect,
                                ..Default::default()
                            })
                    } else {
                        match data.operator.clone() {
                            types::operator_type::Operators::ComparisonType(_) => {
                                if child_operator.second == Box::new(types::Types::Null) {}
                                itered_data.data.value =
                                    types::Types::Operator(types::operator_type::OperatorType {
                                        cloaked: data.cloaked,
                                        first: Box::new(types::Types::Operator(
                                            types::operator_type::OperatorType {
                                                first_filled: true,
                                                cloaked: data.cloaked.clone(),
                                                first: data.first.clone(),
                                                second: child_operator.first.clone(),
                                                operator: data.operator.clone(),
                                                operator_collect: data.operator_collect.clone(),
                                                operator_collected: true,
                                                ..Default::default()
                                            },
                                        )),
                                        first_filled: true,
                                        operator: child_operator.operator,
                                        operator_collect: child_operator.operator_collect,
                                        ..Default::default()
                                    })
                            }
                            _ => {
                                data.second = Box::new(itered_child.itered_data.data.value.clone());
                                data.itered_cache = Box::new(itered_child.itered_data.clone());
                            }
                        }
                        //println!("dont Collapse: {:#?} to {:#?}", child_operator.clone().operator, data.operator);
                    }
                } else {
                    //println!("iterec: {:#?}", itered_child.itered_data);
                    data.itered_cache = Box::new(itered_child.itered_data.clone());
                    data.second = Box::new(itered_child.itered_data.data.value);
                }
            }

            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Array(data) => {
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

            let is_s_n = if last_entry != 0 && !data.collective[last_entry - 1].value.is_complete()
            {
                false
            } else {
                true
            };

            if letter_char == "[" {
                //print!("{:#?} {:#?}, {:#?} {:#?}",!data.child_start, is_s_n, pos, data);
            }
            if letter_char == "[" && !data.child_start && is_s_n {
                if !data.comma && last_entry != 0 {
                    errors.push(error::Error {
                        debug_message: "Tette".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
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
                        debug_message: "Hmlute".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else if data.comma {
                    errors.push(error::Error {
                        debug_message: "qrewrty".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
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
                        debug_message: "Tretra".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else if data.complete {
                    errors.push(error::Error {
                        debug_message: "Nonntkr".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
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
                        on_dot: true,
                        chain: vec![],
                    })
            } else if data.complete
                && types::logical_type::LogicalOpearators::is_opearator(letter_char)
                && is_s_n
            {
                itered_data.data.value =
                    types::Types::Operator(types::operator_type::OperatorType {
                        first: Box::new(types::Types::Array(data.clone())),
                        first_filled: true,
                        operator: types::operator_type::Operators::LogicalType(
                            types::logical_type::LogicalOpearators::Null,
                        ),
                        operator_collect: letter_char.to_string(),
                        ..Default::default()
                    });
            } else if data.complete
                && types::comparison_type::ComparisonOperators::is_opearator(letter_char)
                && is_s_n
            {
                itered_data.data.value =
                    types::Types::Operator(types::operator_type::OperatorType {
                        first: Box::new(types::Types::Array(data.clone())),
                        first_filled: true,
                        operator: types::operator_type::Operators::ComparisonType(
                            types::comparison_type::ComparisonOperators::Null,
                        ),
                        operator_collect: letter_char.to_string(),
                        ..Default::default()
                    });
            } else {
                if letter_char != " " {
                    //TODO IS THIS SAFE ?
                    data.comma = false;
                }
                let mut will_be_itered = if data.collective.len() == 0 {
                    variable::VariableCollector::default()
                } else {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *data.collective[data.collective.len() - 1].value.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                };

                let itered_array_vector = Box::new(collect(
                    &mut will_be_itered,
                    letter_char.clone(),
                    next_char.to_string().clone(),
                    last_char.to_string().clone(),
                    mapper::defs::CursorPosition(0, 0),
                ));

                if let types::Types::Array(ref adata) = itered_array_vector.itered_data.data.value {
                    if adata.complete {
                        data.child_start = false;
                    }
                }

                let itered_entry = match itered_array_vector.itered_data.data.value {
                    types::Types::Number(match_data) => types::array_type::ArrayEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::Number(match_data)),
                    },
                    types::Types::Double(match_data) => types::array_type::ArrayEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::Double(match_data)),
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
                    types::Types::Function => types::array_type::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
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

                if itered_array_vector.errors.len() != 0 {
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

                if data.collective.len() == 0 {
                    data.collective.push(itered_entry);
                } else {
                    data.collective[last_entry - 1] = itered_entry;
                }
            }

            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Cloak(data) => {
            let last_entry = data.clone().collective.len();

            let is_s_n = if last_entry != 0 && !data.collective[last_entry - 1].value.is_complete()
            {
                false
            } else {
                true
            };

            if letter_char == "(" && !data.child_start && is_s_n {
                if !data.comma && last_entry != 0 {
                    errors.push(error::Error {
                        debug_message: "Qutooe".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.child_start = true;
                    if last_entry == 0 {
                        data.collective.push(types::cloak_type::CloakEntry {
                            value_complete: false,
                            value: Box::new(types::Types::Cloak(
                                types::cloak_type::CloakType::default(),
                            )),
                        });
                    } else {
                        data.collective[last_entry - 1] = types::cloak_type::CloakEntry {
                            value_complete: false,
                            value: Box::new(types::Types::Cloak(
                                types::cloak_type::CloakType::default(),
                            )),
                        };
                    }
                }
            } else if letter_char == "," && !data.child_start && is_s_n {
                if data.complete {
                    errors.push(error::Error {
                        debug_message: "erwtes".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else if data.comma {
                    errors.push(error::Error {
                        debug_message: "qrtqw".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
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
                        .push(types::cloak_type::CloakEntry::default());
                }
            } else if letter_char == ")" && !data.child_start && is_s_n {
                println!("Complete the value");
                if data.comma {
                    errors.push(error::Error {
                        debug_message: "Okoe".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else if data.complete {
                    errors.push(error::Error {
                        debug_message: "Cporm".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
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
                        on_dot: true,
                        chain: vec![],
                    })
            } else if data.complete
                && types::logical_type::LogicalOpearators::is_opearator(letter_char)
                && is_s_n
            {
                itered_data.data.value =
                    types::Types::Operator(types::operator_type::OperatorType {
                        first: Box::new(types::Types::Cloak(data.clone())),
                        first_filled: true,
                        operator: types::operator_type::Operators::LogicalType(
                            types::logical_type::LogicalOpearators::Null,
                        ),
                        operator_collect: letter_char.to_string(),
                        ..Default::default()
                    });
            } else if data.complete
                && types::comparison_type::ComparisonOperators::is_opearator(letter_char)
                && is_s_n
            {
                itered_data.data.value =
                    types::Types::Operator(types::operator_type::OperatorType {
                        first: Box::new(types::Types::Cloak(data.clone())),
                        first_filled: true,
                        operator: types::operator_type::Operators::ComparisonType(
                            types::comparison_type::ComparisonOperators::Null,
                        ),
                        operator_collect: letter_char.to_string(),
                        ..Default::default()
                    });
            } else {
                if letter_char != " " {
                    //TODO IS THIS SAFE ?
                    data.comma = false;
                }

                let mut will_be_itered = if data.collective.len() == 0 {
                    variable::VariableCollector::default()
                } else {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *data.collective[data.collective.len() - 1].value.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                };

                let itered_cloak_vector = Box::new(collect(
                    &mut will_be_itered,
                    letter_char.clone(),
                    next_char.to_string().clone(),
                    last_char.to_string().clone(),
                    mapper::defs::CursorPosition(0, 0),
                ));

                if let types::Types::Cloak(ref adata) = itered_cloak_vector.itered_data.data.value {
                    if adata.complete {
                        data.child_start = false;
                    }
                }

                let itered_entry = match itered_cloak_vector.itered_data.data.value {
                    types::Types::Number(match_data) => types::cloak_type::CloakEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::Number(match_data)),
                    },
                    types::Types::Double(match_data) => types::cloak_type::CloakEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::Double(match_data)),
                    },
                    types::Types::Operator(match_data) => types::cloak_type::CloakEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Operator(match_data)),
                    },
                    types::Types::Bool(match_data) => types::cloak_type::CloakEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Bool(match_data)),
                    },
                    types::Types::String(match_data) => types::cloak_type::CloakEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::String(match_data)),
                    },
                    types::Types::Collective => types::cloak_type::CloakEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::Refference(_) => types::cloak_type::CloakEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::Array(match_data) => types::cloak_type::CloakEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Array(match_data)),
                    },
                    types::Types::Cloak(match_data) => types::cloak_type::CloakEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Cloak(match_data)),
                    },
                    types::Types::Function => types::cloak_type::CloakEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::FunctionCall(_) => types::cloak_type::CloakEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::Void => types::cloak_type::CloakEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::VariableType(match_data) => types::cloak_type::CloakEntry {
                        value_complete: true,
                        value: Box::new(types::Types::VariableType(match_data)),
                    },
                    types::Types::Null => types::cloak_type::CloakEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                };

                if itered_cloak_vector.errors.len() != 0 {
                    for returned_error in itered_cloak_vector.errors {
                        //errors.extend(itered_array_vector.errors);
                        let mut edited = returned_error;
                        edited.pos.range_start.0 += pos.0;
                        edited.pos.range_start.1 += pos.1;
                        edited.pos.range_end.0 += pos.0;
                        edited.pos.range_end.1 += pos.1;
                        errors.push(edited);
                    }
                }

                if data.collective.len() == 0 {
                    data.collective.push(itered_entry);
                } else {
                    data.collective[last_entry - 1] = itered_entry;
                }
            }

            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Function => CollectorResponse {
            itered_data: itered_data.clone(),
            errors,
        },
        types::Types::FunctionCall(data) => {
            let mut last_param = data.params.len();
            if last_param == 0 {
                data.params
                    .push(types::function_call::FunctionCallParameter::default());
                last_param = data.params.len();
            }

            let is_s_n =
                if last_param != 0 && data.params[last_param - 1].value.is_string_non_complete() {
                    false
                } else {
                    true
                };

            if letter_char == "," && is_s_n && !data.params[last_param - 1].value.is_array() {
                if data.params[last_param - 1].value.is_complete() {
                    data.comma = true;
                    data.params
                        .push(types::function_call::FunctionCallParameter::default())
                } else {
                    errors.push(error::Error {
                        debug_message: "Crusial".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone().skipChar(1),
                            range_end: pos.clone().skipChar(2),
                        },
                    });
                }
            } else if letter_char == ")" && is_s_n {
                if data.comma {
                    errors.push(error::Error {
                        debug_message: "Rmvoal".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone().skipChar(1),
                            range_end: pos.clone().skipChar(2),
                        },
                    });
                } else {
                    if data.params[last_param - 1].value.is_complete() || true {
                        data.complete = true
                    } else {
                        errors.push(error::Error {
                            debug_message: "Freede".to_string(),
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
                            pos: mapper::defs::Cursor {
                                range_start: pos.clone().skipChar(1),
                                range_end: pos.clone().skipChar(2),
                            },
                        });
                    }
                }
            } else {
                let mut last_param_value = variable::VariableCollector {
                    data: variable::Variable {
                        value: data.params[last_param - 1].value.clone(),
                        ..Default::default()
                    },
                    ..variable::VariableCollector::default()
                };

                data.comma = false;

                let itered_param_value = Box::new(collect(
                    &mut last_param_value,
                    letter_char.clone(),
                    next_char.to_string().clone(),
                    last_char.to_string().clone(),
                    mapper::defs::CursorPosition(0, 0),
                ));

                let _itered_entry = match itered_param_value.itered_data.data.value.clone() {
                    types::Types::Number(match_data) => types::array_type::ArrayEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::Number(match_data)),
                    },
                    types::Types::Double(match_data) => types::array_type::ArrayEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::Double(match_data)),
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
                    types::Types::Function => types::array_type::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
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

                if itered_param_value.errors.len() != 0 {
                    for returned_error in itered_param_value.errors {
                        //errors.extend(itered_array_vector.errors);
                        let mut edited = returned_error;
                        edited.pos.range_start.0 += pos.0;
                        edited.pos.range_start.1 += pos.1;
                        edited.pos.range_end.0 += pos.0;
                        edited.pos.range_end.1 += pos.1;
                        errors.push(edited);
                    }
                }
                data.params[last_param - 1].value = itered_param_value.itered_data.data.value;
            }

            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Void => CollectorResponse {
            itered_data: itered_data.clone(),
            errors,
        },
        types::Types::VariableType(data) => {
            let current_reliability = crate::utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable {
                data.value += letter_char;
            } else {
                if letter_char == " " && !data.value_complete {
                    if data.value == "false" || data.value == "true" {
                        itered_data.data.value = types::Types::Bool(types::bool_type::BoolType {
                            value: data.value.parse::<bool>().unwrap(),
                        });
                    } else {
                        data.value_complete = true;
                    }
                } else if letter_char == "\"" || letter_char == "'" {
                    itered_data.data.value = types::Types::String(types::string_type::StringType {
                        quote_type: letter_char.to_string(),
                        ..Default::default()
                    })
                } else {
                    //String 'i handle la
                    errors.push(error::Error {
                        debug_message: "Wole".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: current_reliability.found.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: mapper::defs::CursorPosition(
                                pos.0,
                                (pos.1 - itered_data.raw_value.len() as i64)
                                    + current_reliability.at as i64,
                            ),
                            range_end: mapper::defs::CursorPosition(
                                pos.0,
                                ((pos.1 - itered_data.raw_value.len() as i64)
                                    + current_reliability.at as i64)
                                    + 1,
                            ),
                        },
                    });
                }
            }
            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Null => {
            //let is_num = itered_data.raw_value.parse::<usize>().is_ok();
            if itered_data.raw_value == "" {
                if letter_char == "\"" || letter_char == "'" {
                    itered_data.data.value = types::Types::String(types::string_type::StringType {
                        quote_type: letter_char.to_string(),
                        ..Default::default()
                    })
                } else if (itered_data.raw_value.clone() + &letter_char)
                    .to_string()
                    .parse::<i32>()
                    .is_ok()
                {
                    itered_data.data.value = types::Types::Number(types::number_type::NumberType {
                        value: (itered_data.raw_value.clone() + &letter_char)
                            .parse::<usize>()
                            .unwrap(),
                        complete: false,
                    })
                } else if letter_char == "[" {
                    println!("Array Started");
                    itered_data.data.value = types::Types::Array(types::array_type::ArrayType {
                        layer_size: 0,
                        child_start: false,
                        complete: false,
                        comma: false,
                        collective: Vec::new(),
                    });
                } else if letter_char == "{" {
                    panic!("Collective is deprecated");
                } else if letter_char == "(" {
                    println!("Cloak Started");
                    itered_data.data.value = types::Types::Cloak(types::cloak_type::CloakType {
                        layer_size: 0,
                        child_start: false,
                        complete: false,
                        comma: false,
                        collective: Vec::new(),
                    });
                } else if letter_char != " " {
                    itered_data.data.value =
                        types::Types::VariableType(types::variable_type::VariableType {
                            value_complete: false,
                            value: itered_data.raw_value.clone() + &letter_char,
                        });
                }
            } else if letter_char != " " {
                /*
                if letter_char == "(" {
                    let current_reliability = crate::utils::reliable_name_range(
                        crate::utils::ReliableNameRanges::VariableName,
                        itered_data.raw_value.clone(),
                    );
                    if current_reliability.reliable {
                        itered_data.data.value =
                            types::Types::FunctionCall(types::function_call::FunctionCall {
                                name: itered_data.raw_value.to_string(),
                                name_pos: mapper::defs::Cursor {
                                    range_start: mapper::defs::CursorPosition(
                                        pos.0,
                                        pos.1 - itered_data.raw_value.len() as i64,
                                    ),
                                    range_end: mapper::defs::CursorPosition(pos.0, pos.1 - 1),
                                },
                                ..Default::default()
                            });
                    } else {
                        errors.push(error::Error {
                            debug_message: "Wole".to_string(),
                            title: error::errorList::error_s1.title.clone(),
                            code: error::errorList::error_s1.code,
                            message: error::errorList::error_s1.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s1.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: current_reliability.found.to_string(),
                                }],
                            ),
                            pos: mapper::defs::Cursor {
                                range_start: mapper::defs::CursorPosition(
                                    pos.0,
                                    (pos.1 - itered_data.raw_value.len() as i64)
                                        + current_reliability.at as i64,
                                ),
                                range_end: mapper::defs::CursorPosition(
                                    pos.0,
                                    ((pos.1 - itered_data.raw_value.len() as i64)
                                        + current_reliability.at as i64)
                                        + 1,
                                ),
                            },
                        });
                    }
                } else
                */
                if next_char == ";" || next_char == " " {
                    if itered_data.raw_value.parse::<i32>().is_ok() {
                        itered_data.data.value =
                            types::Types::Number(types::number_type::NumberType {
                                value: (itered_data.raw_value.clone() + &letter_char)
                                    .parse::<usize>()
                                    .unwrap(),
                                complete: false,
                            })
                    }
                }
                /*
                else {
                    //TODO catch unknown behaivour
                    errors.push(error::Error {
                        debug_message: "Eriea".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: itered_data.raw_value.clone(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos
                                .clone()
                                .popChar(itered_data.raw_value.clone().len() as i64),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
                */
                itered_data.raw_value += &letter_char;
            }
            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
    }
}
