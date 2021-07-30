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
                        debug_message: "5536fed48b1f4ff55c4ab64f3e281d24".to_string(),
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
                    debug_message: "cbc7c6938b772a592bc9f242ac85075f".to_string(),
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
                        debug_message: "1afeae0bf345ddda8904cea545ac29c4".to_string(),
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
                        debug_message: "0b42c5d9c77e8ba3a8a44ee0d89d4cf0".to_string(),
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
                            debug_message: "ac91df4ff1be6c1470394135e2b60b62".to_string(),
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
                        debug_message: "457401abb7a42feb9a8583a9ffaeb208".to_string(),
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
                            debug_message: "468de346b53778b49b3e4c33fed5209c".to_string(),
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
                        debug_message: "3f896e65e91eaa4a0e42e17a204a81ce".to_string(),
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
                            debug_message: "26eeef5db43ef072c8124ddfbf44f4a6".to_string(),
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
            if parser_clone
                .check_keyword(functiondata.data.name.clone())
                .found
            {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "44fa6d45fe82e25a7d6c88e61a772753".to_string(),
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
            }
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
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
