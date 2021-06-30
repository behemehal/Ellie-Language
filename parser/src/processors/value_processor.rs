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
    options: defs::ParserOptions,
) -> CollectorResponse {
    let mut errors: Vec<error::Error> = Vec::new();
    match &mut itered_data.data.value {
        types::Types::Integer(_) => type_processors::integer::collect_integer(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::Float(_) => type_processors::float::collect_float(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::Bool(_) => type_processors::bool::collect_bool(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::String(_) => type_processors::string::collect_string(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::Char(_) => type_processors::char::collect_char(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::Collective => (),
        types::Types::Refference(_) => type_processors::refference::collect_refference(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::Operator(_) => type_processors::operator::collect_operator(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::Array(_) => type_processors::array::collect_array(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::Cloak(_) => type_processors::cloak::collect_cloak(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::ArrowFunction(_) => type_processors::arrow_function::collect_arrow(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::FunctionCall(_) => type_processors::function_call::collect_function_caller(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::ClassCall(_) => type_processors::class_call::collect_class_call(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::Void => (),
        types::Types::VariableType(_) => type_processors::variable::collect_variable(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
        types::Types::Null => type_processors::null::collect_null(
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
            pos,
            options,
        ),
    }
    CollectorResponse {
        itered_data: itered_data.clone(),
        errors,
    }
}
