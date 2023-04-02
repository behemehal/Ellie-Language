#![allow(unused_assignments)]

use crate::processors::{
    items::{self},
    Processor,
};
use ellie_core::{defs, error};
use serde::{Deserialize, Serialize};

/// Iterator struct is used for building a processor interface
/// * Iterator is lower level of tokenizer and it's used for building a processor interface, Its not advised to use it directly, take a look at [`crate::tokenizer::Pager`] instead
/// ## Fields
/// * `pos` - Active position in iterating process [`defs::CursorPosition`]
/// * `collected` - Collected raw language items [`items::Processor`]
/// * `active` - Iterator's on going processor [`items::ItemProcessor`]
/// * `comment_pos` - Position of comment in iterating process [`defs::CursorPosition`]
/// * `comment_start` - Boolean flag for understanding if comment is started [`bool`]
/// * `line_comment` - Boolean flag for understanding if line comment is started [`bool`]
/// * `multi_comment` - Boolean flag for understanding if multi comment is started [`bool`]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Iterator {
    pub pos: defs::CursorPosition,
    pub collected: Vec<items::Processors>,
    pub errors: Vec<error::Error>,
    pub active: items::ItemProcessor,
}

impl Iterator {
    /// After the last char is processed, this method should be called to finish the iterating process
    pub fn finalize(&mut self) {
        if !self.active.is_complete() {
            if self.active.current.is_initalized() {
                if matches!(self.active.current.clone(), items::Processors::Comment(e) if e.line_comment)
                {
                    self.collected.push(self.active.current.clone());
                } else {
                    self.errors.push(error::error_list::ERROR_S26.clone().build(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        self.active.current.get_pos(),
                    ));
                }
            } else if matches!(self.active.current.as_getter_call(), Some(getter_call) if !getter_call.cache.current.is_not_initialized())
            {
                self.errors.push(error::error_list::ERROR_S26.clone().build(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    self.active.current.get_pos(),
                ));
            }
        }
    }

    /// This method iterates current data
    /// ## Parameters
    /// * `last_char` - Last char of the active char
    /// * `letter_char` - Active char
    /// ## Returns
    /// * `bool` - Returns true if the iterating process is hang
    pub fn iterate(&mut self, last_char: char, letter_char: char) -> bool {
        let mut hang = false;
        let in_str_or_char = matches!(self.active.current.clone(),  items::Processors::GetterCall(e) if e.data.as_string().is_some() || e.data.as_char().is_some());

        let is_escape = letter_char == '\n' || letter_char == '\r' || letter_char == '\t';
        if is_escape && !in_str_or_char {
            self.active
                .iterate(&mut self.errors, self.pos, last_char, ' ');
        } else {
            self.active
                .iterate(&mut self.errors, self.pos, last_char, letter_char);
        }
        if self.errors.iter().any(|e| e.code == 0x00) {
            hang = true;
        }

        let mut dont_inc_column = false;

        if letter_char == '\n' && !in_str_or_char {
            self.pos.0 += 1;
            self.pos.1 = 0;
            dont_inc_column = true;
            if !self.active.is_complete() {
                if let items::Processors::GetterCall(e) = self.active.current.clone() {
                    if !e.cache.current.is_not_initialized() {
                        self.errors.push(error::error_list::ERROR_S26.clone().build(
                            vec![],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            e.pos,
                        ));
                        self.active = items::ItemProcessor::default();
                    }
                } else if let items::Processors::Comment(e) = &mut self.active.current {
                    if e.line_comment {
                        self.collected.push(self.active.current.clone());
                        self.active = items::ItemProcessor::default();
                    } else {
                        e.content.push(String::new());
                    }
                }
            }
        } else if letter_char == '\t' && !in_str_or_char {
            self.pos.1 += 4;
            dont_inc_column = true;
        }

        if !dont_inc_column {
            self.pos.1 += 1;
        } else {
            dont_inc_column = false;
        }

        if self.active.is_complete() {
            if matches!(self.active.current.clone(), items::Processors::Condition(e) if (e.chains.clone()[e.chains.len() - 1].rtype == crate::syntax::items::condition::ConditionType::ElseIf || e.chains.clone()[e.chains.len() - 1].rtype == crate::syntax::items::condition::ConditionType::Else))
            {
                let condition = self.active.current.as_condition().unwrap().clone();
                let last_chain = &condition.chains[condition.chains.len() - 1];

                let collected_len = self.collected.len();
                if collected_len == 0 {
                    self.errors.push(error::error_list::ERROR_S1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: match last_chain.rtype {
                                crate::syntax::items::condition::ConditionType::ElseIf => "else if",
                                crate::syntax::items::condition::ConditionType::Else => "else",
                                crate::syntax::items::condition::ConditionType::If => "",
                            }.to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        last_chain.keyword_pos,
                    ));
                } else if let items::Processors::Condition(past) =
                    &mut self.collected[collected_len - 1]
                {
                    let past_chain_len = past.chains.len() - 1;
                    let past_last_chain = &past.chains[past_chain_len];

                    match past_last_chain.rtype {
                        crate::syntax::items::condition::ConditionType::If => {
                            match last_chain.rtype {
                                crate::syntax::items::condition::ConditionType::ElseIf => {
                                    past.chains.push(last_chain.clone());
                                }
                                crate::syntax::items::condition::ConditionType::Else => {
                                    past.chains.push(last_chain.clone());
                                }
                                _ => (),
                            }
                        }
                        crate::syntax::items::condition::ConditionType::ElseIf => {
                            match last_chain.rtype {
                                crate::syntax::items::condition::ConditionType::ElseIf => {
                                    past.chains.push(last_chain.clone());
                                }
                                crate::syntax::items::condition::ConditionType::Else => {
                                    past.chains.push(last_chain.clone());
                                }
                                _ => (),
                            }
                        }
                        crate::syntax::items::condition::ConditionType::Else => {
                            self.errors.push(error::error_list::ERROR_S1.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "else".to_string(),
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                last_chain.keyword_pos,
                            ));
                        }
                    }
                } else {
                    self.errors.push(error::error_list::ERROR_S1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: match last_chain.rtype {
                                crate::syntax::items::condition::ConditionType::ElseIf => "else if",
                                crate::syntax::items::condition::ConditionType::Else => "else",
                                crate::syntax::items::condition::ConditionType::If => "",
                            }.to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        last_chain.keyword_pos,
                    ));
                }
                self.active = items::ItemProcessor::default();
            } else {
                self.collected.push(self.active.current.clone());
                self.active = items::ItemProcessor::default();
            }
        }
        hang
    }
}
