//TODO: no-std use crate::alloc::string::{String, ToString};
//TODO: no-std use crate::alloc::vec::Vec;

use serde::Serialize;

use crate::processors;
use crate::syntax::{condition, function, variable};
use ellie_core::{defs, error, utils};

use crate::alloc::string::{String, ToString};
use crate::alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Parsed {
    pub items: Vec<Collecting>,
    pub syntax_errors: Vec<error::Error>,
}

#[derive(PartialEq, Debug, Clone, Serialize)]
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
    pub fn map(mut self) -> Parsed {
        let mut errors: Vec<error::Error> = Vec::new();
        let parser_options = self.options.clone();

        for (index, char) in self.code.clone().chars().enumerate() {
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
                    processors::type_processor::collect_type(
                        &mut self,
                        letter_char,
                        next_char.clone(),
                        next_next_char.clone(),
                        next_next_next_char.clone(),
                    );
                } else {
                    self.keyword_catch = String::new();
                }

                match self.current {
                    Collecting::Variable(_) => {
                        processors::variable_processor::collect_variable_value(
                            &mut self,
                            &mut errors,
                            letter_char,
                            next_char.clone(),
                            last_char.clone(),
                            parser_options.clone(),
                        )
                    }
                    Collecting::Condition(_) => processors::condition_processor::collect_condition(
                        &mut self,
                        &mut errors,
                        letter_char,
                        next_char.clone(),
                        last_char.clone(),
                        parser_options.clone(),
                    ),
                    Collecting::Function(_) => processors::function_processor::collect_function(
                        &mut self,
                        &mut errors,
                        letter_char,
                        next_char.clone(),
                        last_char.clone(),
                        parser_options.clone(),
                    ),
                    _ => (),
                }
                self.pos.1 += 1;
            } else if last_char == "\r" || letter_char == "\n" {
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
