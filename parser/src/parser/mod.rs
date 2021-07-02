use serde::Serialize;

pub mod iterator;
pub mod scope;

use crate::syntax::{class, condition, constructor, forloop, function, import, ret, variable, caller};
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
    Forloop(forloop::ForloopCollector),
    Condition(condition::ConditionCollector),
    Class(class::ClassCollector),
    Ret(ret::Ret),
    Constructor(constructor::ConstructorCollector),
    Caller(caller::Caller),
    Import(import::Import),
    Getter,
    Setter,
    None,
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Parser {
    pub scope: String,
    pub code: String,
    pub options: defs::ParserOptions,
    pub collected: Vec<Collecting>,
    pub pos: defs::CursorPosition,
    pub ignore_line: bool,
    pub current: Collecting,
    keyword_pos: defs::Cursor,
    pub keyword_catch: String,
    pub keyword_cache: variable::VariableCollector
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            scope: "".to_string(),
            code: "".to_string(),
            options: defs::ParserOptions::default(),
            collected: Vec::new(),
            pos: defs::CursorPosition(0, 0),
            keyword_pos: defs::Cursor::default(),
            ignore_line: false,
            current: Collecting::None,
            keyword_catch: String::new(),
            keyword_cache: variable::VariableCollector::default()
        }
    }
}

impl Parser {
    pub fn new(code: String, scope: String, options: defs::ParserOptions) -> Self {
        Parser {
            scope,
            code,
            options,
            collected: Vec::new(),
            pos: defs::CursorPosition(0, 0),
            keyword_pos: defs::Cursor::default(),
            ignore_line: false,
            current: Collecting::None,
            keyword_catch: String::new(),
            keyword_cache: variable::VariableCollector::default()
        }
    }

    pub fn map(mut self) -> Parsed {
        let mut errors: Vec<error::Error> = Vec::new();

        for (index, char) in self.code.clone().chars().enumerate() {
            let letter_char = &char.to_string();
            let last_char =
                &utils::get_letter(self.code.clone().to_string(), index, false).to_string();
            let next_char =
                &utils::get_letter(self.code.clone().to_string(), index, true).to_string();

            if char != '\n' && char != '\r' && char != '\t' {
                iterator::iter(
                    &mut self,
                    &mut errors,
                    letter_char,
                    next_char.to_string(),
                    last_char.to_string(),
                );
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
