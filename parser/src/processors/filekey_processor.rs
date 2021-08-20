use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::processors::value_processor;
use crate::syntax;
use alloc::boxed::Box;
use ellie_core::{defs, error, utils};

pub fn collect_filekey<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    let clone_parser = parser.clone();
    if let parser::Collecting::FileKey(ref mut file_key_data) = parser.current {
        if !file_key_data.key_name_collected {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable {
                if file_key_data.data.key_name.is_empty() {
                    file_key_data.data.key_name_location.range_start = parser.pos;
                }

                if (last_char == " " || last_char == "\n") && file_key_data.data.key_name != "" {
                    file_key_data.data.key_name = letter_char.to_string();
                }

                file_key_data.data.key_name += letter_char;
                file_key_data.data.key_name_location.range_end = parser.pos;
            } else {
                if letter_char == "=" && !file_key_data.data.key_name.is_empty() {
                    file_key_data.key_name_collected = true;
                } else if letter_char != " "
                    && (letter_char == "@" && file_key_data.data.key_name != "")
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "filekey_processor".to_string(),
                        debug_message: "098335bdd9a67f31a408095dbd5a842c".to_string(),
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
            }
        } else if letter_char == ";" && file_key_data.data.value.is_type_complete() {
            if clone_parser.check_key_keyword(file_key_data.data.key_name.clone()) {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "filekey_processor".to_string(),
                    debug_message: "cd0e16d885211d4939f52f57373a149e".to_string(),
                    title: error::errorList::error_s24.title.clone(),
                    code: error::errorList::error_s24.code,
                    message: error::errorList::error_s24.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s24.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: file_key_data.data.key_name.clone(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: file_key_data.data.key_name_location.range_start,
                        range_end: file_key_data
                            .data
                            .key_name_location
                            .range_end
                            .clone()
                            .skip_char(1),
                    },
                });
            }
            file_key_data.data.pos.range_end = parser.pos.clone().skip_char(1);
            file_key_data.value_collected = true;
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            let mut will_be_itered = syntax::variable::VariableCollector {
                data: syntax::variable::Variable {
                    value: file_key_data.data.value.clone(),
                    ..Default::default()
                },
                ..Default::default()
            };

            let itered_filekey_vector = Box::new(value_processor::collect_value(
                clone_parser,
                &mut will_be_itered,
                letter_char,
                next_char,
                last_char,
            ));

            if !itered_filekey_vector.errors.is_empty() {
                errors.extend(itered_filekey_vector.errors);
            }

            if file_key_data.data.value_location.is_zero() {
                file_key_data.data.value_location.range_start = parser.pos;
            }

            file_key_data.data.value = itered_filekey_vector.itered_data.data.value;
            file_key_data.data.value_location.range_end = parser.pos;
        }
    }
}
