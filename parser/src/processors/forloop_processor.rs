use crate::alloc::string::String;
use crate::alloc::vec::Vec;
use crate::parser;
use ellie_core::{defs, error};

pub fn collect_for(
    parser: &mut parser::Parser,
    _errors: &mut Vec<error::Error>,
    _letter_char: &str,
    _next_char: String,
    _last_char: String,
    _options: defs::ParserOptions,
) {
    if let parser::Collecting::Forloop(ref mut _forloopdata) = parser.current {
        panic!("NOT IMPLEMENTED");
    }
}
