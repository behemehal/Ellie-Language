use crate::parser;
use crate::processors::value_processor;
use crate::syntax::variable;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::error;

pub fn collect_ret<F, E>(
    parser: &mut parser::Parser<F, E>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Sized,
{
    let parser_clone = parser.clone();
    if let parser::Collecting::Ret(ref mut data) = parser.current {
        if letter_char == ";" && data.value.is_type_complete() {
            data.pos.range_end = parser.pos.clone().skip_char(1);
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

            if letter_char != " " && data.value_position.range_start.is_zero() {
                data.value_position.range_start = parser.pos.clone();
            }
            data.value_position.range_end = parser.pos.clone().skip_char(1);

            value_processor::collect_value(
                parser_clone.clone(),
                &mut will_be_itered,
                errors,
                letter_char,
                next_char,
                last_char,
            );

            data.value = will_be_itered.data.value;
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
