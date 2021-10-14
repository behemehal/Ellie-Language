use crate::parser;
use crate::processors;
use crate::syntax::variable;
use alloc::vec::Vec;
use alloc::string::String;
use ellie_core::error;

pub fn collect_caller<F, E>(
    parser: &mut parser::Parser<F, E>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport + Clone + Sized
{
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

            processors::value_processor::collect_value(
                parser_clone.clone(),
                &mut emulated_collector_data,
                errors,
                letter_char,
                next_char,
                last_char,
            );
            caller_data.value = emulated_collector_data.data.value;
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
