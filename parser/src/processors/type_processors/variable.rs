use crate::syntax::{types, variable};
use ellie_core::{error, defs, utils};

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    _last_char: String,
    pos: defs::CursorPosition,
) {
    if let types::Types::VariableType(ref mut data) = itered_data.data.value {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if current_reliability.reliable {
            data.value += letter_char;
        } else if letter_char == " " && !data.value_complete {
            if data.value == "false" || data.value == "true" {
                itered_data.data.value = types::Types::Bool(types::bool_type::BoolType {
                    value: data.value.parse::<bool>().unwrap(),
                });
            } else {
                data.value_complete = true;
            }
        } else if letter_char == "\"" || letter_char == "'" {
            itered_data.data.value = types::Types::String(types::string_type::StringType {
                quote_type: letter_char.to_string(),
                ..Default::default()
            })
        } else {
            //String 'i handle la
            errors.push(error::Error {
                debug_message: "Wole".to_string(),
                title: error::errorList::error_s1.title.clone(),
                code: error::errorList::error_s1.code,
                message: error::errorList::error_s1.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s1.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: current_reliability.found.to_string(),
                    }],
                ),
                pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        pos.0,
                        (pos.1 - itered_data.raw_value.len() as i64)
                            + current_reliability.at as i64,
                    ),
                    range_end: defs::CursorPosition(
                        pos.0,
                        ((pos.1 - itered_data.raw_value.len() as i64)
                            + current_reliability.at as i64)
                            + 1,
                    ),
                },
            });
        }
    }
}
