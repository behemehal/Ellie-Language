use crate::alloc::string::String;
use crate::alloc::vec::Vec;
use crate::parser;
use ellie_core::{defs, error};

pub fn collect_enum<F>(
    parser: &mut parser::Parser<F>,
    _errors: &mut Vec<error::Error>,
    _letter_char: &str,
    _next_char: String,
    _last_char: String,
    _options: defs::ParserOptions,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    if let parser::Collecting::ForLoop(ref mut _forloopdata) = parser.current {
        panic!("NOT IMPLEMENTED");
    }
}
