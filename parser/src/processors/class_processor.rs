use crate::parser;
#[allow(unused_imports)]
use crate::syntax::{class, function, types};
#[allow(unused_imports)]
use ellie_core::{defs, error, utils};

#[allow(unused_imports)]
use crate::alloc::string::{String, ToString};
#[allow(unused_imports)]
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn collect_class(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
    _options: defs::ParserOptions,
) {
    if let parser::Collecting::Class(ref mut classdata) = parser.current {
        
        

    }
}
