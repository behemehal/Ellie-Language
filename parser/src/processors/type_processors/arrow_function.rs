use crate::parser;
use crate::processors;
use crate::syntax::function;
use crate::syntax::{types, variable};
use ellie_core::{defs, error, utils};

use alloc::string::{String, ToString};
<<<<<<< HEAD
<<<<<<< HEAD
use alloc::vec;
=======
<<<<<<< HEAD
use alloc::vec;
=======
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
use alloc::vec::Vec;
=======
>>>>>>> FFI
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_arrow(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
<<<<<<< HEAD
<<<<<<< HEAD
    options: defs::ParserOptions,
=======
<<<<<<< HEAD
    options: defs::ParserOptions,
=======
    options: defs::ParserOptions
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
    options: defs::ParserOptions,
>>>>>>> FFI
) {
    if let types::Types::ArrowFunction(ref mut functiondata) = itered_data.data.value {
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
                        && (last_char != " "
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
                        functiondata.data.parameters[last_entry - 1].data.name += letter_char
                    } else if letter_char == ":" {
                        if last_entry == 0
                            || functiondata.data.parameters[last_entry - 1]
                                .data
                                .name
                                .is_empty()
                        {
                            errors.push(error::Error {
                                debug_message:
                                    "./parser/src/processors/type_processors/arrow_function.rs:59"
                                        .to_string(),
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
                                    range_start: pos,
                                    range_end: pos.clone().skipChar(1),
                                },
                            });
                        } else {
                            functiondata.data.parameters[last_entry - 1].named = true;
                        }
                    } else if letter_char == ")" && last_entry == 0 {
                        functiondata.parameter_wrote = true;
                    } else if letter_char != " " {
                        errors.push(error::Error {
                            debug_message:
                                "./parser/src/processors/type_processors/arrow_function.rs:82"
                                    .to_string(),
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
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if letter_char == ")"
                    && (last_entry == 0
                        || functiondata.data.parameters[last_entry - 1].child_brace == 0)
                {
                    functiondata.parameter_wrote = true;
                } else if letter_char == ","
                    && functiondata.data.parameters[last_entry - 1]
                        .data
                        .rtype
                        .is_definer_complete()
                {
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
                    processors::definer_processor::collect_definer(
                        &mut functiondata.data.parameters[last_entry - 1].data.rtype,
                        errors,
                        letter_char.to_string(),
                        pos,
                        next_char,
                        last_char,
<<<<<<< HEAD
<<<<<<< HEAD
                        options,
=======
<<<<<<< HEAD
                        options,
=======
                        options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
                        options,
>>>>>>> FFI
                    );
                }
            }
        } else if !functiondata.pointer_typed {
            if letter_char == ">" {
                functiondata.pointer_typed = true;
            } else if letter_char == "{" {
                functiondata.return_typed = true;
                functiondata.pointer_typed = true;
            } else if letter_char != " " {
                errors.push(error::Error {
<<<<<<< HEAD
<<<<<<< HEAD
=======
<<<<<<< HEAD
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
                    debug_message: "./parser/src/processors/type_processors/arrow_function.rs:143"
                        .to_string(),
=======
                    debug_message: "./parser/src/processors/type_processors/arrow_function.rs:143" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
=======
                    debug_message: "./parser/src/processors/type_processors/arrow_function.rs:143"
                        .to_string(),
>>>>>>> FFI
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
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            }
        } else if !functiondata.return_typed {
            if letter_char == "{" && functiondata.data.return_type.is_definer_complete() {
                functiondata.return_typed = true;
            } else {
                processors::definer_processor::collect_definer(
                    &mut functiondata.data.return_type,
                    errors,
                    letter_char.to_string(),
                    pos,
                    next_char,
                    last_char,
<<<<<<< HEAD
<<<<<<< HEAD
                    options,
=======
<<<<<<< HEAD
                    options,
=======
                    options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
                    options,
>>>>>>> FFI
                );
            }
        } else if letter_char == "}" && functiondata.brace_count == 0 {
            functiondata.complete = true;
        } else {
            if letter_char == "{" {
                functiondata.brace_count += 1;
            } else if letter_char == "}" && functiondata.brace_count > 0 {
                functiondata.brace_count -= 1;
            }
            functiondata.inside_code_string += letter_char;
<<<<<<< HEAD
<<<<<<< HEAD
            let mut child_parser =
                parser::Parser::new(functiondata.inside_code_string.clone(), options);
=======
<<<<<<< HEAD
            let mut child_parser =
                parser::Parser::new(functiondata.inside_code_string.clone(), options);
=======
            let mut child_parser = parser::Parser::new(
                functiondata.inside_code_string.clone(),
                options
            );
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
            let mut child_parser =
                parser::Parser::new(functiondata.inside_code_string.clone(), options);
>>>>>>> FFI
            child_parser.pos = pos;
            let mapped = child_parser.map();
            for i in mapped.syntax_errors {
                errors.push(i)
            }
            functiondata.data.inside_code = mapped.items;
        }
    }
}
