use crate::syntax::variable;
use ellie_core::{defs, error};

use alloc::string::String;
use alloc::vec::Vec;

pub fn collect(
    _itered_data: &mut variable::VariableCollector,
    _errors: &mut Vec<error::Error>,
    _letter_char: &str,
    _next_char: String,
    _last_char: String,
    _pos: defs::CursorPosition,
) {

    //if let types::Types::ArrowFunction(ref mut data) = itered_data.data.value {
    //    
    //}
}
