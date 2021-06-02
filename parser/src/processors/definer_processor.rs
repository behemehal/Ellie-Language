use crate::alloc::format;
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

pub fn collect_definer(
    type_data: &mut DefinerCollecting,
    errors: &mut Vec<error::Error>,
    letter_char: String,
    pos: defs::CursorPosition,
    next_char: String,
    last_char: String,
    options: defs::ParserOptions,
) {
    match type_data {
        DefinerCollecting::GrowableArray(ref mut data) => {
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
                    options,
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
                        options,
                    )
                }
            } else if letter_char == ")" && data.len.is_type_complete() {
                data.complete = true;
            } else {
                let mut emulated_collector_data = syntax::variable::VariableCollector {
                    rtype: syntax::definers::DefinerCollecting::Generic(
                        syntax::definers::GenericType {
                            rtype: "usize".to_string(),
                        },
                    ),
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
                    options,
                );
                for i in processed_data.errors {
                    errors.push(i)
                }

                if emulated_collector_data.data.value.is_type_complete()  {
                    data.complete = true;
                }

                data.len = emulated_collector_data.data.value;
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
            } else if letter_char != " " && last_char == " " && data.rtype.trim() != "" {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/definer_processor.rs:0".to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: 
