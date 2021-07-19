use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::processors;
use crate::syntax::{definers, function};
use ellie_core::{defs, error, utils};

pub fn collect_function(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    _options: defs::ParserOptions,
) {
    let parser_clone = parser.clone();
    if let parser::Collecting::Function(ref mut functiondata) = parser.current {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if !functiondata.named {
            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n") || functiondata.data.name.is_empty())
            {
                if functiondata.data.name.is_empty() {
                    functiondata.data.name_pos.range_start = parser.pos;
                }
                functiondata.data.name += letter_char;
                functiondata.data.name_pos.range_end = parser.pos.clone().skip_char(1);
            } else if letter_char == "(" && !functiondata.data.name.is_empty() {
                if utils::is_reserved(&functiondata.data.name) {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "4c4dad058656236a396403e329d43513".to_string(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: functiondata.data.name.clone(),
                            }],
                        ),
                        pos: functiondata.data.name_pos,
                    });
                }
                functiondata.named = true;
                functiondata.data.parameters_pos.range_start = parser.pos;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "24987be57e3bd88954c4ceae08c00616".to_string(),
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
        } else if !functiondata.parameter_wrote {
            let mut last_entry = functiondata.data.parameters.len();

            if last_entry == 0 {
                functiondata
                    .data
                    .parameters
                    .push(function::FunctionParameterCollector::default());
                last_entry = 1;
            }

            if !functiondata.data.parameters[last_entry - 1].named {
                if current_reliability.reliable
                    && ((last_char != " " && last_char != "\n")
                        || functiondata.data.parameters[last_entry - 1]
                            .data
                            .name
                            .is_empty())
                {
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
                    functiondata.data.parameters[last_entry - 1].data.name += letter_char;
                } else if letter_char == ":" {
                    functiondata.data.parameters[last_entry - 1].named = true;
                } else if letter_char == ")"
                    && functiondata.data.parameters[last_entry - 1]
                        .data
                        .name
                        .is_empty()
                {
                    functiondata.data.parameters = vec![];
                    functiondata.parameter_wrote = true
                } else if letter_char != " " {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "3c58594479e38a46db3b30b858a5dc54".to_string(),
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
                        debug_message: "a29c1d923ee0fa8f0457545d98c1b9e8".to_string(),
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
                    if !parser_clone.type_exists(name.rtype.clone()) {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "0079a3151ed15d3e11b0087689745c94".to_string(),
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
                        debug_message: "80f95028263e73b27f744cbab6aa3c30".to_string(),
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
                    if !parser_clone.type_exists(name.rtype.clone()) {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "0800e56d54a8ec7fe6a93d585c61ed9f".to_string(),
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
                    parser_clone,
                    &mut functiondata.data.parameters[last_entry - 1].data.rtype,
                    errors,
                    letter_char.to_string(),
                    next_char,
                    last_char,
                );
            }
        } else if !functiondata.return_typed {
            if !functiondata.return_pointer_typed {
                if letter_char == ">" {
                    functiondata.return_pointer_typed = true;
                } else if letter_char == "{" {
                    functiondata.data.return_type =
                        definers::DefinerCollecting::Generic(definers::GenericType {
                            rtype: "void".to_string(),
                        });
                    functiondata.return_typed = true;
                } else if letter_char != " " {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "70ace3cdb208252b3dd64621d0ab4f32".to_string(),
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
            } else if letter_char == "{" && functiondata.data.return_type.is_definer_complete() {
                if let definers::DefinerCollecting::Generic(name) = &functiondata.data.return_type {
                    if !parser_clone.type_exists(name.rtype.clone()) {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "2ee8c474f4f11f5337941a540973078e".to_string(),
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
                functiondata.return_typed = true;
            } else {
                if functiondata.data.return_pos.range_start.is_zero() && letter_char != " " {
                    functiondata.data.return_pos.range_start = parser.pos;
                }
                functiondata.data.return_pos.range_end = parser.pos;
                processors::definer_processor::collect_definer(
                    parser_clone,
                    &mut functiondata.data.return_type,
                    errors,
                    letter_char.to_string(),
                    next_char.clone(),
                    last_char.clone(),
                );
            }
        } else if functiondata.brace_count == 0 && letter_char == "}" {
            //functiondata.data.inside_code = functiondata.code.collected.clone();
            //functiondata.code = Box::new(parser::Parser::default()); //Empty the cache
            let fn_exists = parser_clone.check_keyword(functiondata.data.name.clone());
            if fn_exists.found {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "f741cb86484448b27c4f7992aa0598cf".to_string(),
                    title: error::errorList::error_s24.title.clone(),
                    code: error::errorList::error_s24.code,
                    message: error::errorList::error_s24.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s24.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: functiondata.data.name.clone(),
                        }],
                    ),
                    pos: functiondata.data.name_pos,
                });
            } else {
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            }
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
