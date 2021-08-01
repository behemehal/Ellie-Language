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
    if let types::Types::ArrowFunction(ref mut function_data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype =
                definers::DefinerCollecting::Function(definers::FunctionType::default());
        }

        if !function_data.parameter_wrote {
            if letter_char == "(" && !function_data.param_bracket_opened {
                function_data.param_bracket_opened = true;
            } else {
                let mut last_entry = function_data.data.parameters.len();
                let typing_name = if last_entry == 0 {
                    true
                } else {
                    !function_data.collecting_parameters.named
                };

                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );

                if typing_name {
                    if current_reliability.reliable
                        && ((last_char != " " && last_char != "\n")
                            || last_entry == 0
                            || function_data.data.parameters[last_entry - 1]
                                .name
                                .is_empty())
                    {
                        if last_entry == 0 {
                            function_data
                                .data
                                .parameters
                                .push(function::FunctionParameter::default());
                            last_entry = 1;
                        }
                        if function_data.data.parameters[last_entry - 1]
                            .name
                            .is_empty()
                        {
                            function_data.data.parameters[last_entry - 1]
                                .pos
                                .range_start = parser.pos;
                        }
                        if function_data.data.parameters[last_entry - 1]
                            .name_pos
                            .range_start
                            .is_zero()
                            && letter_char != " "
                        {
                            function_data.data.parameters[last_entry - 1]
                                .name_pos
                                .range_start = parser.pos;
                        }
                        function_data.data.parameters[last_entry - 1]
                            .name_pos
                            .range_end = parser.pos.clone().skip_char(1);
                        function_data.data.parameters[last_entry - 1].name += letter_char
                    } else if letter_char == ":" {
                        if last_entry == 0
                            || function_data.data.parameters[last_entry - 1]
                                .name
                                .is_empty()
                        {
                            errors.push(error::Error {
                                scope: "arrow_function".to_string(),
                                debug_message: "2d1a21845fc76a63e9b4a25088595287".to_string(),
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
                            function_data.collecting_parameters.named = true;
                        }
                    } else if letter_char == ")" && last_entry == 0 {
                        function_data.parameter_wrote = true;
                    } else if letter_char != " " {
                        errors.push(error::Error {
                            scope: "arrow_function".to_string(),
                            debug_message: "3ca3d4effefe03e1e77247a48a79aa62".to_string(),
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
                    && (last_entry == 0 || function_data.collecting_parameters.child_brace == 0)
                {
                    if function_data.has_dedup() {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "28d74502957806fd4c2cf68c0ce0e286".to_string(),
                            title: error::errorList::error_s10.title.clone(),
                            code: error::errorList::error_s10.code,
                            message: error::errorList::error_s10.message.clone(),
                            builded_message: error::BuildedError::build_from_string(
                                error::errorList::error_s10.message.clone(),
                            ),
                            pos: function_data.data.parameters[last_entry - 1].name_pos,
                        });
                    }
                    if let definers::DefinerCollecting::Function(function) =
                        itered_data.data.rtype.clone()
                    {
                        if !function_data.data.parameters[last_entry - 1]
                            .rtype
                            .clone()
                            .same_as(function.params[last_entry - 1].clone())
                        {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "70f8600a9301a5404f074a27fdfa66b6".to_string(),
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
                                            value: function_data.data.parameters[last_entry - 1]
                                                .rtype
                                                .raw_name(),
                                        },
                                    ],
                                ),
                                pos: function_data.data.parameters[last_entry - 1].type_pos,
                            });
                        }
                    }
                    if let definers::DefinerCollecting::Generic(name) =
                        &function_data.data.parameters[last_entry - 1].rtype
                    {
                        if !parser.type_exists(name.rtype.clone()) {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "59d9ab213a5a6fb4fef49f376919063b".to_string(),
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
                                pos: function_data.data.parameters[last_entry - 1].type_pos,
                            });
                        }
                    }
                    function_data.parameter_wrote = true;
                } else if letter_char == ","
                    && function_data.data.parameters[last_entry - 1]
                        .rtype
                        .is_definer_complete()
                {
                    if function_data.has_dedup() {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "95668d6d2e4540dd31ac0c7034f78c71".to_string(),
                            title: error::errorList::error_s10.title.clone(),
                            code: error::errorList::error_s10.code,
                            message: error::errorList::error_s10.message.clone(),
                            builded_message: error::BuildedError::build_from_string(
                                error::errorList::error_s10.message.clone(),
                            ),
                            pos: function_data.data.parameters[last_entry - 1].name_pos,
                        });
                    }
                    if let definers::DefinerCollecting::Generic(name) =
                        &function_data.data.parameters[last_entry - 1].rtype
                    {
                        if !parser.type_exists(name.rtype.clone()) {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "2a8384d65736f39c75db357b9a0d1cd1".to_string(),
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
                                pos: function_data.data.parameters[last_entry - 1].type_pos,
                            });
                        }
                    }
                    if let definers::DefinerCollecting::Function(function) =
                        itered_data.data.rtype.clone()
                    {
                        if !function_data.data.parameters[last_entry - 1]
                            .rtype
                            .clone()
                            .same_as(function.params[last_entry - 1].clone())
                        {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "a56e13b4587e56831a3f5e6415197877".to_string(),
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
                                            value: function_data.data.parameters[last_entry - 1]
                                                .rtype
                                                .raw_name(),
                                        },
                                    ],
                                ),
                                pos: function_data.data.parameters[last_entry - 1].type_pos,
                            });
                        }
                    }
                    //If its type's comma dont stop collecting it
                    function_data
                        .data
                        .parameters
                        .push(function::FunctionParameter::default());
                    function_data.collecting_parameters =
                        function::FunctionParameterCollector::default()
                } else {
                    if letter_char == ")" {
                        function_data.collecting_parameters.child_brace -= 1;
                    } else if letter_char == "(" {
                        function_data.collecting_parameters.child_brace += 1;
                    }
                    function_data.data.parameters[last_entry - 1].pos.range_end =
                        parser.pos.clone().skip_char(1);
                    if function_data.data.parameters[last_entry - 1]
                        .type_pos
                        .range_start
                        .is_zero()
                        && letter_char != " "
                    {
                        function_data.data.parameters[last_entry - 1]
                            .type_pos
                            .range_start = parser.pos;
                    }
                    function_data.data.parameters[last_entry - 1]
                        .type_pos
                        .range_end = parser.pos.clone().skip_char(1);
                    processors::definer_processor::collect_definer(
                        parser.clone(),
                        &mut function_data.data.parameters[last_entry - 1].rtype,
                        errors,
                        letter_char.to_string(),
                        next_char,
                        last_char,
                    );
                }
            }
        } else if !function_data.pointer_typed {
            if letter_char == ">" {
                function_data.pointer_typed = true;
            } else if letter_char == "{" {
                if itered_data.data.dynamic {
                } else if let definers::DefinerCollecting::Function(function) =
                    itered_data.data.rtype.clone()
                {
                    let fndata_type = if function_data.data.return_type.raw_name() == "" {
                        "void".to_string()
                    } else {
                        function_data.data.return_type.raw_name()
                    };

                    if *function.returning.raw_name() != fndata_type {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "c03c5dd09bbb45cfbe4e9818d4780c98".to_string(),
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

                function_data.return_typed = true;
                function_data.pointer_typed = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: "arrow_function".to_string(),
                    debug_message: "464ee2c8660a313ec7049749e9c34af2".to_string(),
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
        } else if !function_data.return_typed {
            if letter_char == "{" && function_data.data.return_type.is_definer_complete() {
                if let definers::DefinerCollecting::Generic(name) = &function_data.data.return_type
                {
                    if !parser.type_exists(name.rtype.clone()) {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "f6e60f509652c3bffda9f8648f867f40".to_string(),
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
                            pos: function_data.data.return_pos,
                        });
                    }
                }
                if let definers::DefinerCollecting::Function(function) =
                    itered_data.data.rtype.clone()
                {
                    if !function_data
                        .data
                        .return_type
                        .clone()
                        .same_as(*function.returning.clone())
                    {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "785d82ecc6a7c6f6dfbb12d577f093d8".to_string(),
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
                                        value: function_data.data.return_type.raw_name(),
                                    },
                                ],
                            ),
                            pos: function_data.data.return_pos,
                        });
                    }
                }
                function_data.return_typed = true;
            } else {
                if function_data.data.return_pos.range_start.is_zero() && letter_char != " " {
                    function_data.data.return_pos.range_start = parser.pos;
                }
                function_data.data.return_pos.range_end = parser.pos;
                processors::definer_processor::collect_definer(
                    parser.clone(),
                    &mut function_data.data.return_type,
                    errors,
                    letter_char.to_string(),
                    next_char,
                    last_char,
                );
            }
        } else if letter_char == "." && function_data.complete {
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
        } else if function_data.complete
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
        } else if function_data.complete
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
        } else if function_data.complete
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
        } else if letter_char == "}" && function_data.brace_count == 0 {
            function_data.complete = true;
        } else {
            if letter_char == "{" {
                function_data.brace_count += 1;
            } else if letter_char == "}" && function_data.brace_count != 0 {
                function_data.brace_count -= 1;
            }

            let code_letter = if last_char.clone() == "\n" || last_char.clone() == "\r" {
                last_char + letter_char //Make sure we get the lines correctly
            } else {
                letter_char.to_string()
            };
            function_data.code += &code_letter;
        }
    }
}
