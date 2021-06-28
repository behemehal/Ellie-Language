use crate::parser;
use crate::syntax::class;

use ellie_core::{defs, error, utils};

use crate::alloc::string::{String, ToString};

use crate::alloc::boxed::Box;
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn collect_class(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    _options: defs::ParserOptions,
) {
    if let parser::Collecting::Class(ref mut classdata) = parser.current {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if !classdata.name_collected {
            if current_reliability.reliable && (last_char != " " || classdata.data.name.is_empty())
            {
                if classdata.data.name.is_empty() {
                    classdata.data.name_pos.range_start = parser.pos;
                }
                classdata.data.name += letter_char;
                classdata.data.name_pos.range_end = parser.pos;
            } else if letter_char == "<" && !classdata.data.name.is_empty() {
                classdata.name_collected = true;
            } else if letter_char == "{" && !classdata.data.name.is_empty() {
                classdata.name_collected = true;
                classdata.generic_definings_collected = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.clone() + "/class_processor",
                    debug_message: "9841eff3bfeba5c42edc6ec6bd6168be".to_string(),
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
                        range_end: parser.pos.clone().skipChar(1),
                    },
                });
            }
        } else if !classdata.generic_definings_collected {
            let mut last_entry = classdata.data.generic_definings.len();

            if last_entry == 0 && current_reliability.reliable {
                //...reliable will make sure in case of no parameter used no parameter data will be applied
                classdata
                    .data
                    .generic_definings
                    .push(class::GenericDefining::default());
                last_entry = 1;
            }

            if current_reliability.reliable
                && (last_char != " "
                    || classdata.data.generic_definings[last_entry - 1]
                        .name
                        .is_empty())
            {
                if classdata.data.generic_definings[last_entry - 1]
                    .name
                    .is_empty()
                {
                    classdata.data.generic_definings[last_entry - 1]
                        .pos
                        .range_start = parser.pos;
                }
                classdata.at_comma = false;
                classdata.data.generic_definings[last_entry - 1].name += letter_char;
            } else if letter_char == ">" && !classdata.at_comma {
                classdata.generic_definings_collected = true;
            } else if letter_char == "," && !classdata.at_comma {
                classdata.at_comma = true;
                classdata
                    .data
                    .generic_definings
                    .push(class::GenericDefining::default());
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.clone() + "/class_processor",
                    debug_message: "9841eff3bfeba5c42edc6ec6bd6168be".to_string(),
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
                        range_end: parser.pos.clone().skipChar(1),
                    },
                });
            }
        } else if !classdata.has_code && letter_char == "{" {
            classdata.has_code = true;
        } else if classdata.brace_count == 0 && letter_char == "}" {
            for i in classdata.code.collected.clone() {
                match i {
                    parser::Collecting::Variable(e) => {
                        classdata.data.properties.push(e.data);
                    }
                    parser::Collecting::Function(e) => {
                        classdata.data.methods.push(e.data);
                    }
                    parser::Collecting::Constructor(e) => {
                        classdata.data.constructor = e.data;
                    }
                    _ => {}
                };
            }

            //classdata.data.inside_code = classdata.code.collected.clone();

            classdata.code = Box::new(parser::Parser::default()); //Empty the cache
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if letter_char == "{" {
                classdata.brace_count += 1;
            } else if letter_char == "}" && classdata.brace_count != 0 {
                classdata.brace_count -= 1;
            }

            let mut child_parser = classdata.code.clone();
            child_parser.options = parser.options.clone();
            child_parser.options.parser_type = defs::ParserType::ClassParser;
            let mut child_parser_errors: Vec<error::Error> = Vec::new();
            parser::iterator::iter(
                &mut child_parser,
                &mut child_parser_errors,
                letter_char,
                next_char,
                last_char,
            );

            for i in child_parser_errors {
                let mut edited = i;
                edited.pos.range_start.0 += parser.pos.0;
                edited.pos.range_start.1 += parser.pos.1;
                edited.pos.range_end.0 += parser.pos.0;
                edited.pos.range_end.1 += parser.pos.1;
                errors.push(edited);
            }
            classdata.code = child_parser;
        }
    }
}
