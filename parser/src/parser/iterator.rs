use crate::alloc::borrow::ToOwned;
use crate::alloc::string::String;
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::processors;
use ellie_core::{defs, error};

pub fn iter<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    if parser.current == parser::Collecting::None {
        if !parser.keyword_catch.is_empty()
            && parser.pos.1 == 0
            && parser.keyword_catch.trim() != ""
            && !parser.on_comment
        {
            errors.push(error::Error {
                path: parser.options.path.clone(),
                scope: parser.scope.scope_name.clone(),
                debug_message: "700c52ebde05efbf720fa6d845cfde77".to_owned(),
                title: error::errorList::error_s23.title.clone(),
                code: error::errorList::error_s23.code,
                message: error::errorList::error_s23.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s23.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: parser.keyword_catch.clone(),
                    }],
                ),
                pos: defs::Cursor {
                    range_start: defs::CursorPosition(parser.keyword_pos.range_start.0, 0),
                    range_end: parser.keyword_pos.range_end,
                },
            });
            parser.keyword_pos.range_start = parser.pos;
            parser.keyword_catch = String::new();
            parser.keyword_errors = Vec::new();
            parser.keyword_cache = crate::syntax::variable::VariableCollector::default();
        } else {
            if parser.keyword_catch.trim().is_empty() && letter_char != " " {
                parser.keyword_pos.range_start = parser.pos;
            }
            if letter_char != " " {
                parser.keyword_pos.range_end = parser.pos.clone().skip_char(1);
            }
            parser.keyword_catch += letter_char;
            processors::type_processor::collect_type(
                parser,
                errors,
                letter_char,
                last_char.clone(),
                next_char.clone(),
            );

            if parser.current == parser::Collecting::None
                && !parser.on_comment
                && !parser.on_line_comment
            {
                if parser.keyword_cache.data.value.is_type_complete() && letter_char == ";" {
                    parser.collected.push(parser::Collecting::ValueCall(
                        parser.keyword_cache.data.value.clone(),
                    ));
                    parser.keyword_catch = "".to_owned();
                } else {
                    processors::value_processor::collect_value(
                        parser.clone(),
                        &mut parser.keyword_cache,
                        &mut parser.keyword_errors,
                        letter_char,
                        next_char,
                        last_char,
                    );
                }
            }
        }
    } else {
        parser.keyword_pos.range_start = parser.pos;
        parser.keyword_catch = String::new();
        parser.keyword_errors = Vec::new();
        parser.keyword_cache = crate::syntax::variable::VariableCollector::default();
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
        parser::Collecting::Enum(_) => processors::enum_processor::collect_enum(
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
        parser::Collecting::ForLoop(_) => processors::for_loop_processor::collect_for(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
        ),
        parser::Collecting::Getter(_) => processors::getter_processor::collect_getter_value(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
        ),
        parser::Collecting::Setter(_) => processors::setter_processor::collect_setter_value(
            parser,
            errors,
            letter_char,
            next_char.clone(),
            last_char.clone(),
        ),
        _ => (),
    };
}
