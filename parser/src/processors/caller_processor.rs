use crate::parser;
use crate::processors;
use crate::syntax::variable;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::error;

#[derive(Debug, Clone, PartialEq)]
pub struct CollectorResponse {
    parser: parser::Parser,
    data: variable::VariableCollector,
}

pub fn collect_caller(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String
) {
    let parser_clone = parser.clone();
    if let parser::Collecting::Caller(ref mut callerdata) = parser.current {
        if letter_char == ";" && callerdata.value.is_type_complete() {
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            let mut emulated_collector_data = variable::VariableCollector {
                data: variable::Variable {
                    dynamic: true,
                    value: callerdata.value.clone(),
                    ..Default::default()
                },
                ..Default::default()
            };

            let collected = processors::value_processor::collect_value(
                parser_clone,
                &mut emulated_collector_data,
                letter_char,
                next_char,
                last_char,
            );

            for i in collected.errors {
                errors.push(i)
            }
            callerdata.value = collected.itered_data.data.value;
        }
    }
}
