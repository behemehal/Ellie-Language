use crate::parser;
use crate::syntax::definers::DefinerCollecting;
use crate::syntax::{types, variable};
use alloc::{borrow::ToOwned, string::String, vec::Vec};
use ellie_core::{defs, error, utils};

pub fn collect_bracket_reference<F, E>(
    parser: parser::Parser<F, E>,
    itered_data: &mut variable::VariableCollector,
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
    if let types::Types::BracketReference(ref mut bracket_reference_data) = itered_data.data.value {
        todo!();
    } else {
        panic!("Unexpected parser behaviour")
    }
}
