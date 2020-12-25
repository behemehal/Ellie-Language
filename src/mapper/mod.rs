//TODO: no-std use crate::alloc::string::{String, ToString};
//TODO: no-std use crate::alloc::vec::Vec;

#![allow(warnings)] //TODO Remove this

pub mod defs;
pub mod value_collector;

#[derive(Debug, Clone, PartialEq)]
pub struct Mapped {
    pub items: Vec<Collecting>,
    pub syntax_errors: Vec<crate::error::Error>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Collecting {
    Variable(crate::syntax::variable::VariableCollector),
    None,
}

pub struct Mapper {
    code: String,
    options: defs::MapperOptions,
    collected: Vec<Collecting>,
    pos: defs::CursorPosition,
    ignoreLine: bool,
    current: Collecting,
}

impl Mapper {
    pub fn new(code: String, options: defs::MapperOptions) -> Self {
        Mapper {
            code,
            options,
            collected: Vec::new(),
            pos: defs::CursorPosition(0, 0),
            ignoreLine: false,
            current: Collecting::None,
        }
    }
    pub fn Map(&mut self) -> Mapped {
        let mut errors: Vec<crate::error::Error> = Vec::new();

        pub fn cursorWriteDebug(letter_char: String, row: i64, column: i64) {
            //println!("Scaned | char: {:?}, {}:{} |", letter_char, row + 1, column + 2);
        }

        'charLoop: for (index, char) in self.code.chars().enumerate() {
            let letter_char = &char.to_string();
            let last_char =
                &crate::utils::get_letter(self.code.to_string(), index, false).to_owned();
            let next_char =
                &crate::utils::get_letter(self.code.to_string(), index, true).to_owned();
            let next_next_char =
                &crate::utils::get_letter(self.code.to_string(), index, true).to_owned();

            if char != '\n' && char != '\r' && char != '\t' {
                cursorWriteDebug(letter_char.clone(), self.pos.0, self.pos.1);
            }

            if next_char != "" && (last_char == "\n" || letter_char == "\r") {
                self.pos.0 += 1;
                self.pos.1 = 0;
                continue 'charLoop;
            } else if letter_char == "\t" {
                println!("TAB");
            } else if (letter_char == "d" && next_char == " ") && self.current == Collecting::None {
                self.current = Collecting::Variable(crate::syntax::variable::VariableCollector {
                    initialized: true,
                    r#type: "dynamic".to_string(),
                    ..Default::default()
                });
                self.pos.1 += 1;
                continue 'charLoop;
            } else if (letter_char == "v" && next_char == " ") && self.current == Collecting::None {
                self.current = Collecting::Variable(crate::syntax::variable::VariableCollector {
                    initialized: true,
                    ..Default::default()
                });
                self.pos.1 += 1;
                continue 'charLoop;
            }

            match &mut self.current {
                Collecting::Variable(data) => {
                    if !data.named {
                        if letter_char == ":" {
                            if data.name == "" {
                                errors.push(crate::error::Error {
                                    debug_message: "\\src\\mapper\\mod.rs:90:0".to_string(),
                                    title: crate::error::errorList::error_s1.title.clone(),
                                    code: crate::error::errorList::error_s1.code,
                                    message: crate::error::errorList::error_s1.message.clone(),
                                    builded_message: crate::error::Error::build(
                                        crate::error::errorList::error_s1.message.clone(),
                                        vec![crate::error::ErrorBuildField {
                                            key: "token".to_string(),
                                            value: letter_char.to_string(),
                                        }]
                                    ),
                                    pos: crate::mapper::defs::Cursor {
                                        range_start: self.pos.clone(),
                                        range_end: self.pos.clone().skipChar(1),
                                    },
                                });
                            } else {
                                data.named = true;
                                //println!("|DEBUG| VARIABLE NAMED: {}", data.name);
                            }
                        } else if letter_char == ";" && data.r#type == "dynamic" {
                            //println!("|DEBUG| DYNAMIC VARIABLE COMPLETE: {}", data.r#type);
                            self.collected.push(self.current.clone());
                            self.current = Collecting::None;
                        } else if letter_char == "=" && data.r#type == "dynamic" {
                            if data.name == "" {
                                errors.push(crate::error::Error {
                                    debug_message: "\\src\\mapper\\mod.rs:117:0".to_string(),
                                    title: crate::error::errorList::error_s1.title.clone(),
                                    code: crate::error::errorList::error_s1.code,
                                    message: crate::error::errorList::error_s1.message.clone(),
                                    builded_message: crate::error::Error::build(
                                        crate::error::errorList::error_s1.message.clone(),
                                        vec![crate::error::ErrorBuildField {
                                            key: "token".to_string(),
                                            value: letter_char.to_string(),
                                        }]
                                    ),
                                    pos: crate::mapper::defs::Cursor {
                                        range_start: self.pos.clone(),
                                        range_end: self.pos.clone().skipChar(1),
                                    },
                                });
                            } else {
                                data.named = true;
                                //println!("|DEBUG| VARIABLE NAMED: {}", data.name);
                            }
                        } else {
                            let currentReliability = crate::utils::reliable_name_range(
                                crate::utils::ReliableNameRanges::VariableName,
                                letter_char.clone(),
                            );
                            if currentReliability.reliable {
                                if last_char == " " && data.name != "" {
                                    errors.push(crate::error::Error {
                                        debug_message: "\\src\\mapper\\mod.rs:145:0".to_string(),
                                        title: crate::error::errorList::error_s1.title.clone(),
                                        code: crate::error::errorList::error_s1.code,
                                        message: crate::error::errorList::error_s1.message.clone(),
                                        builded_message: crate::error::Error::build(
                                            crate::error::errorList::error_s1.message.clone(),
                                            vec![crate::error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: letter_char.to_string(),
                                            }]
                                        ),
                                        pos: crate::mapper::defs::Cursor {
                                            range_start: self.pos.clone(),
                                            range_end: self.pos.clone().skipChar(1),
                                        },
                                    });
                                } else {
                                    data.name = data.name.clone() + &letter_char.clone();
                                }
                            } else if letter_char != " "
                                && (letter_char != ":" || letter_char != "=" || letter_char != ";")
                                && (last_char == " " || data.name != "")
                            {
                                errors.push(crate::error::Error {
                                    debug_message: "\\src\\mapper\\mod.rs:169:0".to_string(),
                                    title: crate::error::errorList::error_s1.title.clone(),
                                    code: crate::error::errorList::error_s1.code,
                                    message: crate::error::errorList::error_s1.message.clone(),
                                    builded_message: crate::error::Error::build(
                                        crate::error::errorList::error_s1.message.clone(),
                                        vec![crate::error::ErrorBuildField {
                                            key: "token".to_string(),
                                            value: letter_char.to_string(),
                                        }]
                                    ),
                                    pos: crate::mapper::defs::Cursor {
                                        range_start: self.pos.clone(),
                                        range_end: self.pos.clone().skipChar(1),
                                    },
                                });
                            }
                        }
                    } else if !data.typed && data.r#type != "dynamic" {
                        if letter_char == ";" {
                            self.collected.push(self.current.clone());
                            self.current = Collecting::None;
                        } else if letter_char == "=" {
                            if data.r#type == "" {
                                errors.push(crate::error::Error {
                                    debug_message: "\\src\\mapper\\mod.rs:194:0".to_string(),
                                    title: crate::error::errorList::error_s1.title.clone(),
                                    code: crate::error::errorList::error_s1.code,
                                    message: crate::error::errorList::error_s1.message.clone(),
                                    builded_message: crate::error::Error::build(
                                        crate::error::errorList::error_s1.message.clone(),
                                        vec![crate::error::ErrorBuildField {
                                            key: "token".to_string(),
                                            value: letter_char.to_string(),
                                        }]
                                    ),
                                    pos: crate::mapper::defs::Cursor {
                                        range_start: self.pos.clone(),
                                        range_end: self.pos.clone().skipChar(1),
                                    },
                                });
                            } else {
                                data.typed = true;
                            }
                        } else {
                            let currentReliability = crate::utils::reliable_name_range(
                                crate::utils::ReliableNameRanges::VariableName,
                                letter_char.clone(),
                            );
                            if currentReliability.reliable
                                && (data.r#type == "" || (last_char != " "))
                            {
                                data.r#type = data.r#type.clone() + &letter_char.clone();
                            } else if data.r#type != ""
                                && (last_char == " " && (letter_char != ":" && letter_char != " "))
                            {
                                errors.push(crate::error::Error {
                                    debug_message: "\\src\\mapper\\mod.rs:226:0".to_string(),
                                    title: crate::error::errorList::error_s1.title.clone(),
                                    code: crate::error::errorList::error_s1.code,
                                    message: crate::error::errorList::error_s1.message.clone(),
                                    builded_message: crate::error::Error::build(
                                        crate::error::errorList::error_s1.message.clone(),
                                        vec![crate::error::ErrorBuildField {
                                            key: "token".to_string(),
                                            value: letter_char.to_string(),
                                        }]
                                    ),
                                    pos: crate::mapper::defs::Cursor {
                                        range_start: self.pos.clone(),
                                        range_end: self.pos.clone().skipChar(1),
                                    },
                                });
                            }
                        }
                    } else {
                        let mut collecter_response: bool = false;
                        if letter_char == ";" {
                            self.collected.push(self.current.clone());
                            self.current = Collecting::None;

                        //if !collecter_response {
                        //    println!(
                        //        "| MAPPING HALTED | Unexpected token at: {}:{} '{}'|",
                        //        self.pos.0, self.pos.1, letter_char
                        //    );
                        //} else {
                        //}
                        } else {
                            let mut cd = data.clone();
                            let collected = value_collector::collect(
                                &mut cd,
                                letter_char.to_string(),
                                next_char.to_string(),
                                last_char.to_string(),
                                self.pos.clone(),
                            );
                            self.current = Collecting::Variable(collected.itered_data);
                        }
                    }
                }
                _ => (),
            };
            if char != '\n' && char != '\r' && char != '\t' {
                self.pos.1 += 1;
            }
        }
        Mapped {
            items: self.collected.clone(),
            syntax_errors: errors,
        }
    }
}
