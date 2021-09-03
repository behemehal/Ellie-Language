use crate::parser;
use crate::processors;
use ellie_core::{defs, error, utils};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_getter_value<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    let parser_clone = parser.clone();
    if let parser::Collecting::Getter(ref mut getter_data) = parser.current {
        if !getter_data.name_wrote {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n") || getter_data.data.name.is_empty())
            {
                if getter_data.data.name.is_empty() {
                    getter_data.data.name_pos.range_start = parser.pos;
                }
                getter_data.data.name += letter_char;
                getter_data.data.name_pos.range_end = parser.pos.clone().skip_char(1);
            } else if letter_char == ":" && !getter_data.data.name.is_empty() {
                if utils::is_reserved(&getter_data.data.name) {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "b33c40fb1f6ae4432f337dd4f0dc9179".to_string(),
                        title: error::errorList::error_s21.title.clone(),
                        code: error::errorList::error_s21.code,
                        message: error::errorList::error_s21.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s21.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: getter_data.data.name.clone(),
                            }],
                        ),
                        pos: getter_data.data.name_pos,
                    });
                }
                getter_data.name_wrote = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "b5a2ec8c7a226bd97600b4965375fae0".to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if !getter_data.type_wrote {
            if getter_data.data.rtype.is_definer_complete() && letter_char == "{" {
                getter_data.type_wrote = true;
                getter_data.data.bracket_start_pos.range_start = parser.pos;
                getter_data.data.bracket_start_pos.range_end = parser.pos.clone().skip_char(1);
            } else {
                if getter_data.data.rtype_pos.range_start.0 == 0
                    && getter_data.data.rtype_pos.range_start.1 == 0
                    && letter_char != " "
                {
                    getter_data.data.rtype_pos.range_start = parser.pos;
                }
                processors::definer_processor::collect_definer(
                    parser_clone,
                    &mut getter_data.data.rtype,
                    errors,
                    letter_char.to_string(),
                    next_char,
                    last_char,
                );
                getter_data.data.rtype_pos.range_end = parser.pos;
            }
        } else if !getter_data.param_bracket_opened
            && getter_data.brace_count == 0
            && letter_char == "}"
        {
            getter_data.data.code = getter_data.inside_code.collected.clone();
            getter_data.inside_code = Box::new(parser::RawParser::default()); //Empty the cache
            getter_data.data.bracket_start_pos.range_start = parser.pos;
            getter_data.data.bracket_start_pos.range_end = parser.pos.clone().skip_char(1);
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else if !getter_data.param_bracket_opened {
            if letter_char == "{" {
                getter_data.brace_count += 1;
            } else if letter_char == "}" && getter_data.brace_count != 0 {
                getter_data.brace_count -= 1;
            }
            let mut child_parser = getter_data.inside_code.clone().to_no_resolver_parser();

            if getter_data.inside_code.pos.is_zero() {
                //Make sure upper scope imported once

                for item in parser.collected.clone() {
                    //Import variables as temporary for syntax support, we will remove them after collecting complete
                    child_parser.collected.push(parser::Collecting::ImportItem(
                        crate::syntax::import_item::ImportItem {
                            from_path: "<temporary>".to_string(),
                            public: true,
                            item: Box::new(item),
                        },
                    ));
                }
            }

            child_parser.options = parser.options.clone();
            child_parser.options.parser_type = defs::ParserType::RawParser;
            child_parser.pos = parser.pos;
            child_parser.scope.scope_name = "core/function_processor".to_string();
            child_parser.current = getter_data.inside_code.current.clone();
            child_parser.keyword_catch = getter_data.inside_code.keyword_catch.clone();
            child_parser.keyword_cache = getter_data.inside_code.keyword_cache.clone();

            let mut child_parser_errors: Vec<error::Error> = Vec::new();
            parser::iterator::iter(
                &mut child_parser,
                &mut child_parser_errors,
                letter_char,
                next_char,
                last_char,
            );
            for i in child_parser_errors {
                errors.push(i);
            }
            getter_data.inside_code = Box::new(child_parser.to_raw());
        }
    }
}
