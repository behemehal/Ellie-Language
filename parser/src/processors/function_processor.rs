use crate::alloc::borrow::ToOwned;
use crate::alloc::boxed::Box;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::processors;
use crate::syntax::{definers, function, import_item, native_function, variable};
use ellie_core::{defs, error, utils};

pub fn collect_function<F, E>(
    parser: &mut parser::Parser<F, E>,
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
    let parser_clone = parser.clone();
    if let parser::Collecting::Function(ref mut function_data) = parser.current {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if !function_data.named {
            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n") || function_data.data.name.is_empty())
            {
                if function_data.data.name.is_empty() {
                    function_data.data.name_pos.range_start = parser.pos;
                }
                function_data.data.name += letter_char;
                function_data.data.name_pos.range_end = parser.pos.clone().skip_char(1);
            } else if letter_char == "(" && !function_data.data.name.is_empty() {
                if utils::is_reserved(&function_data.data.name) {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "47e4317b0d180d51b67a4eeee7ae0ea3".to_owned(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: function_data.data.name.clone(),
                            }],
                        ),
                        pos: function_data.data.name_pos,
                    });
                }
                function_data.named = true;
                function_data.data.parameters_pos.range_start = parser.pos;
            } else if letter_char != " " {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "104dc6061824862299e178c2984199a4".to_owned(),
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
        } else if !function_data.parameter_wrote {
            let mut last_entry = function_data.data.parameters.len();

            if last_entry == 0 {
                function_data
                    .data
                    .parameters
                    .push(function::FunctionParameter::default());
                last_entry = 1;
            }

            if !function_data.collecting_parameters.named {
                if current_reliability.reliable
                    && ((last_char != " " && last_char != "\n")
                        || function_data.data.parameters[last_entry - 1]
                            .name
                            .is_empty())
                {
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
                    function_data.data.parameters[last_entry - 1].name += letter_char;
                } else if letter_char == ":" {
                    function_data.collecting_parameters.named = true;
                } else if letter_char == ")"
                    && function_data.data.parameters[last_entry - 1]
                        .name
                        .is_empty()
                {
                    function_data.data.parameters = vec![];
                    function_data.parameter_wrote = true
                } else if letter_char == "*"
                    && function_data.data.parameters[last_entry - 1].name == ""
                {
                    function_data.data.parameters[last_entry - 1].multi_capture = true;
                } else if letter_char != " " {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "0937b19f551b7112f0c09aa19e549509".to_owned(),
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
                        debug_message: "259e48b8c3235d11f849b05a724b4f44".to_owned(),
                        title: error::errorList::error_s10.title.clone(),
                        code: error::errorList::error_s10.code,
                        message: error::errorList::error_s10.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s10.message.clone(),
                        ),
                        pos: function_data.data.parameters[last_entry - 1].name_pos,
                    });
                }

                if last_entry > 1 && function_data.data.parameters[last_entry - 2].multi_capture {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "7faef22a9a497077d4ef6e7c9e768b74".to_owned(),
                        title: error::errorList::error_s35.title.clone(),
                        code: error::errorList::error_s35.code,
                        message: error::errorList::error_s35.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s35.message.clone(),
                        ),
                        pos: function_data.data.parameters[last_entry - 1].pos,
                    });
                }

                if let definers::DefinerCollecting::Generic(name) =
                    &function_data.data.parameters[last_entry - 1].rtype
                {
                    if !parser_clone.type_exists(name.rtype.clone()) {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "3409d4c2d54d6433a6c236ff06bd07a1".to_owned(),
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
                        debug_message: "f0c7248c8ad0010f26b96a2d9915f016".to_owned(),
                        title: error::errorList::error_s10.title.clone(),
                        code: error::errorList::error_s10.code,
                        message: error::errorList::error_s10.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s10.message.clone(),
                        ),
                        pos: function_data.data.parameters[last_entry - 1].name_pos,
                    });
                }
                if last_entry > 1 && function_data.data.parameters[last_entry - 2].multi_capture {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "7232aedf8d26cf2deb5f34f8ff671ec3".to_owned(),
                        title: error::errorList::error_s35.title.clone(),
                        code: error::errorList::error_s35.code,
                        message: error::errorList::error_s35.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s35.message.clone(),
                        ),
                        pos: function_data.data.parameters[last_entry - 1].pos,
                    });
                }
                if let definers::DefinerCollecting::Generic(name) =
                    &function_data.data.parameters[last_entry - 1].rtype
                {
                    if !parser_clone.type_exists(name.rtype.clone()) {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "a76da27c9854786ab04d9436ee7edddd".to_owned(),
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
                    parser_clone,
                    &mut function_data.data.parameters[last_entry - 1].rtype,
                    errors,
                    letter_char.to_string(),
                    next_char,
                    last_char,
                );
            }
        } else if !function_data.return_typed {
            if !function_data.return_pointer_typed {
                if letter_char == ">" {
                    function_data.return_pointer_typed = true;
                } else if letter_char == ";"
                    && parser.options.parser_type == ellie_core::defs::ParserType::HeaderParser
                {
                    function_data.data.return_type =
                        definers::DefinerCollecting::Generic(definers::GenericType {
                            rtype: "void".to_owned(),
                        });
                    function_data.data.pos.range_end = parser.pos.clone().skip_char(1);
                    parser.collected.push(parser::Collecting::NativeFunction(
                        native_function::NativeFunction::from_runtime(function_data.data.clone()),
                    ));
                    parser.current = parser::Collecting::None;
                } else if letter_char == "{" {
                    function_data.data.return_type =
                        definers::DefinerCollecting::Generic(definers::GenericType {
                            rtype: "void".to_owned(),
                        });
                    function_data.return_typed = true;
                } else if letter_char != " " {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "f939273ba22273b83291a573bbabc242".to_owned(),
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
            } else if letter_char == ";"
                && function_data.data.return_type.is_definer_complete()
                && parser.options.parser_type == ellie_core::defs::ParserType::HeaderParser
            {
                function_data.data.pos.range_end = parser.pos.clone().skip_char(1);
                parser.collected.push(parser::Collecting::NativeFunction(
                    native_function::NativeFunction::from_runtime(function_data.data.clone()),
                ));
                parser.current = parser::Collecting::None;
            } else if letter_char == "{" && function_data.data.return_type.is_definer_complete() {
                if let definers::DefinerCollecting::Generic(name) = &function_data.data.return_type
                {
                    if !parser_clone.type_exists(name.rtype.clone()) {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "8e210d64ff7c789afc7f5f52344331ae".to_owned(),
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
                function_data.return_typed = true;
            } else {
                if function_data.data.return_pos.range_start.is_zero() && letter_char != " " {
                    function_data.data.return_pos.range_start = parser.pos;
                }
                function_data.data.return_pos.range_end = parser.pos;
                processors::definer_processor::collect_definer(
                    parser_clone,
                    &mut function_data.data.return_type,
                    errors,
                    letter_char.to_string(),
                    next_char.clone(),
                    last_char.clone(),
                );
            }
        } else if function_data.brace_count == 0 && letter_char == "}" {
            if parser_clone
                .check_keyword(function_data.data.name.clone(), false, false)
                .found
            {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "a05fb53e662a74ae38ccd27437898979".to_owned(),
                    title: error::errorList::error_s24.title.clone(),
                    code: error::errorList::error_s24.code,
                    message: error::errorList::error_s24.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s24.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: function_data.data.name.clone(),
                        }],
                    ),
                    pos: function_data.data.name_pos,
                });
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
                                                    .raw_name_with_extensions(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token1".to_owned(),
                                                value: function_data
                                                    .data
                                                    .return_type
                                                    .raw_name_with_extensions(),
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
                    debug_message: "replace_function_472".to_owned(),
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
            function_data.data.pos.range_end = parser.pos.clone().skip_char(1);
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
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
            errors.extend(child_parser_errors);
            function_data.code = Box::new(child_parser.to_raw());
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
