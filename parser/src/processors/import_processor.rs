use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use ellie_core::{defs, error};

pub fn collect_import(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    _last_char: String,
    _options: defs::ParserOptions,
) {
    if let parser::Collecting::Import(ref mut importdata) = parser.current {
        if letter_char != " " && letter_char != "\n" || importdata.path.is_empty() {
            importdata.pos.range_end = parser.pos;
            importdata.path_pos.range_end = parser.pos;
            if letter_char == ";" {
                let response = (parser.resolver)(importdata.path.clone());
                if !response.found {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "59bc5156eed94e34105cd2c5ee1d935e".to_string(),
                        title: error::errorList::error_s28.title.clone(),
                        code: error::errorList::error_s28.code,
                        message: error::errorList::error_s28.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s28.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: importdata.path.clone(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else {
                    for item in response.file_content.items {
                        parser.collected.push(item);
                    }
                }
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char != " " {
                importdata.path += letter_char;
            }
        } else if letter_char != " " {
            errors.push(error::Error {
                scope: parser.scope.scope_name.clone(),
                debug_message: "59bc5156eed94e34105cd2c5ee1d935e".to_string(),
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
}
