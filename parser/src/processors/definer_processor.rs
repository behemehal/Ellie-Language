use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::syntax;
use crate::syntax::definers::DefinerCollecting;
use ellie_core::{defs, error, utils};

/*
i8                    //generic type
array(array(i8), 5)   //i8 5 sized array
fn(i16, i32)::i8      //a function that takes i16 and i32 as parameter and returns i8 as result
cloak(i8, i32)        //a cloak that contains i8 as first parameter i32 as second

*/

<<<<<<< HEAD
pub fn collect_definer(
=======
pub fn collect(
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
    type_data: &mut DefinerCollecting,
    errors: &mut Vec<error::Error>,
    letter_char: String,
    pos: defs::CursorPosition,
    next_char: String,
    last_char: String,
<<<<<<< HEAD
    options: defs::ParserOptions,
=======
    options: defs::ParserOptions
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
) {
    match type_data {
        DefinerCollecting::DynamicArray(ref mut data) => {
            if letter_char == "(" && !data.bracket_inserted {
                data.bracket_inserted = true;
            } else if letter_char == ")" && data.rtype.is_definer_complete() {
                data.complete = true;
            } else {
                collect_definer(
                    &mut data.rtype,
                    errors,
                    letter_char,
                    pos,
                    next_char,
                    last_char,
<<<<<<< HEAD
                    options,
=======
                    options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
                        &mut data.rtype,
                        errors,
                        letter_char,
                        pos,
                        next_char,
                        last_char,
<<<<<<< HEAD
                        options,
=======
                        options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
                    )
                }
            } else {
                let mut emulated_collector_data = syntax::variable::VariableCollector {
<<<<<<< HEAD
                    rtype: syntax::definers::DefinerCollecting::Generic(
                        syntax::definers::GenericType {
                            rtype: "usize".to_string(),
                        },
                    ),
=======
                    r#type: syntax::definers::DefinerCollecting::Generic(syntax::definers::GenericType {
                        r#type: "usize".to_string()
                    }),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
                    data: syntax::variable::Variable {

                        value: data.len.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                let processed_data = crate::processors::value_processor::collect_value(
                    &mut emulated_collector_data,
                    &letter_char,
                    next_char,
                    last_char,
                    pos,
<<<<<<< HEAD
                    options,
=======
                    options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
                );
                for i in processed_data.errors {
                    errors.push(i)
                }
                if emulated_collector_data.data.value.is_type_complete() {
                    data.complete = true;
                }
                data.len = emulated_collector_data.data.value;
            }
        }
        DefinerCollecting::Generic(data) => {
<<<<<<< HEAD
            if letter_char == "(" && data.rtype.trim() == "fn" {
                *type_data = DefinerCollecting::Function(syntax::definers::FunctionType {
                    bracket_inserted: true,
                    params: vec![DefinerCollecting::Generic(
                        syntax::definers::GenericType::default(),
                    )],
                    ..Default::default()
                });
            } else if letter_char == "(" && data.rtype == "array" {
=======
            if letter_char == "(" && data.r#type.trim() == "fn" {
                *type_data = DefinerCollecting::Function(syntax::definers::FunctionType {
                    bracket_inserted: true,
                    params: vec![DefinerCollecting::Generic(syntax::definers::GenericType::default())],
                    ..Default::default()
                });
            } else if letter_char == "(" && data.r#type == "array" {
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
                *type_data = DefinerCollecting::Array(syntax::definers::ArrayType {
                    bracket_inserted: true,
                    ..Default::default()
                });
<<<<<<< HEAD
            } else if letter_char == "(" && data.rtype == "cloak" {
                *type_data = DefinerCollecting::Cloak(syntax::definers::CloakType {
                    bracket_inserted: true,
                    rtype: vec![DefinerCollecting::Generic(
                        syntax::definers::GenericType::default(),
                    )],
                    ..Default::default()
                });
            } else if letter_char == "(" && data.rtype == "dynamicArray" {
=======
            } else if letter_char == "(" && data.r#type == "cloak" {
                *type_data = DefinerCollecting::Cloak(syntax::definers::CloakType {
                    bracket_inserted: true,
                    r#type: vec![DefinerCollecting::Generic(syntax::definers::GenericType::default())],
                    ..Default::default()
                });
            } else if letter_char == "(" && data.r#type == "dynamicArray" {
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
                *type_data = DefinerCollecting::DynamicArray(syntax::definers::DynamicArrayType {
                    bracket_inserted: true,
                    ..Default::default()
                });
            } else if letter_char != " " && last_char == " " && data.rtype.trim() != "" {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/definer_processor.rs:103".to_string(),
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
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
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
                        debug_message: "./parser/src/processors/definer_processor.rs:129"
                            .to_string(),
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
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
            }
        }
        DefinerCollecting::Function(data) => {
            if !data.parameter_collected {
                if letter_char == "(" && !data.bracket_inserted {
                    data.bracket_inserted = true;
<<<<<<< HEAD
                    data.params.push(DefinerCollecting::Generic(
                        syntax::definers::GenericType::default(),
                    ));
                } else if letter_char == ")" && data.bracket_inserted {
                    data.parameter_collected = true;
                } else if letter_char == "," && !data.params.is_empty() && !data.at_comma {
                    data.params.push(DefinerCollecting::Generic(
                        syntax::definers::GenericType::default(),
                    ));
=======
                    data.params
                        .push(DefinerCollecting::Generic(syntax::definers::GenericType::default()));
                } else if letter_char == ")" && data.bracket_inserted {
                    data.parameter_collected = true;
                } else if letter_char == "," && !data.params.is_empty() && !data.at_comma {
                    data.params
                        .push(DefinerCollecting::Generic(syntax::definers::GenericType::default()));
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
                    data.at_comma = true;
                } else if data.params.is_empty() && data.bracket_inserted {
                    //This should have been filled If everything were right
                    errors.push(error::Error {
                        debug_message: "./parser/src/processors/definer_processor.rs:164"
                            .to_string(),
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
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else if data.bracket_inserted {
                    data.at_comma = false;
                    let len = data.params.clone().len();
                    collect_definer(
                        &mut data.params[if len == 0 { 0 } else { len - 1 }],
                        errors,
                        letter_char,
                        pos,
                        next_char,
                        last_char,
<<<<<<< HEAD
                        options,
=======
                        options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
                    );

                    if data.params[if len == 0 { 0 } else { len - 1 }].is_definer_complete() {
                        data.complete = true;
                    }
                }
            } else if !data.return_typed {
                if data.return_keyword != 2 {
                    if letter_char != ":" {
                        errors.push(error::Error {
                            debug_message: "./parser/src/processors/definer_processor.rs:202"
                                .to_string(),
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
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                    data.return_keyword += 1;
                } else {
                    data.complete = true;
                    collect_definer(
                        &mut data.returning,
                        errors,
                        letter_char,
                        pos,
                        next_char,
                        last_char,
<<<<<<< HEAD
                        options,
=======
                        options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
                    )
                }
            }
        }
        DefinerCollecting::Cloak(data) => {
<<<<<<< HEAD
            let length_of_childs = data.rtype.len();
=======
            let length_of_childs = data.r#type.len();
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                data.rtype.push(DefinerCollecting::Generic(
                    syntax::definers::GenericType::default(),
                ));
=======
                data.r#type.push(DefinerCollecting::Generic(syntax::definers::GenericType::default()));
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
            } else if letter_char == ")" && is_complete {
                data.complete = true;
            } else {
                collect_definer(
                    &mut data.rtype[if length_of_childs == 1 {
                        0
                    } else {
                        length_of_childs - 1
                    }],
                    errors,
                    letter_char,
                    pos,
                    next_char,
                    last_char,
<<<<<<< HEAD
                    options,
                )
            }
        }
=======
                    options
                )
            }
        },
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
        DefinerCollecting::Dynamic => {}
    }
}
