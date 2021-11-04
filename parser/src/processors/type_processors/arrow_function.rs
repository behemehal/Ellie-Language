use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::processors;
use crate::syntax::function;
use crate::syntax::{definers, import_item, types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error, utils};

pub fn collect_arrow<F, E>(
    parser: parser::Parser<F, E>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Sized,
{
    if let types::Types::ArrowFunction(ref mut function_data) = itered_data.data.value {
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
                                scope: "arrow_function".to_owned(),
                                debug_message: "a619b0c5f8750b77b3abe6ee3b4f74a5".to_owned(),
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
                            function_data.collecting_parameters.named = true;
                        }
                    } else if letter_char == ")" && last_entry == 0 {
                        function_data.parameter_wrote = true;
                    } else if letter_char != " " {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: "arrow_function".to_owned(),
                            debug_message: "11265926187906d37352347816ddb95a".to_owned(),
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
                } else if letter_char == ")"
                    && (last_entry == 0 || function_data.collecting_parameters.child_brace == 0)
                {
                    if function_data.has_dedup() {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "99fb1d7a170333c0d0c8d2140e0e4b07".to_owned(),
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
                                debug_message: "c20e73c999f8354d5b4b4242a9d01d16".to_owned(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_owned(),
                                            value: function.params[last_entry - 1].raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_owned(),
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
                                debug_message: "84e036c4986d98b0c6a010a38d356ed4".to_owned(),
                                title: error::errorList::error_s6.title.clone(),
                                code: error::errorList::error_s6.code,
                                message: error::errorList::error_s6.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s6.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
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
                            debug_message: "1cfec0528cc801249a02f33cfbe3ce12".to_owned(),
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
                                debug_message: "fca509e48e19961401ed878a68aa1480".to_owned(),
                                title: error::errorList::error_s6.title.clone(),
                                code: error::errorList::error_s6.code,
                                message: error::errorList::error_s6.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s6.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
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
                                debug_message: "4e690232a41a8a1114c24a97ab820f6a".to_owned(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_owned(),
                                            value: function.params[last_entry - 1].raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_owned(),
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
                        "void".to_owned()
                    } else {
                        function_data.data.return_type.raw_name()
                    };

                    if *function.returning.raw_name() != fndata_type {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "349178099b05f29aa5443377e7804bfe".to_owned(),
                            title: error::errorList::error_s3.title.clone(),
                            code: error::errorList::error_s3.code,
                            message: error::errorList::error_s3.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s3.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "token1".to_owned(),
                                        value: function.returning.raw_name(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_owned(),
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
                    scope: "arrow_function".to_owned(),
                    debug_message: "237d7210b8bdbe4b1e6918bad28e0ccf".to_owned(),
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
        } else if !function_data.return_typed {
            if letter_char == "{" && function_data.data.return_type.is_definer_complete() {
                if let definers::DefinerCollecting::Generic(name) = &function_data.data.return_type
                {
                    if !parser.type_exists(name.rtype.clone()) {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "92e5610216abd307d1996e8cf5ec08c8".to_owned(),
                            title: error::errorList::error_s6.title.clone(),
                            code: error::errorList::error_s6.code,
                            message: error::errorList::error_s6.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s6.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
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
                            debug_message: "a402b5f8f7b8ace16650648f390563d3".to_owned(),
                            title: error::errorList::error_s3.title.clone(),
                            code: error::errorList::error_s3.code,
                            message: error::errorList::error_s3.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s3.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "token1".to_owned(),
                                        value: function.returning.raw_name(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_owned(),
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
        } else if function_data.complete
            && ellie_core::utils::is_extended(letter_char, next_char).is_some()
        {
            match ellie_core::utils::is_extended(letter_char, next_char).unwrap() {
                ellie_core::utils::FoundExtended::Reference => {
                    itered_data.data.value =
                        types::Types::Reference(types::reference_type::ReferenceTypeCollector {
                            data: types::reference_type::ReferenceType {
                                reference_pos: itered_data.data.value_pos,
                                reference: Box::new(itered_data.data.value.clone()),
                                chain: Vec::new(),
                            },
                            root_available: true,
                            on_dot: false,
                            complete: false,
                            last_entry: itered_data.data.value.clone().to_definer(),
                        });
                }
                ellie_core::utils::FoundExtended::BracketReference => {
                    itered_data.data.value = types::Types::BracketReference(
                        types::bracket_reference_type::BracketReferenceCollector {
                            complete: false,
                            data: types::bracket_reference_type::BracketReference {
                                pos: defs::Cursor {
                                    range_start: parser.pos,
                                    ..Default::default()
                                },
                                target: itered_data.data.value.clone().to_definer(),
                            },
                            ..Default::default()
                        },
                    );
                }
                ellie_core::utils::FoundExtended::LogicalOperator => {
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
                }
                ellie_core::utils::FoundExtended::ComparisonOperator => {
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
                }
                ellie_core::utils::FoundExtended::ArithmeticOperator => {
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
                ellie_core::utils::FoundExtended::AssignmentOperator => {
                    itered_data.data.value =
                        types::Types::Operator(types::operator_type::OperatorTypeCollector {
                            data: types::operator_type::OperatorType {
                                first: Box::new(itered_data.data.value.clone()),
                                operator: types::operator_type::Operators::AssignmentType(
                                    types::assignment_type::AssignmentOperators::Null,
                                ),
                                ..Default::default()
                            },
                            operator_collect: letter_char.to_string(),
                            first_filled: true,
                            ..Default::default()
                        });
                }
                ellie_core::utils::FoundExtended::FunctionCall => todo!(),
            }
        } else if letter_char == "}" && function_data.brace_count == 0 {
            if itered_data.data.dynamic {
                itered_data.data.rtype =
                    definers::DefinerCollecting::Function(definers::FunctionType {
                        complete: true,
                        params: function_data
                            .data
                            .parameters
                            .clone()
                            .into_iter()
                            .map(|param| param.rtype)
                            .collect::<Vec<_>>(),
                        returning: Box::new(function_data.data.return_type.clone()),
                        return_typed: true,
                        return_keyword: 3,
                        parameter_collected: true,
                        bracket_inserted: true,
                        at_comma: false,
                    })
            }
            //Filter out temporary items
            let mut filtered_items: Vec<parser::Collecting> = Vec::new();
            let mut ret_found = false;
            for item in function_data.code.collected.clone() {
                match item {
                    parser::Collecting::ImportItem(e) => {
                        if e.from_path != "<temporary>" {
                            filtered_items.push(parser::Collecting::ImportItem(e))
                        }
                    }
                    parser::Collecting::Ret(return_item) => {
                        if !ret_found {
                            ret_found = true;
                            if return_item.value.clone().to_definer()
                                != function_data.data.return_type
                            {
                                panic!("{:#?}", return_item.value.clone().to_definer());
                                errors.push(error::Error {
                                    path: parser.options.path.clone(),
                                    scope: parser.scope.scope_name.clone(),
                                    debug_message: "replace_getter_121".to_owned(),
                                    title: error::errorList::error_s3.title.clone(),
                                    code: error::errorList::error_s3.code,
                                    message: error::errorList::error_s3.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s3.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token2".to_owned(),
                                                value: return_item
                                                    .value
                                                    .clone()
                                                    .to_definer()
                                                    .raw_name_with_extensions_with_hashes(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token1".to_owned(),
                                                value: function_data
                                                    .data
                                                    .return_type
                                                    .raw_name_with_extensions_with_hashes(),
                                            },
                                        ],
                                    ),
                                    pos: return_item.pos,
                                });
                            }
                        }
                        filtered_items.push(parser::Collecting::Ret(return_item))
                    }
                    e => filtered_items.push(e),
                }
            }

            if !ret_found && function_data.data.return_type.raw_name() != "void" {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "replace_getter_159".to_owned(),
                    title: error::errorList::error_s3.title.clone(),
                    code: error::errorList::error_s3.code,
                    message: error::errorList::error_s3.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s3.message.clone(),
                        vec![
                            error::ErrorBuildField {
                                key: "token2".to_owned(),
                                value: "void".to_owned(),
                            },
                            error::ErrorBuildField {
                                key: "token1".to_owned(),
                                value: function_data.data.return_type.raw_name_with_extensions(),
                            },
                        ],
                    ),
                    pos: function_data.data.return_pos,
                });
            }
            function_data.data.inside_code = filtered_items;

            function_data.complete = true;
        } else {
            if letter_char == "{" {
                function_data.brace_count += 1;
            } else if letter_char == "}" && function_data.brace_count != 0 {
                function_data.brace_count -= 1;
            }

            let mut child_parser = function_data.code.clone().to_no_resolver_parser();

            if function_data.code.pos.is_zero() {
                //Make sure upper scope imported once

                for item in parser.collected.clone() {
                    //Import variables as temporary for syntax support, we will remove them after collecting complete
                    child_parser.collected.push(parser::Collecting::ImportItem(
                        import_item::ImportItem {
                            resolution_id: 0,
                            from_import: 0,
                            from_path: "<temporary>".to_owned(),
                            public: true,
                            item: Box::new(item),
                        },
                    ));
                }

                for param in function_data.data.parameters.clone() {
                    //Import variables as temporary for syntax support, we will remove them after collecting complete
                    child_parser.collected.push(parser::Collecting::ImportItem(
                        import_item::ImportItem {
                            resolution_id: 0,
                            from_import: 0,
                            from_path: "<temporary>".to_owned(),
                            public: true,
                            item: Box::new(parser::Collecting::Variable(if param.multi_capture {
                                variable::VariableCollector {
                                    data: variable::Variable {
                                        pos: param.pos,
                                        value_pos: param.type_pos,
                                        name_pos: param.name_pos,
                                        name: param.name,
                                        rtype: definers::DefinerCollecting::GrowableArray(
                                            definers::GrowableArrayType {
                                                rtype: Box::new(param.rtype),
                                                ..Default::default()
                                            },
                                        ),
                                        public: true,
                                        hash: "not_required".to_owned(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }
                            } else {
                                variable::VariableCollector {
                                    data: variable::Variable {
                                        pos: param.pos,
                                        value_pos: param.type_pos,
                                        name_pos: param.name_pos,
                                        rtype: param.rtype,
                                        name: param.name,
                                        public: true,
                                        hash: "not_required".to_owned(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }
                            })),
                        },
                    ));
                }
            }

            child_parser.options = parser.options.clone();
            child_parser.options.parser_type = defs::ParserType::RawParser;
            child_parser.pos = parser.pos;
            child_parser.scope.scope_name = "core/function_processor".to_owned();
            child_parser.current = function_data.code.current.clone();
            child_parser.keyword_catch = function_data.code.keyword_catch.clone();
            child_parser.keyword_cache = function_data.code.keyword_cache.clone();

            let mut child_parser_errors: Vec<error::Error> = Vec::new();
            parser::iterator::iter(
                &mut child_parser,
                &mut child_parser_errors,
                letter_char,
                next_char,
                last_char,
            );
            for i in child_parser_errors {
                errors.push(i);
            }

            function_data.code = Box::new(child_parser.to_raw());
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
