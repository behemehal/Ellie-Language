use crate::parser;
use crate::processors;
use crate::syntax::{function, types};
use ellie_core::{defs, error, utils};

use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn collect_function(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    options: defs::ParserOptions,
) {
    if let parser::Collecting::Function(ref mut functiondata) = parser.current {
        if !functiondata.initialized {
            if last_char == " " && letter_char != " " {
                functiondata.initialized = true;
                functiondata.name_pos.range_start.0 = parser.pos.0; //Function naming started so we set the position
                functiondata.name_pos.range_start.1 = parser.pos.1; //Function naming started so we set the position
                functiondata.data.name += letter_char;
            }
        } else if !functiondata.named {
            if letter_char == "(" {
                functiondata.name_pos.range_end.0 = parser.pos.0; // function naming ended
                functiondata.name_pos.range_end.1 = parser.pos.1; // function naming ended
                functiondata.parameter_bracket_start_pos.range_start.0 = parser.pos.0; //parameter start
                functiondata.parameter_bracket_start_pos.range_start.1 = parser.pos.1; //parameter start
                functiondata.parameter_bracket_start_pos.range_end.0 = parser.pos.skipChar(1).0; //parameter start
                functiondata.parameter_bracket_start_pos.range_end.1 = parser.pos.skipChar(1).1; //parameter start
                functiondata.named = true;
            } else if last_char == " " && letter_char != " " && !functiondata.data.name.is_empty() {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/function_processor.rs:0".to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: 
