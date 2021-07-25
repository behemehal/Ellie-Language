use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::syntax;
use crate::processors::value_processor;
use alloc::boxed::Box;
use ellie_core::{defs, error, utils};

pub fn collect_filekey(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String
) {
    let clone_parser = parser.clone();
    if let parser::Collecting::FileKey(ref mut filekeydata) = parser.current {
        if !filekeydata.keyname_collected {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable {
                if filekeydata.data.keyname.is_empty() {
                    filekeydata.data.keyname_location.range_start = parser.pos;
                }

                if (last_char == " " || last_char == "\n") && filekeydata.data.keyname != "" {
                    filekeydata.data.keyname = letter_char.to_string();
                }

                filekeydata.data.keyname += letter_char;
                filekeydata.data.keyname_location.range_end = parser.pos;
            } else {
                if letter_char == "=" && !filekeydata.data.keyname.is_empty() {
                    filekeydata.keyname_collected = true;
                } else if letter_char != " " && (letter_char == "@" && filekeydata.data.keyname != ""){
                    errors.push(error::Error {
                        scope: "filekey_processor".to_string(),
                        debug_message: "replace_filekey_42".to_string(),
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
        } else if letter_char == ";" && filekeydata.data.value.is_type_complete() {
            if clone_parser.check_keyword(filekeydata.data.keyname.clone()).found {
                errors.push(error::Error {
                    scope: "filekey_processor".to_string(),
                    debug_message: "replace_filekey_64".to_string(),
                    title: error::errorList::error_s24.title.clone(),
                    code: error::errorList::error_s24.code,
                    message: error::errorList::error_s24.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s24.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: filekeydata.data.keyname.clone(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: filekeydata.data.keyname_location.range_start,
                        range_end: filekeydata.data.keyname_location.range_end.clone().skip_char(1)
                    },
                });
            }
            if utils::is_reserved(&filekeydata.data.keyname) {
                errors.push(error::Error {
                    scope: "filekey_processor".to_string(),
                    debug_message: "replace_filekey_processor83".to_string(),
                    title: error::errorList::error_s21.title.clone(),
                    code: error::errorList::error_s21.code,
                    message: error::errorList::error_s21.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s21.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: filekeydata.data.keyname.clone(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: filekeydata.data.keyname_location.range_start,
                        range_end: filekeydata.data.keyname_location.range_end.clone().skip_char(1)
                    },
                });
            }
            filekeydata.value_collected = true;
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            let mut will_be_itered = syntax::variable::VariableCollector {
                data: syntax::variable::Variable {
                    value: filekeydata.data.value.clone(),
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

            if filekeydata.data.value_location.is_zero() {
                filekeydata.data.value_location.range_start = parser.pos;
            }
            
            filekeydata.data.value = itered_filekey_vector.itered_data.data.value;
            filekeydata.data.value_location.range_end = parser.pos;
        }


    }
}
