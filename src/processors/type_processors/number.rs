use crate::error;
use crate::mapper;
use crate::syntax::{variable, types};

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    _last_char: String,
    pos: mapper::defs::CursorPosition,
) {
    if let types::Types::Number(ref mut data) = itered_data.data.value {
        let is_num = letter_char.parse::<usize>().is_ok();
        if is_num {
            if data.complete {
                errors.push(error::Error {
                    debug_message: "Caria".to_string(),
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
                    pos: mapper::defs::Cursor {
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            } else {
                //data.complete = true;
                data.value = (data.value.to_string() + letter_char)
                    .parse::<usize>()
                    .unwrap();
            }
        } else if letter_char == "." {
            // String prototype
            data.complete = true;
            itered_data.data.value =
                types::Types::Refference(types::refference_type::RefferenceType {
                    refference: Box::new(itered_data.data.value.clone()),
                    on_dot: true,
                    chain: Vec::new(),
                });
        } else if types::logical_type::LogicalOpearators::is_opearator(letter_char) {
            data.complete = true;
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorType {
                    first: Box::new(itered_data.data.value.clone()),
                    first_filled: true,
                    operator: types::operator_type::Operators::LogicalType(
                        types::logical_type::LogicalOpearators::Null,
                    ),
                    operator_collect: letter_char.to_string(),
                    ..Default::default()
                });
        } else if types::comparison_type::ComparisonOperators::is_opearator(letter_char) {
            data.complete = true;
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorType {
                    first: Box::new(itered_data.data.value.clone()),
                    first_filled: true,
                    operator_collect: letter_char.to_string(),
                    operator: types::operator_type::Operators::ComparisonType(
                        types::comparison_type::ComparisonOperators::Null,
                    ),
                    ..Default::default()
                });
        } else if letter_char == " " || letter_char == ")" {
            data.complete = true;
        } else {
            errors.push(error::Error {
                debug_message: "mRNA".to_string(),
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
                pos: mapper::defs::Cursor {
                    range_start: pos,
                    range_end: pos.clone().skipChar(1),
                },
            });
        }
    }
}