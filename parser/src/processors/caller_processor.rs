use crate::parser;
use crate::processors;
use crate::syntax::variable;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::error;

#[derive(Clone)]
pub struct CollectorResponse {
    parser: parser::Parser,
    data: variable::VariableCollector,
}

pub fn collect_caller(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    let parser_clone = parser.clone();
    if let parser::Collecting::Caller(ref mut caller_data) = parser.current {
        if letter_char == ";" && caller_data.value.is_type_complete() {
            caller_data.pos.range_end = parser.pos.clone().skip_char(1);
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            let mut emulated_collector_data = variable::VariableCollector {
                data: variable::Variable {
                    dynamic: true,
                    value: caller_data.value.clone(),
                    ..Default::default()
                },
                ..Default::default()
            };

            let collected = processors::value_processor::collect_value(
                parser_clone.clone(),
                &mut emulated_collector_data,
                letter_char,
                next_char,
                last_char,
            );

            for i in collected.errors {
                errors.push(i)
            }
            caller_data.value = collected.itered_data.data.value;
        }
    }
}
