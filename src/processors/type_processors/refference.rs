use crate::error;
use crate::mapper;
use crate::syntax::{types, variable};
use crate::utils;

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
    pos: mapper::defs::CursorPosition,
) {
    if let types::Types::Refference(ref mut data) = itered_data.data.value {
        if letter_char == "." {
            if data.on_dot {
                errors.push(error::Error {
                    debug_message: "Yugirmnoa".to_string(),
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
                data.on_dot = true;
            }
        } else if letter_char != " " {
            if data.on_dot {
                data.on_dot = false;
                data.chain.push(letter_char.to_string());
            } else if last_char == " " && !data.chain.is_empty() && !data.chain[data.chain.len() - 1].is_empty() {
                if utils::is_opearators(letter_char) {
                    //itered_data.data.value = types::Types::Operators(types::OperatorType {
                    //    first: Box::new(itered_data.data.value.clone()),
                    //    operator_collect: letter_char.to_string(),
                    //    collecting_operator: true,
                    //    ..Default::default()
                    //});
                } else {
                    errors.push(error::Error {
                        debug_message: "Fsteasthialvi".to_string(),
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
            } else {

                /*

                If refference value is a number and also given chain element is a also number 

                */

                let is_num = letter_char.parse::<usize>().is_ok();
                if is_num {
                    if data.chain.is_empty() { 
                    
                        if let types::Types::Number(child_data) = *data.refference {
                            itered_data.data.value =
                            types::Types::Double(types::double_type::DoubleType {
                                value: (child_data.value.to_string() + letter_char).parse::<f32>().unwrap(),
                                raw_value: (child_data.value.to_string() + letter_char),
                                complete: false,
                            });
                        } else {
                            panic!("Error");
                        }

                    } else {
                        //Properties cannot be a raw number
                        panic!("Error");
                    }
                } else {
                    let chain_last_element = data.chain.len() - 1;
                    data.chain[chain_last_element] =
                        data.chain[chain_last_element].clone() + letter_char;
                }
            }
        }
    }
}
