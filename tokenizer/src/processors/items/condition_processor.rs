use crate::processors::types::Processor;
use crate::syntax::items::condition;
use ellie_core::{defs, error, utils};

impl super::Processor for condition::Condition {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        let chain_len = self.chains.clone().len();
        let mut chain = &mut self.chains[chain_len - 1];

        match chain.rtype {
            condition::ConditionType::If => {
                if !chain.condition_filled {
                    if chain.condition.is_complete() && letter_char == '{' {
                        chain.condition_filled = true;
                    } else {
                        chain
                            .condition
                            .iterate(errors, cursor, last_char, letter_char);
                    }
                } else if letter_char == '}' && chain.brace_count == 0 {
                    chain.code = chain.iterator.collected.clone();
                    chain.complete = true;
                } else {
                    if letter_char == '{' {
                        chain.brace_count += 1;
                    } else if letter_char == '}' && chain.brace_count != 0 {
                        chain.brace_count -= 1;
                    }
                    chain.iterator.iterate(last_char, letter_char);
                }
            }
            condition::ConditionType::ElseIf => {
                if !chain.keyword_captured {
                    match chain.condition.current.clone() {
                        crate::processors::types::Processors::Variable(e) => {
                            if e.data.value.len() == 1 && e.data.value == "i" && letter_char == 'f'
                            {
                                chain.condition =
                                    crate::processors::types::TypeProcessor::default();
                                chain.rtype = condition::ConditionType::ElseIf;
                                chain.keyword_captured = true;
                                chain.keyword_pos.range_end = cursor.clone().skip_char(1);
                            } else if e.data.value == "" && letter_char == '{' {
                                chain.keyword_captured = true;
                                chain.rtype = condition::ConditionType::Else;
                            } else if e.data.value.len() != 0 {
                                errors.push(error::errorList::error_s1.clone().build(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: letter_char.to_string(),
                                    }],
                                    "cond_0x57".to_owned(),
                                    e.data.pos,
                                ));
                            }
                        }
                        e => {
                            errors.push(error::errorList::error_s1.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char.to_string(),
                                }],
                                "cond_0x68".to_owned(),
                                e.get_pos(),
                            ));
                        }
                    }
                    if !chain.keyword_captured {
                        chain
                            .condition
                            .iterate(errors, cursor, last_char, letter_char);
                    }
                } else if !chain.condition_filled {
                    if chain.condition.is_complete() && letter_char == '{' {
                        chain.condition_filled = true;
                    } else {
                        chain
                            .condition
                            .iterate(errors, cursor, last_char, letter_char);
                    }
                } else if letter_char == '}' && chain.brace_count == 0 {
                    chain.code = chain.iterator.collected.clone();
                    chain.complete = true;
                } else {
                    if letter_char == '{' {
                        chain.brace_count += 1;
                    } else if letter_char == '}' && chain.brace_count != 0 {
                        chain.brace_count -= 1;
                    }
                    chain.iterator.iterate(last_char, letter_char);
                }
            }
            condition::ConditionType::Else => {
                if letter_char == '}' && chain.brace_count == 0 {
                    chain.code = chain.iterator.collected.clone();
                    chain.complete = true;
                } else {
                    if letter_char == '{' {
                        chain.brace_count += 1;
                    } else if letter_char == '}' && chain.brace_count != 0 {
                        chain.brace_count -= 1;
                    }
                    chain.iterator.iterate(last_char, letter_char);
                }
            }
        }
    }
}
