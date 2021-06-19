use crate::parser;
#[allow(unused_imports)]
use crate::syntax::{function, types};
#[allow(unused_imports)]
use ellie_core::{defs, error, utils};

#[allow(unused_imports)]
use crate::alloc::string::{String, ToString};
#[allow(unused_imports)]
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn collect_class(
    _parser: &mut parser::Parser,
    _errors: &mut Vec<error::Error>,
    _letter_char: &str,
    _next_char: String,
    _last_char: String,
    _options: defs::ParserOptions,
) {
    panic!("NOT IMPLEMENTED")
}
