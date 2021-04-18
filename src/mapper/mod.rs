//TODO: no-std use crate::alloc::string::{String, ToString};
//TODO: no-std use crate::alloc::vec::Vec;

#![allow(warnings)] //TODO Remove this

pub mod defs;

use crate::error;
use crate::mapper;
use crate::processors;
use crate::syntax::{condition, function, types, variable};
use crate::utils;

#[derive(Debug, Clone, PartialEq)]
pub struct Mapped {
    pub items: Vec<Collecting>,
    pub syntax_errors: Vec<error::Error>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Collecting {
    Variable(variable::VariableCollector),
    Function(function::FunctionCollector),
    Condition(condition::ConditionCollector),
    None,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Mapper {
    pub code: String,
    pub options: defs::MapperOptions,
    pub collected: Vec<Collecting>,
    pub pos: defs::CursorPosition,
    pub ignore_line: bool,
    pub current: Collecting,
}

impl Mapper {
    pub fn new(code: String, options: defs::MapperOptions) -> Self {
        Mapper {
            code,
            options,
            collected: Vec::new(),
            pos: defs::CursorPosition(0, 0),
            ignore_line: false,
            current: Collecting::None,
        }
    }
    pub fn Map(&mut self) -> Mapped {
        let mut errors: Vec<error::Error> = Vec::new();

        pub fn cursorWriteDebug(letter_char: String, row: i64, column: i64) {
            //println!(
            //    "Scaned | char: {:?}, {}:{} |",
            //    letter_char,
            //    row,
            //    column
            //);
        }

        'charLoop: for (index, char) in self.code.clone().chars().enumerate() {
            let letter_char = &char.to_string();
            let last_char =
                &utils::get_letter(self.code.clone().to_string(), index, false).to_string();
            let next_char =
                &utils::get_letter(self.code.clone().to_string(), index, true).to_owned();
            let next_next_char =
                &utils::get_letter(self.code.clone().to_string(), index + 1, true).to_owned();
            let next_next_next_char =
                &utils::get_letter(self.code.clone().to_string(), index + 2, true).to_owned();

            if char != '\n' && char != '\r' && char != '\t' {
                if (letter_char == "d" && next_char == " ") && self.current == Collecting::None {
                    self.current = Collecting::Variable(variable::VariableCollector {
                        initialized: true,
                        data: variable::Variable {
                            dynamic: true,
                            pos: defs::Cursor {
                                range_start: self.pos,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    continue 'charLoop;
                } else if (letter_char == "v" && next_char == " ")
                    && self.current == Collecting::None
                {
                    self.current = Collecting::Variable(variable::VariableCollector {
                        initialized: true,
                        data: variable::Variable {
                            pos: defs::Cursor {
                                range_start: self.pos,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    continue 'charLoop;
                } else if (letter_char == "f"
                    && next_char == "n"
                    && next_next_char == " "
                    && self.current == Collecting::None)
                {
                    self.current = Collecting::Function(function::FunctionCollector::default());
                    continue 'charLoop;
                } else if (letter_char == "e"
                    && next_char == "l"
                    && next_next_char == "s"
                    && next_next_next_char == "e")
                    && self.current == Collecting::None
                {
                    let collected_length = self.collected.clone().len();
                    if collected_length == 0 {
                        panic!("Error");
                    } else if let Collecting::Condition(value) =
                        &mut self.collected[collected_length - 1]
                    {
                        let mut repeated_condition = condition::ConditionCollector::default();
                        repeated_condition.chains = value.chains.clone();
                        repeated_condition.might_be_else_if = true;
                        self.current = mapper::Collecting::Condition(repeated_condition);
                        self.collected.remove(collected_length - 1);
                    } else {
                        //User used else statement without if
                        panic!("Error: {:#?}", self.collected);
                    }
                } else if (letter_char == "i" && next_char == "f" && next_next_char == " ")
                    && self.current == Collecting::None
                {
                    self.current = Collecting::Condition(condition::ConditionCollector {
                        ..Default::default()
                    });
                }

                match self.current {
                    Collecting::Variable(_) => processors::variable_processor::collect(
                        self,
                        &mut errors,
                        letter_char,
                        next_char.clone(),
                        last_char.clone(),
                    ),
                    Collecting::Condition(_) => processors::condition_processor::collect(
                        self,
                        &mut errors,
                        letter_char,
                        next_char.clone(),
                        last_char.clone(),
                    ),
                    Collecting::Function(_) => processors::function_processor::collect(
                        self,
                        &mut errors,
                        letter_char,
                        next_char.clone(),
                        last_char.clone(),
                    ),
                    (_) => (),
                }
                self.pos.1 += 1;
            } else if (last_char == "\r" || letter_char == "\n") {
                self.pos.0 += 1;
                self.pos.1 = 0;
            }
        }
        Mapped {
            items: self.collected.clone(),
            syntax_errors: errors,
        }
    }
}
