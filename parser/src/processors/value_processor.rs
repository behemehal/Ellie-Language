use crate::processors::type_processors;
use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, PartialEq)]
pub struct CollectorResponse {
    pub itered_data: variable::VariableCollector,
    pub errors: Vec<error::Error>,
}

pub fn collect_value(
    itered_data: &mut variable::VariableCollector,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
<<<<<<< HEAD
    options: defs::ParserOptions,
=======
<<<<<<< HEAD
    options: defs::ParserOptions,
=======
    options: defs::ParserOptions
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
) -> CollectorResponse {
    let mut errors: Vec<error::Error> = Vec::new();
    match &mut itered_data.data.value {
        types::Types::Number(_) => type_processors::number::collect_number(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
        ),
        types::Types::Bool(_) => (),
        types::Types::String(_) => type_processors::string::collect_string(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
        ),
        types::Types::Char(_) => type_processors::char::collect_char(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
        ),
        types::Types::Collective => (),
        types::Types::Refference(_) => type_processors::refference::collect_refference(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
<<<<<<< HEAD
            options,
=======
<<<<<<< HEAD
            options,
=======
            options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
        ),
        types::Types::Operator(_) => type_processors::operator::collect_operator(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
<<<<<<< HEAD
            options,
=======
<<<<<<< HEAD
            options,
=======
            options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
        ),
        types::Types::Array(_) => type_processors::array::collect_array(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
<<<<<<< HEAD
            options,
=======
<<<<<<< HEAD
            options,
=======
            options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
        ),
        types::Types::Cloak(_) => type_processors::cloak::collect_cloak(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
<<<<<<< HEAD
            options,
=======
<<<<<<< HEAD
            options,
=======
            options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
        ),
        types::Types::ArrowFunction(_) => type_processors::arrow_function::collect_arrow(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
<<<<<<< HEAD
            options,
=======
<<<<<<< HEAD
            options,
=======
            options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
        ),
        types::Types::FunctionCall(_) => type_processors::function_call::collect_function_caller(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
<<<<<<< HEAD
            options,
=======
<<<<<<< HEAD
            options,
=======
            options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
        ),
        types::Types::Void => (),
        types::Types::VariableType(_) => type_processors::variable::collect_variable(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
        ),
        types::Types::Null => type_processors::null::collect_null(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
        ),
    }
    CollectorResponse {
        itered_data: itered_data.clone(),
        errors,
    }
}
