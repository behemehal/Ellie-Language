//TODO: no-std use crate::alloc::string::{String, ToString};
//TODO: no-std use crate::alloc::vec::Vec;

#![allow(warnings)] //TODO Remove this

use ellie_core::{defs, error, utils};
use crate::parser;
use crate::processors;
use crate::syntax::{condition, function, types, variable};

use crate::alloc::vec::Vec;
use crate::alloc::string::{String, ToString};

#[derive(Debug, Clone, PartialEq)]
pub struct Parsed {
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
pub struct Parser {
    pub code: String,
    pub options: defs::ParserOptions,
    pub collected: Vec<Collecting>,
    pub pos: defs::CursorPosition,
    pub ignore_line: bool,
    pub current: Collecting,
    pub keyword_catch: String,
}

impl Parser {
    pub fn type_Check(&self, r#type: String) -> bool {
        true
    }

    pub fn new(code: String, options: defs::ParserOptions) -> Self {
        Parser {
            code,
            options,
            collected: Vec::new(),
            pos: defs::CursorPosition(0, 0),
            ignore_line: false,
            current: Collecting::None,
            keyword_catch: String::new(),
        }
    }
    pub fn Map(mut self) -> Parsed {
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
                &utils::get_letter(self.code.clone().to_string(), index, true).to_string();
            let next_next_char =
                &utils::get_letter(self.code.clone().to_string(), index + 1, true).to_string();
            let next_next_next_char =
                &utils::get_letter(self.code.clone().to_string(), index + 2, true).to_string();

            if char != '\n' && char != '\r' && char != '\t' {
                if self.current == Collecting::None {
                    self.keyword_catch += letter_char;
                    processors::type_processor::collect(
                        &mut self,
                        letter_char,
                        next_char.clone(),
                        next_next_char.clone(),
                        next_next_next_char.clone()
                    );
                } else {
                    self.keyword_catch = String::new();
                }

                match self.current {
                    Collecting::Variable(_) => processors::variable_processor::collect(
                        &mut self,
                        &mut errors,
                        letter_char,
                        next_char.clone(),
                        last_char.clone(),
                    ),
                    Collecting::Condition(_) => processors::condition_processor::collect(
                        &mut self,
                        &mut errors,
                        letter_char,
                        next_char.clone(),
                        last_char.clone(),
                    ),
                    Collecting::Function(_) => processors::function_processor::collect(
                        &mut self,
                        &mut errors,
                        letter_char,
                        next_char.clone(),
                        last_char.clone(),
                    ),
                    (_) => ()
                }
                self.pos.1 += 1;
            } else if (last_char == "\r" || letter_char == "\n") {
                self.pos.0 += 1;
                self.pos.1 = 0;
            }
        }
        Parsed {
            items: self.collected.clone(),
            syntax_errors: errors,
        }
    }
}
