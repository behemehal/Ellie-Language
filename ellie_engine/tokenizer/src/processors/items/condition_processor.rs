use crate::{processors::EscapeCharEmitter, syntax::items::condition};
use ellie_core::{defs, error};

impl crate::processors::Processor for condition::Condition {
    fn emits_line_endings(&self) -> EscapeCharEmitter {
        if self.chains.len() != 0 {
            EscapeCharEmitter::dont_emit()
        } else {
            self.chains[self.chains.len() - 1]
                .iterator
                .emits_line_endings()
        }
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;
        let chain_len = self.chains.len();
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
                    chain.iterator.finalize();
                    errors.extend(chain.iterator.errors.clone());
                    chain.complete = true;
                } else {
                    if letter_char == '{' {
                        chain.brace_count += 1;
                    } else if letter_char == '}' && chain.brace_count != 0 {
                        chain.brace_count -= 1;
                    }
                    chain.iterator.pos = cursor;
                    hang = chain.iterator.iterate(last_char, letter_char);
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
                                chain.keyword_pos.range_end = cursor;
                            } else if e.data.value == "" && letter_char == '{' {
                                chain.keyword_captured = true;
                                chain.rtype = condition::ConditionType::Else;
                            } else if e.data.value.len() != 0 {
                                errors.push(error::error_list::ERROR_S1.clone().build(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: letter_char.to_string(),
                                    }],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    e.data.pos,
                                ));
                            }
                        }
                        e => {
                            errors.push(error::error_list::ERROR_S1.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char.to_string(),
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                    chain.iterator.finalize();
                    errors.extend(chain.iterator.errors.clone());
                } else {
                    if letter_char == '{' {
                        chain.brace_count += 1;
                    } else if letter_char == '}' && chain.brace_count != 0 {
                        chain.brace_count -= 1;
                    }
                    chain.iterator.pos = cursor;
                    hang = chain.iterator.iterate(last_char, letter_char);
                }
            }
            condition::ConditionType::Else => {
                if letter_char == '}' && chain.brace_count == 0 {
                    chain.code = chain.iterator.collected.clone();
                    chain.iterator.finalize();
                    errors.extend(chain.iterator.errors.clone());
                    chain.complete = true;
                } else {
                    if letter_char == '{' {
                        chain.brace_count += 1;
                    } else if letter_char == '}' && chain.brace_count != 0 {
                        chain.brace_count -= 1;
                    }
                    chain.iterator.pos = cursor;
                    hang = chain.iterator.iterate(last_char, letter_char);
                }
            }
        }
        self.pos.range_end = cursor;
        hang
    }
}
