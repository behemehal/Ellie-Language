use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{types, variable};
use ellie_core::error;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

pub fn collect_negative<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    if let types::Types::Negative(ref mut negative_data) = itered_data.data.value {
        let mut will_be_itered = variable::VariableCollector {
            data: variable::Variable {
                value: *negative_data.value.clone(),
                ..Default::default()
            },
            ..variable::VariableCollector::default()
        };

        let itered_negative_vector = Box::new(value_processor::collect_value(
            parser.clone(),
            &mut will_be_itered,
            letter_char,
            next_char,
            last_char,
        ));

        if !itered_negative_vector.errors.is_empty() {
            errors.extend(itered_negative_vector.errors);
        }
        negative_data.value = Box::new(itered_negative_vector.itered_data.data.value);
        
    }
}
