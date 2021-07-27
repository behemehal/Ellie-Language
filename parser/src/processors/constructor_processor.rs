use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;
use crate::parser;
use crate::syntax::constructor;
use ellie_core::{defs, error, utils};

pub fn collect_constructor(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
) {
    if let parser::Collecting::Constructor(ref mut constructordata) = parser.current {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if !constructordata.named {
            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n") || constructordata.data.name.is_empty())
            {
                if constructordata.data.name.is_empty() {
                    constructordata.data.name_pos.range_start = parser.pos;
                }

                constructordata.data.name += letter_char;
                constructordata.data.name_pos.range_end = parser.pos.clone().skip_char(1);
            } else if letter_char == "(" {
                constructordata.named = true;
                constructordata.data.parameters_pos.range_start = parser.pos;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "f0b0af04411d1f27effcd14b4d49219b".to_string(),
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
        } else if !constructordata.parameter_wrote {
            let mut last_entry = constructordata.data.parameters.len();

            if last_entry == 0 && current_reliability.reliable {
                //...reliable will make sure in case of no parameter used no parameter data will be applied
                constructordata
                    .data
                    .parameters
                    .push(constructor::ConstructorParameter::default());
                last_entry = 1;
            }

            if current_reliability.reliable
                && ((last_char != " " && last_char != "\n")
                    || constructordata.data.parameters[last_entry - 1]
                        .name
                        .is_empty())
            {
                if constructordata.data.parameters[last_entry - 1]
                    .name
                    .is_empty()
                {
                    constructordata.data.parameters[last_entry - 1]
                        .pos
                        .range_start = parser.pos;
                }
                constructordata.at_comma = false;
                constructordata.data.parameters[last_entry - 1].name += letter_char;
            } else if letter_char == ")" && !constructordata.at_comma {
                constructordata.parameter_wrote = true;
            } else if letter_char == "," && !constructordata.at_comma {
                constructordata.at_comma = true;
                constructordata
                    .data
                    .parameters
                    .push(constructor::ConstructorParameter::default());
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "612ba77cee33981aff72ed785c793033".to_string(),
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
        } else if !constructordata.has_code {
            if letter_char == "{" {
                constructordata.has_code = true;
            } else if letter_char == ";" {
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "1a411c8be3fed7efae767713ca30a4dd".to_string(),
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
        } else if constructordata.brace_count == 0 && letter_char == "}" {
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if letter_char == "{" {
                constructordata.brace_count += 1;
            } else if letter_char == "}" && constructordata.brace_count != 0 {
                constructordata.brace_count -= 1;
            }

            let code_letter = if last_char.clone() == "\n" || last_char.clone() == "\r" {
                last_char + letter_char //Make sure we get the lines correctly
            } else {
                letter_char.to_string()
            };
            constructordata.code += &code_letter;
        }
    }
}
