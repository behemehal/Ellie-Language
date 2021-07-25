use crate::parser;
use crate::processors::value_processor;
use crate::syntax::variable;
use ellie_core::error;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

pub fn collect_ret(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String
) {
    let parser_clone = parser.clone();
    if let parser::Collecting::Ret(ref mut data) = parser.current {
        if letter_char == ";" && data.value.is_type_complete() {
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            let mut will_be_itered = variable::VariableCollector {
                data: variable::Variable {
                    value: data.value.clone(),
                    ..Default::default()
                },
                ..variable::VariableCollector::default()
            };
            let itered_ret_vector = Box::new(value_processor::collect_value(
                parser_clone.clone(),
                &mut will_be_itered,
                letter_char,
                next_char,
                last_char,
            ));
            if !itered_ret_vector.errors.is_empty() {
                errors.extend(itered_ret_vector.errors);
            }

            data.value = itered_ret_vector.itered_data.data.value;
        }
    }
}
