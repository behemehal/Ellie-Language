use crate::alloc::boxed::Box;
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
    options: defs::ParserOptions,
) {
    if let parser::Collecting::Function(ref mut functiondata) = parser.current {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if !functiondata.named {
            if current_reliability.reliable
                && (last_char != " " || functiondata.data.name.is_empty())
            {
                if functiondata.data.name.is_empty() {
                    functiondata.data.name_pos.range_start = parser.pos;
                }

                functiondata.data.name += letter_char;
                functiondata.data.name_pos.range_end = parser.pos;
            } else if letter_char == "(" {
                functiondata.named = true;
                functiondata.data.parameters_pos.range_start = parser.pos;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.clone() + "/function_processor",
                    debug_message: "9841eff3bfeba5c42edc6ec6bd6168be".to_string(),
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
                        range_end: parser.pos.clone().skipChar(1),
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
                    && (last_char != " "
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
                        scope: parser.scope.clone() + "/function_processor",
                        debug_message: "9841eff3bfeba5c42edc6ec6bd6168be".to_string(),
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
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                }
            } else if letter_char == ")"
                && (last_entry == 0
                    || functiondata.data.parameters[last_entry - 1].child_brace == 0)
            {
                if functiondata.has_dedup() {
                    errors.push(error::Error {
                        scope: parser.scope.clone() + "/function_processor",
                        debug_message: "9841eff3bfeba5c42edc6ec6bd6168be".to_string(),
                        title: error::errorList::error_s10.title.clone(),
                        code: error::errorList::error_s10.code,
                        message: error::errorList::error_s10.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s10.message.clone(),
                        ),
                        pos: functiondata.data.parameters[last_entry - 1].data.pos,
                    });
                }
                functiondata.parameter_wrote = true;
            } else if letter_char == ","
                && functiondata.data.parameters[last_entry - 1]
                    .data
                    .rtype
                    .is_definer_complete()
            {
                //If its type's comma dont stop collecting it
                if functiondata.has_dedup() {
                    errors.push(error::Error {
                        scope: parser.scope.clone() + "/function_processor",
                        debug_message: "9841eff3bfeba5c42edc6ec6bd6168be".to_string(),
                        title: error::errorList::error_s10.title.clone(),
                        code: error::errorList::error_s10.code,
                        message: error::errorList::error_s10.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s10.message.clone(),
                        ),
                        pos: functiondata.data.parameters[last_entry - 1].data.pos,
                    });
                }
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
                    .range_end = parser.pos.clone().skipChar(1);
                processors::definer_processor::collect_definer(
                    &mut functiondata.data.parameters[last_entry - 1].data.rtype,
                    errors,
                    letter_char.to_string(),
                    parser.pos,
                    next_char,
                    last_char,
                    options,
                );
            }
        } else if !functiondata.return_typed {
            if !functiondata.return_pointer_typed {
                if letter_char == ">" {
                    functiondata.return_pointer_typed = true;
                } else if letter_char == "{" {
                    functiondata.data.return_type = Box::new(definers::DefinerCollecting::Generic(
                        definers::GenericType {
                            rtype: "void".to_string(),
                        },
                    ));
                    functiondata.return_typed = true;
                } else if letter_char != " " {
                    errors.push(error::Error {
                        scope: parser.scope.clone() + "/function_processor",
                        debug_message: "9841eff3bfeba5c42edc6ec6bd6168be".to_string(),
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
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                }
            } else if letter_char == "{" && functiondata.data.return_type.is_definer_complete() {
                functiondata.return_typed = true;
            } else {
                processors::definer_processor::collect_definer(
                    &mut functiondata.data.return_type,
                    errors,
                    letter_char.to_string(),
                    parser.pos,
                    next_char.clone(),
                    last_char.clone(),
                    options,
                );
            }
        } else if functiondata.brace_count == 0 && letter_char == "}" {
            functiondata.data.inside_code = functiondata.code.collected.clone();
            functiondata.code = Box::new(parser::Parser::default()); //Empty the cache
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if letter_char == "{" {
                functiondata.brace_count += 1;
            } else if letter_char == "}" && functiondata.brace_count != 0 {
                functiondata.brace_count -= 1;
            }

            let mut child_parser = functiondata.code.clone();
            child_parser.options = parser.options.clone();
            let mut child_parser_errors: Vec<error::Error> = Vec::new();
            parser::iterator::iter(
                &mut child_parser,
                &mut child_parser_errors,
                letter_char,
                next_char,
                last_char,
            );

            for i in child_parser_errors {
                let mut edited = i;
                edited.pos.range_start.0 += parser.pos.0;
                edited.pos.range_start.1 += parser.pos.1;
                edited.pos.range_end.0 += parser.pos.0;
                edited.pos.range_end.1 += parser.pos.1;
                errors.push(edited);
            }
            functiondata.code = child_parser;
        }
    }
}
