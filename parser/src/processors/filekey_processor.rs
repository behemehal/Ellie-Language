use crate::alloc::borrow::ToOwned;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::processors::value_processor;
use crate::syntax;
use ellie_core::{defs, error, utils};

pub fn collect_filekey<F, E>(
    parser: &mut parser::Parser<F, E>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Copy + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Copy
        + Sized,
{
    let clone_parser = parser.clone();
    if let parser::Collecting::FileKey(ref mut file_key_data) = parser.current {
        if !file_key_data.key_name_collected {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable {
                if file_key_data.data.key_name.is_empty() && letter_char != " " {
                    file_key_data.data.key_name_location.range_start = parser.pos;
                }

                if (last_char == " " || last_char == "\n") && file_key_data.data.key_name != "" {
                    file_key_data.data.key_name = letter_char.to_string();
                }

                file_key_data.data.key_name += letter_char;
                file_key_data.data.key_name_location.range_end = parser.pos.clone().skip_char(1);
            } else {
                if letter_char == "=" && !file_key_data.data.key_name.is_empty() {
                    file_key_data.key_name_collected = true;
                } else if letter_char != " "
                    && (letter_char == "@" && file_key_data.data.key_name != "")
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "filekey_processor".to_owned(),
                        debug_message: "3d4c4cd85f4938aacb80d801b5d16eb5".to_owned(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
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
                    scope: "filekey_processor".to_owned(),
                    debug_message: "caff038ac8073fd1bf16f5b69d231be4".to_owned(),
                    title: error::errorList::error_s24.title.clone(),
                    code: error::errorList::error_s24.code,
                    message: error::errorList::error_s24.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s24.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
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
            if file_key_data.data.key_name == "parserWarn"
                || file_key_data.data.key_name == "parserAbort"
                || file_key_data.data.key_name == "parserInfo"
            {
                match file_key_data.data.value.clone() {
                    syntax::types::Types::String(e) => {
                        #[cfg(feature = "std")]
                        std::println!(
                            "\u{001b}{}[ParserMessage - {}]\u{001b}[0m: {} - {}:{}:{}",
                            if file_key_data.data.key_name == "parserWarn" {
                                "[33m"
                            } else if file_key_data.data.key_name == "parserAbort" {
                                "[31m"
                            } else if file_key_data.data.key_name == "parserInfo" {
                                "[34m"
                            } else {
                                "[34m"
                            },
                            file_key_data.data.key_name.split("parser").last().unwrap(),
                            e.data.value,
                            parser.options.path,
                            parser.pos.0 + 1,
                            0
                        );
                    }
                    _ => {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: "filekey_processor".to_owned(),
                            debug_message: "64d2aaabee8d8a245f1b6d6c0d9ef2ea".to_owned(),
                            title: error::errorList::error_s40.title.clone(),
                            code: error::errorList::error_s40.code,
                            message: error::errorList::error_s40.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s40.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: file_key_data
                                        .data
                                        .value
                                        .clone()
                                        .to_definer()
                                        .raw_name_with_extensions(),
                                }],
                            ),
                            pos: file_key_data.data.value_location,
                        });
                    }
                }
            } else {
                parser.collected.push(parser.current.clone());
            }
            parser.current = parser::Collecting::None;
        } else {
            let mut will_be_itered = syntax::variable::VariableCollector {
                data: syntax::variable::Variable {
                    value: file_key_data.data.value.clone(),
                    ..Default::default()
                },
                ..Default::default()
            };

            value_processor::collect_value(
                clone_parser,
                &mut will_be_itered,
                errors,
                letter_char,
                next_char,
                last_char,
            );

            if file_key_data.data.value_location.range_start.is_zero() && letter_char != " " {
                file_key_data.data.value_location.range_start = parser.pos;
            }

            file_key_data.data.value = will_be_itered.data.value;
            file_key_data.data.value_location.range_end = parser.pos.clone().skip_char(1);
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
