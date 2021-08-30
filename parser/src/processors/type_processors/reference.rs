use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{definers, types, variable};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error, utils};

pub fn collect_reference<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
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
            if last_entry == 0 {
                let deep_scan = parser.resolve_deep_call(*reference_data.data.reference.clone());
                if deep_scan == parser::DeepCallResponse::NoElement {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "function_call_processor".to_string(),
                        debug_message: "5124e5854e7aa3d281b61dca37bdb6cc".to_string(),
                        title: error::errorList::error_s6.title.clone(),
                        code: error::errorList::error_s6.code,
                        message: error::errorList::error_s6.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s6.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: (*reference_data.data.reference.get_type()).to_string(),
                            }],
                        ),
                        pos: reference_data.data.reference_pos,
                    });
                } else {
                    reference_data.root_available = true;
                }
            } else if reference_data.root_available {
                let resolved_reference = parser
                    .clone()
                    .resolve_reference_call(reference_data.clone());

                if let Some(resolved_errors) = resolved_reference {
                    errors.extend(resolved_errors)
                } else {
                    reference_data
                        .data
                        .chain
                        .push(types::reference_type::Chain {
                            pos: defs::Cursor {
                                range_start: parser.pos.clone().skip_char(1),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                }
            }
            reference_data.on_dot = true;
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

            let itered_reference_call_vector = Box::new(value_processor::collect_value(
                parser.clone(),
                &mut will_be_itered,
                letter_char,
                next_char,
                last_char,
            ));

            if !itered_reference_call_vector.errors.is_empty() {
                errors.extend(itered_reference_call_vector.errors);
            }

            if last_entry == 0 {
                reference_data
                    .data
                    .chain
                    .push(types::reference_type::Chain {
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos,
                        },
                        value: itered_reference_call_vector.itered_data.data.value.clone(),
                    });
            } else {
                reference_data.data.chain[last_entry - 1].value =
                    itered_reference_call_vector.itered_data.data.value.clone();
                if reference_data.data.chain[last_entry - 1].pos.is_zero() {
                    reference_data.data.chain[last_entry - 1].pos.range_start = parser.pos;
                }
                reference_data.data.chain[last_entry - 1].pos.range_end = parser.pos;
            }
            reference_data.complete = itered_reference_call_vector
                .itered_data
                .data
                .value
                .is_type_complete();
            
        }
    }
}
