use ellie_core::{error, defs, utils};
use crate::parser;
use crate::syntax::{function, types};

use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::alloc::string::{String, ToString};

pub fn collect(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
) {
    
}