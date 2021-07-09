use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::syntax;
use crate::syntax::definers::DefinerCollecting;
use ellie_core::{defs, error, utils};

/*
i8                    //generic type
array(array(i8), 5)   //i8 5 sized array
fn(i16, i32)::i8      //a function that takes i16 and i32 as parameter and returns i8 as result
cloak(i8, i32)        //a cloak that contains i8 as first parameter i32 as second
*/

pub fn collect_definer(
    parser: parser::Parser,
    type_data: &mut DefinerCollecting,
    errors: &mut Vec<error::Error>,
    letter_char: String,
    next_char: String,
    last_char: String,
) {
    match type_data {
        DefinerCollecting::GrowableArray(ref mut data) => {
            if letter_char == "(" && !data.bracket_inserted {
                data.bracket_inserted = true;
            } else if letter_char == ")" && data.rtype.is_definer_complete() {
                data.complete = true;
            } else {
                collect_definer(
                    parser,
                    &mut data.rtype,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                )
            }
        }
        DefinerCollecting::Array(ref mut data) => {
            if !data.typed {
                if letter_char == "(" && !data.bracket_inserted {
                    data.bracket_inserted = true;
                } else if letter_char == "," && data.rtype.is_definer_complete() {
                    data.typed = true;
                } else {
                    collect_definer(
                        parser,
                        &mut data.rtype,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                }
            } else if letter_char == ")" && data.len.complete {
                data.complete = true;
            } else {
                let mut emulated_collector_data = syntax::variable::VariableCollector {
                    data: syntax::variable::Variable {
                        value: syntax::types::Types::Integer(data.len.clone()),
                        rtype: syntax::definers::DefinerCollecting::Generic(
                            syntax::definers::GenericType {
                                rtype: "integer".to_string(),
                            },
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                let processed_data = crate::processors::value_processor::collect_value(
                    parser.clone(),
                    &mut emulated_collector_data,
                    &letter_char,
                    next_char,
                    last_char,
                );
                for i in processed_data.errors {
                    errors.push(i)
                }

                if !emulated_collector_data.data.value.is_integer() && letter_char != " " {
                    errors.push(error::Error {
                        scope: "definer_processor".to_string(),
                        debug_message: "212794d91d21d1ff26877b62a3d6e8b9".to_string(),
                        title: error::errorList::error_s20.title.clone(),
                        code: error::errorList::error_s20.code,
                        message: error::errorList::error_s20.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s20.message.clone(),
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                }

                if emulated_collector_data.data.value.is_type_complete() {
                    data.complete = true;
                }

                if let syntax::types::Types::Integer(e) = emulated_collector_data.data.value {
                    data.len = e;
                }
            }
        }
        DefinerCollecting::Generic(data) => {
            if letter_char == "(" && data.rtype.trim() == "fn" {
                *type_data = DefinerCollecting::Function(syntax::definers::FunctionType {
                    bracket_inserted: true,
                    params: vec![DefinerCollecting::Generic(
                        syntax::definers::GenericType::default(),
                    )],
                    ..Default::default()
                });
            } else if letter_char == "(" && data.rtype == "array" {
                *type_data = DefinerCollecting::Array(syntax::definers::ArrayType {
                    bracket_inserted: true,
                    ..Default::default()
                });
            } else if letter_char == "(" && data.rtype == "cloak" {
                *type_data = DefinerCollecting::Cloak(syntax::definers::CloakType {
                    bracket_inserted: true,
                    rtype: vec![DefinerCollecting::Generic(
                        syntax::definers::GenericType::default(),
                    )],
                    ..Default::default()
                });
            } else if letter_char == "(" && data.rtype == "growableArray" {
                *type_data =
                    DefinerCollecting::GrowableArray(syntax::definers::GrowableArrayType {
                        bracket_inserted: true,
                        ..Default::default()
                    });
            } else if letter_char != " "
                && (last_char == " " || last_char == "\n")
                && data.rtype.trim() != ""
            {
                errors.push(error::Error {
                    scope: "definer_processor".to_string(),
                    debug_message: "3f8abcbf79e2bb231fc36f97a20e10cc".to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char,
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            } else {
                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );
                if current_reliability.reliable {
                    data.rtype += &letter_char;
                    data.rtype = utils::trim_good(data.rtype.trim().to_string());
                } else if letter_char != " " {
                    errors.push(error::Error {
                        scope: "definer_processor".to_string(),
                        debug_message: "8185398c3aa6946810a8ec90b176c9dd".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char,
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                }
            }
        }
        DefinerCollecting::Function(data) => {
            if !data.parameter_collected {
                if letter_char == "(" && !data.bracket_inserted {
                    data.bracket_inserted = true;
                    data.params.push(DefinerCollecting::Generic(
                        syntax::definers::GenericType::default(),
                    ));
                } else if letter_char == ")" && data.bracket_inserted {
                    data.parameter_collected = true;
                } else if letter_char == "," && !data.params.is_empty() && !data.at_comma {
                    data.params.push(DefinerCollecting::Generic(
                        syntax::definers::GenericType::default(),
                    ));
                    data.at_comma = true;
                } else if data.params.is_empty() && data.bracket_inserted {
                    //This should have been filled If everything were right
                    errors.push(error::Error {
                        scope: "definer_processor".to_string(),
                        debug_message: "972eb9100eda54c2e94d8a7b290d9a2a".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char,
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else if data.bracket_inserted {
                    data.at_comma = false;
                    let len = data.params.clone().len();
                    collect_definer(
                        parser,
                        &mut data.params[if len == 0 { 0 } else { len - 1 }],
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    );

                    if data.params[if len == 0 { 0 } else { len - 1 }].is_definer_complete() {
                        data.complete = true;
                    }
                }
            } else if !data.return_typed {
                if data.return_keyword != 2 {
                    if letter_char != ":" {
                        errors.push(error::Error {
                            scope: "definer_processor".to_string(),
                            debug_message: "d8cf020edcaaeb379dad223651f07245".to_string(),
                            title: error::errorList::error_s1.title.clone(),
                            code: error::errorList::error_s1.code,
                            message: error::errorList::error_s1.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s1.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char,
                                }],
                            ),
                            pos: defs::Cursor {
                                range_start: parser.pos,
                                range_end: parser.pos.clone().skip_char(1),
                            },
                        });
                    }
                    data.return_keyword += 1;
                } else {
                    data.complete = true;
                    collect_definer(
                        parser,
                        &mut data.returning,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                }
            }
        }
        DefinerCollecting::Cloak(data) => {
            let length_of_childs = data.rtype.len();
            let is_complete = if length_of_childs == 0 {
                false
            } else {
                data.rtype[if length_of_childs == 1 {
                    0
                } else {
                    length_of_childs - 1
                }]
                .is_definer_complete()
            };

            if letter_char == "," && is_complete {
                data.rtype.push(DefinerCollecting::Generic(
                    syntax::definers::GenericType::default(),
                ));
            } else if letter_char == ")" && is_complete {
                data.complete = true;
            } else {
                collect_definer(
                    parser,
                    &mut data.rtype[if length_of_childs == 1 {
                        0
                    } else {
                        length_of_childs - 1
                    }],
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                )
            }
        }
        DefinerCollecting::Dynamic => {}
    }
}
