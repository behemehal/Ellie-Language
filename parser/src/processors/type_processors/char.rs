use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_char(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
) {
    if let types::Types::Char(ref mut data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.rtype = crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "char".to_string(),
                },
            );
        }

        if letter_char == "'" && last_char != "\\" {
            if data.value == '\0' {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/char.rs:0".to_string(),
                    title: error::errorList::error_s14.title.clone(),
                    code: error::errorList::error_s14.code,
                    message: error::errorList::error_s14.message.clone(),
                    builded_message: error::BuildedError::build_from_string(error::errorList::error_s14.message.clone()),
                    pos: defs::Cursor {
                        range_start: pos.clone().popChar(1),
                        range_end: pos.clone().skipChar(1),
                    },
                });
            }
            data.complete = true;
        } else if letter_char == 
