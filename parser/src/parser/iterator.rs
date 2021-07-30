use crate::parser;
use crate::processors;
use ellie_core::error;

use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn iter(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if parser.current == parser::Collecting::None {
        if !parser.keyword_catch.is_empty()
            && parser.pos.1 == 0
            && parser.keyword_catch.trim() != ""
            && !parser.on_comment
        {
            errors.push(error::Error {
                scope: parser.scope.scope_name.clone(),
                debug_message: "3854ff24d000686faa66d0c9c7ff9b12".to_string(),
                title: error::errorList::error_s23.title.clone(),
                code: error::errorList::error_s23.code,
                message: error::errorList::error_s23.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s23.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: parser.keyword_catch.clone(),
                    }],
                ),
                pos: parser.keyword_pos,
            });
            parser.keyword_catch = String::new();
        } else {
            if parser.keyword_catch.is_empty() {
                parser.keyword_pos.range_start = parser.pos;
            }
            parser.keyword_pos.range_end = parser.pos.clone().skip_char(1);
            parser.keyword_catch += letter_char;

            processors::type_processor::collect_type(
                parser,
                errors,
                letter_char,
                last_char.clone(),
                next_char.clone(),
            );
        }
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
        ),
        parser::Collecting::Condition(_) => processors::condition_processor::collect_condition(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
        ),
        parser::Collecting::Function(_) => processors::function_processor::collect_function(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
        ),
        parser::Collecting::Class(_) => processors::class_processor::collect_class(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
        ),
        parser::Collecting::Ret(_) => processors::ret_processor::collect_ret(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
        ),
        parser::Collecting::Constructor(_) => {
            processors::constructor_processor::collect_constructor(
                parser,
                errors,
                letter_char,
                next_char.clone(),
                last_char.clone(),
            )
        }
        parser::Collecting::Import(_) => processors::import_processor::collect_import(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
        ),
        parser::Collecting::Caller(_) => processors::caller_processor::collect_caller(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
        ),
        parser::Collecting::FileKey(_) => processors::filekey_processor::collect_filekey(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
        ),
        parser::Collecting::Forloop(_) => processors::forloop_processor::collect_for(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
        ),
        _ => (),
    };
}
