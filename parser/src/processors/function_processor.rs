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
                        debug_message: "bdd3e2a2dc2276b18c767fe5d5ee13d3".to_string(),
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
                    debug_message: "5ef6508a77e5b8f486c30b6a2131efd2".to_string(),
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
                } else if letter_char == "*"
                    && function_data.data.parameters[last_entry - 1].name == ""
                {
                    function_data.data.parameters[last_entry - 1].multi_capture = true;
                } else if letter_char != " " {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "996acc567e100932ff8791af0a4029c5".to_string(),
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
                        debug_message: "e470c2cd0e13d6d50a2569ec315398e6".to_string(),
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
                        debug_message: "7928234ccb57c375123b4b78f820b5bd".to_string(),
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
                            debug_message: "e4b6e009f00b83bedf2e520ee5e442af".to_string(),
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
                        debug_message: "5581acd62154e09a64902e966e18cf65".to_string(),
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
                        debug_message: "bcb4b1c03dc9d64f86032f4816578d6d".to_string(),
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
                            debug_message: "16b9587916fc7006f6f0f15f438a6365".to_string(),
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
                } else if letter_char == ";"
                    && parser.options.parser_type == ellie_core::defs::ParserType::HeaderParser
                {
                    function_data.data.pos.range_end = parser.pos.clone().skip_char(1);
                    parser.collected.push(parser::Collecting::NativeFunction(
                        native_function::NativeFunction::from_runtime(function_data.data.clone()),
                    ));
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
                        debug_message: "a68c169ba66f55049419046fd59aef55".to_string(),
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
                            debug_message: "1c783724de027e4e5a76fe345a89c2a1".to_string(),
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
                    debug_message: "ca7edba09711cd7253a0dbe2e467e93f".to_string(),
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
