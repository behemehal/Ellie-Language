use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    _last_char: String,
    pos: defs::CursorPosition,
) {
    let q: char;

    if let types::Types::Char(ref mut data) = itered_data.data.value {
        if letter_char == "'" {
            if data.value.is_empty() {
                errors.push(error::Error {
                    debug_message: "Smzlkg".to_string(),
                    title: error::errorList::error_s14.title.clone(),
                    code: error::errorList::error_s14.code,
                    message: error::errorList::error_s14.message.clone(),
                    builded_message: error::errorList::error_s14.message.clone(),
                    pos: defs::Cursor {
                        range_start: pos.clone().popChar(1),
                        range_end: pos.clone().skipChar(1),
                    },
                });
            }
            data.complete = true;
        } else if data.value.is_empty() {
            data.value += letter_char;
        } else {
            errors.push(error::Error {
                debug_message: "Smzlkg".to_string(),
                title: error::errorList::error_s15.title.clone(),
                code: error::errorList::error_s15.code,
                message: error::errorList::error_s15.message.clone(),
                builded_message: error::errorList::error_s15.message.clone(),
                pos: defs::Cursor {
                    range_start: pos.clone().popChar(1),
                    range_end: pos.clone().skipChar(1),
                },
            });
        }
    }
}
