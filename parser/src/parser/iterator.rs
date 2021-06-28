use crate::parser;
use crate::processors;
use ellie_core::error;

use crate::alloc::string::String;
use crate::alloc::vec::Vec;

pub fn iter(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if parser.current == parser::Collecting::None {
        parser.keyword_catch += letter_char;
        processors::type_processor::collect_type(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            parser.options.clone(),
        );
    } else {
        parser.keyword_catch = String::new();
    }

    match parser.current {
        parser::Collecting::Variable(_) => processors::variable_processor::collect_variable_value(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
            parser.options.clone(),
        ),

        parser::Collecting::Condition(_) => processors::condition_processor::collect_condition(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
            parser.options.clone(),
        ),
        parser::Collecting::Function(_) => processors::function_processor::collect_function(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
            parser.options.clone(),
        ),
        parser::Collecting::Class(_) => processors::class_processor::collect_class(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
            parser.options.clone(),
        ),
        parser::Collecting::Ret(_) => processors::ret_processor::collect_ret(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
            parser.options.clone(),
        ),
        parser::Collecting::Constructor(_) => {
            processors::constructor_processor::collect_constructor(
                parser,
                errors,
                letter_char,
                next_char.clone(),
                last_char.clone(),
                parser.options.clone(),
            )
        }
        _ => (),
    };
}
