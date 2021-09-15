use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{types, variable};
use alloc::vec::Vec;
use ellie_core::{defs, error};

pub fn collect_reference<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    if let types::Types::Reference(ref mut reference_data) = itered_data.data.value {
        let last_entry = reference_data.data.chain.len();

        if letter_char == "."
            && !reference_data.on_dot
            && (last_entry == 0
                || reference_data.data.chain[last_entry - 1]
                    .value
                    .is_type_complete())
        {
            reference_data.on_dot = true;
            reference_data
                .data
                .chain
                .push(types::reference_type::Chain::default())
        } else {
            reference_data.on_dot = false;
            let mut will_be_itered = if last_entry == 0 {
                variable::VariableCollector {
                    ignore_existence: true,
                    ..Default::default()
                }
            } else {
                variable::VariableCollector {
                    data: variable::Variable {
                        value: reference_data.data.chain[last_entry - 1].value.clone(),
                        ..Default::default()
                    },
                    ignore_existence: true,
                    ..Default::default()
                }
            };

            value_processor::collect_value(
                parser.clone(),
                &mut will_be_itered,
                errors,
                letter_char,
                next_char,
                last_char,
            );

            if last_entry == 0 {
                reference_data
                    .data
                    .chain
                    .push(types::reference_type::Chain {
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos,
                        },
                        value: will_be_itered.data.value.clone(),
                    });
            } else {
                reference_data.data.chain[last_entry - 1].value = will_be_itered.data.value.clone();
                if reference_data.data.chain[last_entry - 1].pos.is_zero() {
                    reference_data.data.chain[last_entry - 1].pos.range_start = parser.pos;
                }
                reference_data.data.chain[last_entry - 1].pos.range_end =
                    parser.pos.clone().skip_char(1);
            }
            reference_data.complete = will_be_itered.data.value.is_type_complete();
        }
    }
}
