use crate::syntax::{types, variable};
use ellie_core::{defs, error, utils};

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_refference(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    _options: defs::ParserOptions,
) {
    if let types::Types::Refference(ref mut data) = itered_data.data.value {
        if letter_char == "." {
            if data.on_dot {
                errors.push(error::Error {
                    scope: "refference_processor".to_string(),
                    debug_message: "1c37314828bf33e4941ec5ff4ddf3fcc".to_string(),
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
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            } else {
                data.on_dot = true;
            }
        } else if letter_char != " " {
            if data.on_dot {
                data.on_dot = false;
            }
            if last_char == " "
                && !data.chain.is_empty()
                && !data.chain[data.chain.len() - 1].is_empty()
            {
                if utils::is_opearators(letter_char) {
                    //itered_data.data.value = types::Types::Operators(types::OperatorType {
                    //    first: Box::new(itered_data.data.value.clone()),
                    //    operator_collect: letter_char.to_string(),
                    //    collecting_operator: true,
                    //    ..Default::default()
                    //});
                } else {
                    errors.push(error::Error {
                        scope: "refference_processor".to_string(),
                        debug_message: "a3777976f422ed842947640f5cd5ade6".to_string(),
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
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
            } else {
                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );
                if current_reliability.reliable {
                    if data.chain.is_empty() {
                        data.chain.push(letter_char.to_string());
                    } else {
                        let chain_last_element = data.chain.len() - 1;
                        data.chain[chain_last_element] =
                            data.chain[chain_last_element].clone() + letter_char;
                    }
                } else {
                    errors.push(error::Error {
                        scope: "refference_processor".to_string(),
                        debug_message: "21493b1410a41fd28970ff9c07f75d24".to_string(),
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
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
            }
        }
    }
}
