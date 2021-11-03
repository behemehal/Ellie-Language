use crate::parser;
use crate::syntax::definers::DefinerCollecting;
use crate::syntax::{types, variable};
use alloc::{borrow::ToOwned, string::String, vec::Vec};
use ellie_core::{defs, error, utils};

pub fn collect_reference<F, E>(
    parser: parser::Parser<F, E>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Copy + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Copy
        + Sized,
{
    if let types::Types::Reference(ref mut reference_data) = itered_data.data.value {
        let last_entry = reference_data.data.chain.len();
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_owned(),
        );

        if letter_char == "."
            && !reference_data.on_dot
            && (last_entry == 0 || reference_data.data.chain[last_entry - 1].value.len() != 0)
        {
            reference_data.complete = false;
            reference_data.on_dot = true;
            reference_data
                .data
                .chain
                .push(types::reference_type::Chain::default())
        } else if current_reliability.reliable {
            reference_data.on_dot = false;
            reference_data.complete = true;

            if last_entry == 0 {
                reference_data
                    .data
                    .chain
                    .push(types::reference_type::Chain {
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                        value: letter_char.to_owned(),
                    })
            } else {
                reference_data.data.chain[last_entry - 1].pos.range_end =
                    parser.pos.clone().skip_char(1);
                reference_data.data.chain[last_entry - 1].value += letter_char;
            };

            if !utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                next_char.to_owned(),
            )
            .reliable
                && reference_data.root_available
            {
                match parser.resolve_reference_call(reference_data.clone()) {
                    Ok(e) => {
                        if itered_data.data.dynamic {
                            itered_data.data.rtype = e.clone();
                        }
                        reference_data.last_entry = e;
                    }
                    Err(e) => {
                        if itered_data.data.dynamic {
                            itered_data.data.rtype = DefinerCollecting::Error(0);
                        }
                        errors.extend(e)
                    }
                }
            } else if !reference_data.root_available {
                if itered_data.data.dynamic {
                    itered_data.data.rtype = DefinerCollecting::Error(1);
                }
            }
        } else {
            errors.push(error::Error {
                path: parser.options.path.clone(),
                scope: parser.scope.scope_name.clone(),
                debug_message: "replace_reference_63".to_owned(),
                title: error::errorList::error_s10.title.clone(),
                code: error::errorList::error_s10.code,
                message: error::errorList::error_s10.message.clone(),
                builded_message: error::BuildedError::build_from_string(
                    error::errorList::error_s10.message.clone(),
                ),
                pos: defs::Cursor {
                    range_start: parser.pos,
                    range_end: parser.pos.clone().skip_char(1),
                },
            });
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
