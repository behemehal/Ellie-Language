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
                                path: parser.options.path.clone(),
                                scope: "arrow_function".to_string(),
                                debug_message: "dec82c2a7ffaea3a35df2c0b9de84f25".to_string(),
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
                            path: parser.options.path.clone(),
                            scope: "arrow_function".to_string(),
                            debug_message: "483788a1b5f2bb54553d6bc8a6926708".to_string(),
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
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "6997924b2b9f4e80d336fdcf2628f7a4".to_string(),
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
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "8689e48d8027ea161726df2c01ac71da".to_string(),
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
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "5c222edf0781f4c912255811873500ae".to_string(),
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
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "95a870fe3dcda9722ff78b7390d88f7e".to_string(),
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
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "5753df7d8cdf7491dbbeb313f3237bd1".to_string(),
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
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "3406c5450a5e98bdfa45c786ae2a2901".to_string(),
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
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "2839128742cff95544972c982d5bbcfc".to_string(),
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
                    path: parser.options.path.clone(),
                    scope: "arrow_function".to_string(),
                    debug_message: "466c49e4ce4627e27148861eba7fdac8".to_string(),
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
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "80d5d1020afdb8317241b026b1a519e3".to_string(),
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
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "978b476526240f978494bedab7dcbe40".to_string(),
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
                types::Types::Reference(types::reference_type::ReferenceTypeCollector {
                    data: types::reference_type::ReferenceType {
                        reference: Box::new(itered_data.data.value.clone()),
                        chain: Vec::new(),
                    },
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
