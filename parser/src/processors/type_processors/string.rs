use crate::syntax::{types, variable};
use ellie_core::{defs, error, utils};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_string(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
) {
    if let types::Types::String(ref mut data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.rtype = crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "string".to_string(),
                },
            );
        }

        if letter_char == "\"" && last_char != "\\" {
            if data.complete {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/string.rs:0"
                        .to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: 
