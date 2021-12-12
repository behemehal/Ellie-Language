use ellie_core::{defs, error};
use serde::{Deserialize, Serialize};

use crate::processors::{
    items::{self},
    Processor,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Iterator {
    pub pos: defs::CursorPosition,
    pub collected: Vec<items::Processors>,
    pub errors: Vec<error::Error>,
    pub active: items::ItemProcessor,
    pub comment_pos: defs::Cursor,
    pub comment_start: bool,
    pub line_comment: bool,
    pub multi_comment: bool,
}

impl Iterator {
    pub fn finalize(&mut self) {
        if !self.active.is_complete() && self.active.current.is_initalized() {
            self.errors.push(error::error_list::ERROR_S26.clone().build(
                vec![],
                "tok_0x23".to_owned(),
                self.active.current.get_pos(),
            ));
        } else if self.multi_comment {
            self.errors.push(error::error_list::ERROR_S26.clone().build(
                vec![],
                "ite_0x29".to_owned(),
                self.comment_pos,
            ));
        }
    }

    pub fn iterate(&mut self, last_char: char, letter_char: char) {
        let in_str_or_char = matches!(self.active.current.clone(),  items::Processors::GetterCall(e) if e.data.as_string().is_some() || e.data.as_char().is_some());

        if self.comment_start {
            if letter_char == '/' {
                self.comment_start = false;
                self.line_comment = true;
            } else if letter_char == '*' {
                self.comment_start = false;
                self.multi_comment = true;
            } else {
                self.errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "ite_0x67".to_owned(),
                    defs::Cursor::build_with_skip_char(self.pos),
                ));
            }
        }

        if (letter_char != '\n' && letter_char != '\r' || in_str_or_char)
            && !self.line_comment
            && !self.multi_comment
        {
            if !self.active.is_complete() {
                if let items::Processors::GetterCall(e) = self.active.current.clone() {
                    if e.data.is_not_initialized() && (letter_char == '/' && !in_str_or_char) {
                        self.comment_pos.range_start = self.pos;
                        self.comment_start = true;
                    }
                }
            }

            if !self.comment_start {
                self.active
                    .iterate(&mut self.errors, self.pos, last_char, letter_char);
            }
        }

        if letter_char == '\n' && !in_str_or_char {
            if self.line_comment {
                self.line_comment = false;
            }
            self.pos.0 += 1;
            self.pos.1 = 0;
            if !self.active.is_complete() {
                if let items::Processors::GetterCall(e) = self.active.current.clone() {
                    if !e.cache.current.is_not_initialized() {
                        self.errors.push(error::error_list::ERROR_S26.clone().build(
                            vec![],
                            "tok_0x68".to_owned(),
                            e.pos,
                        ));
                        self.active = items::ItemProcessor::default();
                    }
                }
            }
        }
        self.pos.1 += 1;

        if self.multi_comment {
            if letter_char == '/' && last_char == '*' {
                self.multi_comment = false;
            } else {
                self.comment_pos.range_end = self.pos;
            }
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
                        "ite_0x141".to_owned(),
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
                                "ite_0x141".to_owned(),
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
                        "ite_0x153".to_owned(),
                        last_chain.keyword_pos,
                    ));
                }
                self.active = items::ItemProcessor::default();
            } else {
                self.collected.push(self.active.current.clone());
                self.active = items::ItemProcessor::default();
            }
        }
    }
}
