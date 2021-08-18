use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::processors;
use crate::syntax::{definers, function, native_function};
use ellie_core::{defs, error, utils};

pub fn collect_function<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
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
                        debug_message: "4e62b0cf65b69200f3f5e4d3e65f92c8".to_string(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
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
                    debug_message: "cd03f17e4356d0802aea85e16729e320".to_string(),
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
                } else if letter_char == "*" && function_data.data.parameters[last_entry - 1].name == "" {
                    function_data.data.parameters[last_entry - 1].multi_capture = true;
                } else if letter_char != " " {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "72024563586e0e66a903a76e6fd18d25".to_string(),
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
                        debug_message: "1bcc6e8edbe38ae5d2c5823074f06af7".to_string(),
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
                        debug_message: "1bcc6e8edbe38ae5d2c5823074f06af7".to_string(),
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
                            debug_message: "e3e157927e9be3a18918bca9567dd9e4".to_string(),
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
                        debug_message: "c787fa7906ebaae259366a275fc372dd".to_string(),
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
                        debug_message: "1bcc6e8edbe38ae5d2c5823074f06af7".to_string(),
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
                            debug_message: "7182ea3fef72b6f8ec4298a847e0a8ed".to_string(),
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
                } else if letter_char == ";" && parser.options.parser_type == ellie_core::defs::ParserType::HeaderParser {
                    function_data.data.pos.range_end = parser.pos.clone().skip_char(1);
                    parser.collected.push(parser::Collecting::NativeFunction(native_function::NativeFunction::from_runtime(function_data.data.clone())));
                    parser.current = parser::Collecting::None;
                } else if letter_char == "{" {
                    function_data.data.return_type =
                        definers::DefinerCollecting::Generic(definers::GenericType {
                            rtype: "void".to_string(),
                        });
                    function_data.return_typed = true;
                } else if letter_char != " " {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "96060dfc63b93ef83bfaa63a5f273428".to_string(),
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
            } else if letter_char == ";" && function_data.data.return_type.is_definer_complete() && parser.options.parser_type == ellie_core::defs::ParserType::HeaderParser {
                function_data.data.pos.range_end = parser.pos.clone().skip_char(1);
                parser.collected.push(parser::Collecting::NativeFunction(native_function::NativeFunction::from_runtime(function_data.data.clone())));
                parser.current = parser::Collecting::None;
            } else if letter_char == "{" && function_data.data.return_type.is_definer_complete() {
                if let definers::DefinerCollecting::Generic(name) = &function_data.data.return_type
                {
                    if !parser_clone.type_exists(name.rtype.clone()) {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "e1e72685c7fcf085ab83ab4acfbdcfdf".to_string(),
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
                .check_keyword(function_data.data.name.clone())
                .found
            {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "edadc49f00612195288df5b60ff24849".to_string(),
                    title: error::errorList::error_s24.title.clone(),
                    code: error::errorList::error_s24.code,
                    message: error::errorList::error_s24.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s24.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: function_data.data.name.clone(),
                        }],
                    ),
                    pos: function_data.data.name_pos,
                });
            }
            function_data.data.pos.range_end = parser.pos.clone().skip_char(1);
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
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
