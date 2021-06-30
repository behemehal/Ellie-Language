use crate::alloc::boxed::Box;
use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::processors;
use crate::syntax::{definers, function};
use ellie_core::{defs, error, utils};

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
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char != " " {
                importdata.path += letter_char;
            }
        } else if letter_char != " " {
            errors.push(error::Error {
                scope: parser.scope.clone() + "/import_processor",
                debug_message: "replace".to_string(),
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
    }

}