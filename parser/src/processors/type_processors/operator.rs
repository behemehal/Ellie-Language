use crate::processors::value_processor;
use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_operator(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    options: defs::ParserOptions,
) {
    if let types::Types::Operator(ref mut data) = itered_data.data.value {
        if !data.operator_collected {
            //Operator
            let is_opearator = types::operator_type::Operators::resolve_operator(
                data.operator.clone(),
                &(data.operator_collect.clone() + letter_char),
            );
            if is_opearator.is_err() {
                if letter_char == " " {
                    data.operator_collected = true;
                } else {
                    errors.push(error::Error {
                        debug_message: "./parser/src/processors/type_processors/operator.rs:0"
                            .to_string(),
                        title: error::errorList::error_s13.title.clone(),
                        code: error::errorList::error_s13.code,
                        message: error::errorList::error_s13.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s13.message.clone(),
                            vec![error::ErrorBuildField {
                                key: 
