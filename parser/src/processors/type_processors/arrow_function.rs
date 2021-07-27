use crate::parser;
use crate::processors;
use crate::syntax::function;
use crate::syntax::{definers, types, variable};
use ellie_core::{defs, error, utils};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_arrow(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::ArrowFunction(ref mut functiondata) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype =
                definers::DefinerCollecting::Function(definers::FunctionType::default());
        }

        if !functiondata.parameter_wrote {
            if letter_char == "(" && !functiondata.param_bracket_opened {
                functiondata.param_bracket_opened = true;
            } else {
                let mut last_entry = functiondata.data.parameters.len();
                let typing_name = if last_entry == 0 {
                    true
                } else {
                    !functiondata.data.parameters[last_entry - 1].named
                };

                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );

                if typing_name {
                    if current_reliability.reliable
                        && ((last_char != " " && last_char != "\n")
                            || last_entry == 0
                            || functiondata.data.parameters[last_entry - 1]
                                .data
                                .name
                                .is_empty())
                    {
                        if last_entry == 0 {
                            functiondata
                                .data
                                .parameters
                                .push(function::FunctionParameterCollector::default());
                            last_entry = 1;
                        }
                        if functiondata.data.parameters[last_entry - 1]
                            .data
                            .name
                            .is_empty()
                        {
                            functiondata.data.parameters[last_entry - 1]
                                .data
                                .pos
                                .range_start = parser.pos;
                        }
                        if functiondata.data.parameters[last_entry - 1]
                            .data
                            .name_pos
                            .range_start
                            .is_zero()
                            && letter_char != " "
                        {
                            functiondata.data.parameters[last_entry - 1]
                                .data
                                .name_pos
                                .range_start = parser.pos;
                        }
                        functiondata.data.parameters[last_entry - 1]
                            .data
                            .name_pos
                            .range_end = parser.pos.clone().skip_char(1);
                        functiondata.data.parameters[last_entry - 1].data.name += letter_char
                    } else if letter_char == ":" {
                        if last_entry == 0
                            || functiondata.data.parameters[last_entry - 1]
                                .data
                                .name
                                .is_empty()
                        {
                            errors.push(error::Error {
                                scope: "arrow_function".to_string(),
                                debug_message: "dcb1cd3218380e566fc07f2b1d3394a4".to_string(),
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
                            functiondata.data.parameters[last_entry - 1].named = true;
                        }
                    } else if letter_char == ")" && last_entry == 0 {
                        functiondata.parameter_wrote = true;
                    } else if letter_char != " " {
                        errors.push(error::Error {
                            scope: "arrow_function".to_string(),
                            debug_message: "64f9bc7f3c29c50a1d66049f5af55d66".to_string(),
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
                } else if letter_char == ")"
                    && (last_entry == 0
                        || functiondata.data.parameters[last_entry - 1].child_brace == 0)
                {
                    if functiondata.has_dedup() {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "c032aa1f8595bbac77f4172017580d82".to_string(),
                            title: error::errorList::error_s10.title.clone(),
                            code: error::errorList::error_s10.code,
                            message: error::errorList::error_s10.message.clone(),
                            builded_message: error::BuildedError::build_from_string(
                                error::errorList::error_s10.message.clone(),
                            ),
                            pos: functiondata.data.parameters[last_entry - 1].data.name_pos,
                        });
                    }
                    if let definers::DefinerCollecting::Function(function) =
                        itered_data.data.rtype.clone()
                    {
                        if !functiondata.data.parameters[last_entry - 1]
                            .data
                            .rtype
                            .clone()
                            .same_as(function.params[last_entry - 1].clone())
                        {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "06842f99ae3ad30d9ada11b2c94a584c".to_string(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: function.params[last_entry - 1].raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: functiondata.data.parameters[last_entry - 1]
                                                .data
                                                .rtype
                                                .raw_name(),
                                        },
                                    ],
                                ),
                                pos: functiondata.data.parameters[last_entry - 1].data.type_pos,
                            });
                        }
                    }
                    if let definers::DefinerCollecting::Generic(name) =
                        &functiondata.data.parameters[last_entry - 1].data.rtype
                    {
                        if !parser.type_exists(name.rtype.clone()) {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "47f302ebe066999638c8ff93b0bbae7d".to_string(),
                                title: error::errorList::error_s6.title.clone(),
                                code: error::errorList::error_s6.code,
                                message: error::errorList::error_s6.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s6.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: name.rtype.clone(),
                                    }],
                                ),
                                pos: functiondata.data.parameters[last_entry - 1].data.type_pos,
                            });
                        }
                    }
                    functiondata.parameter_wrote = true;
                } else if letter_char == ","
                    && functiondata.data.parameters[last_entry - 1]
                        .data
                        .rtype
                        .is_definer_complete()
                {
                    if functiondata.has_dedup() {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "1032b531c867f84b7a88e511a13a83fa".to_string(),
                            title: error::errorList::error_s10.title.clone(),
                            code: error::errorList::error_s10.code,
                            message: error::errorList::error_s10.message.clone(),
                            builded_message: error::BuildedError::build_from_string(
                                error::errorList::error_s10.message.clone(),
                            ),
                            pos: functiondata.data.parameters[last_entry - 1].data.name_pos,
                        });
                    }
                    if let definers::DefinerCollecting::Generic(name) =
                        &functiondata.data.parameters[last_entry - 1].data.rtype
                    {
                        if !parser.type_exists(name.rtype.clone()) {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "e2a28753eb41447c805ee1cb63586ed6".to_string(),
                                title: error::errorList::error_s6.title.clone(),
                                code: error::errorList::error_s6.code,
                                message: error::errorList::error_s6.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s6.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: name.rtype.clone(),
                                    }],
                                ),
                                pos: functiondata.data.parameters[last_entry - 1].data.type_pos,
                            });
                        }
                    }
                    if let definers::DefinerCollecting::Function(function) =
                        itered_data.data.rtype.clone()
                    {
                        if !functiondata.data.parameters[last_entry - 1]
                            .data
                            .rtype
                            .clone()
                            .same_as(function.params[last_entry - 1].clone())
                        {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "a90d56520faab6000dd14ac46c777fc0".to_string(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: function.params[last_entry - 1].raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: functiondata.data.parameters[last_entry - 1]
                                                .data
                                                .rtype
                                                .raw_name(),
                                        },
                                    ],
                                ),
                                pos: functiondata.data.parameters[last_entry - 1].data.type_pos,
                            });
                        }
                    }
                    //If its type's comma dont stop collecting it
                    functiondata
                        .data
                        .parameters
                        .push(function::FunctionParameterCollector::default());
                } else {
                    if letter_char == ")" {
                        functiondata.data.parameters[last_entry - 1].child_brace -= 1;
                    } else if letter_char == "(" {
                        functiondata.data.parameters[last_entry - 1].child_brace += 1;
                    }
                    functiondata.data.parameters[last_entry - 1]
                        .data
                        .pos
                        .range_end = parser.pos.clone().skip_char(1);
                    if functiondata.data.parameters[last_entry - 1]
                        .data
                        .type_pos
                        .range_start
                        .is_zero()
                        && letter_char != " "
                    {
                        functiondata.data.parameters[last_entry - 1]
                            .data
                            .type_pos
                            .range_start = parser.pos;
                    }
                    functiondata.data.parameters[last_entry - 1]
                        .data
                        .type_pos
                        .range_end = parser.pos.clone().skip_char(1);
                    processors::definer_processor::collect_definer(
                        parser.clone(),
                        &mut functiondata.data.parameters[last_entry - 1].data.rtype,
                        errors,
                        letter_char.to_string(),
                        next_char,
                        last_char,
                    );
                }
            }
        } else if !functiondata.pointer_typed {
            if letter_char == ">" {
                functiondata.pointer_typed = true;
            } else if letter_char == "{" {
                if itered_data.data.dynamic {
                } else if let definers::DefinerCollecting::Function(function) =
                    itered_data.data.rtype.clone()
                {
                    let fndata_type = if functiondata.data.return_type.raw_name() == "" {
                        "void".to_string()
                    } else {
                        functiondata.data.return_type.raw_name()
                    };

                    if *function.returning.raw_name() != fndata_type {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "890387cfb6f16512c2c421b56ada70dd".to_string(),
                            title: error::errorList::error_s3.title.clone(),
                            code: error::errorList::error_s3.code,
                            message: error::errorList::error_s3.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s3.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "token1".to_string(),
                                        value: function.returning.raw_name(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_string(),
                                        value: fndata_type,
                                    },
                                ],
                            ),
                            pos: itered_data.data.type_pos,
                        });
                    }
                }

                functiondata.return_typed = true;
                functiondata.pointer_typed = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: "arrow_function".to_string(),
                    debug_message: "dfd305001bf124ed2b17d6336485a27c".to_string(),
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
        } else if !functiondata.return_typed {
            if letter_char == "{" && functiondata.data.return_type.is_definer_complete() {
                if let definers::DefinerCollecting::Generic(name) = &functiondata.data.return_type {
                    if !parser.type_exists(name.rtype.clone()) {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "078b46163313a98a6951a4a74fb9c393".to_string(),
                            title: error::errorList::error_s6.title.clone(),
                            code: error::errorList::error_s6.code,
                            message: error::errorList::error_s6.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s6.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: name.rtype.clone(),
                                }],
                            ),
                            pos: functiondata.data.return_pos,
                        });
                    }
                }
                if let definers::DefinerCollecting::Function(function) =
                    itered_data.data.rtype.clone()
                {
                    if !functiondata
                        .data
                        .return_type
                        .clone()
                        .same_as(*function.returning.clone())
                    {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "12625b7abdcfb640ef8410042e800e0d".to_string(),
                            title: error::errorList::error_s3.title.clone(),
                            code: error::errorList::error_s3.code,
                            message: error::errorList::error_s3.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s3.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "token1".to_string(),
                                        value: function.returning.raw_name(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_string(),
                                        value: functiondata.data.return_type.raw_name(),
                                    },
                                ],
                            ),
                            pos: functiondata.data.return_pos,
                        });
                    }
                }
                functiondata.return_typed = true;
            } else {
                if functiondata.data.return_pos.range_start.is_zero() && letter_char != " " {
                    functiondata.data.return_pos.range_start = parser.pos;
                }
                functiondata.data.return_pos.range_end = parser.pos;
                processors::definer_processor::collect_definer(
                    parser.clone(),
                    &mut functiondata.data.return_type,
                    errors,
                    letter_char.to_string(),
                    next_char,
                    last_char,
                );
            }
        } else if letter_char == "." && functiondata.complete {
            itered_data.data.value =
                types::Types::Reference(types::reference_type::ReferenceType {
                    reference: Box::new(itered_data.data.value.clone()),
                    chain: Vec::new(),
                    on_dot: false,
                });
            processors::type_processors::reference::collect_reference(
                parser.clone(),
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
            )
        } else if functiondata.complete
            && types::logical_type::LogicalOperators::is_logical_operator(letter_char)
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
        } else if functiondata.complete
            && types::comparison_type::ComparisonOperators::is_comparison_operator(letter_char)
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
        } else if functiondata.complete
            && types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(letter_char)
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
        } else if letter_char == "}" && functiondata.brace_count == 0 {
            functiondata.complete = true;
        } else {
            if letter_char == "{" {
                functiondata.brace_count += 1;
            } else if letter_char == "}" && functiondata.brace_count != 0 {
                functiondata.brace_count -= 1;
            }

            let code_letter = if last_char.clone() == "\n" || last_char.clone() == "\r" {
                last_char + letter_char //Make sure we get the lines correctly
            } else {
                letter_char.to_string()
            };
            functiondata.code += &code_letter;
        }
    }
}
