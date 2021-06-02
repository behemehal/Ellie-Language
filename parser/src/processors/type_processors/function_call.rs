use crate::processors::value_processor;
use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_function_caller(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    options: defs::ParserOptions,
) {
    if let types::Types::FunctionCall(ref mut data) = itered_data.data.value {
        let mut last_param = data.params.len();
        if last_param == 0 {
            data.params
                .push(types::function_call::FunctionCallParameter::default());
            last_param = data.params.len();
        }

        let is_s_n =
            !(last_param == 0 && data.params[last_param - 1].value.is_string_non_complete());

        if letter_char == "," && is_s_n && !data.params[last_param - 1].value.is_array() {
            if data.params[last_param - 1].value.is_type_complete() {
                data.comma = true;
                data.params
                    .push(types::function_call::FunctionCallParameter::default())
            } else {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/function_call.rs:0"
                        .to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: 
