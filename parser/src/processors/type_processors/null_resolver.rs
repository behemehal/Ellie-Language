use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{types, variable};
use alloc::boxed::Box;
use alloc::vec::Vec;
use ellie_core::error;

pub fn collect_null_resolver<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    if let types::Types::NullResolver(ref mut null_resolver_data) = itered_data.data.value {
        let mut will_be_itered = variable::VariableCollector {
            data: variable::Variable {
                value: *null_resolver_data.value.clone(),
                ..Default::default()
            },
            ..variable::VariableCollector::default()
        };

        Box::new(value_processor::collect_value(
            parser.clone(),
            &mut will_be_itered,
            errors,
            letter_char,
            next_char,
            last_char,
        ));

        null_resolver_data.value = Box::new(will_be_itered.data.value);
    } else {
        panic!("Unexpected parser behaviour")
    }
}
