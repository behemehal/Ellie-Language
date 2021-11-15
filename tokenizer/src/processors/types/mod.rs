pub mod char_processor;
pub mod float_processor;
pub mod integer_processor;
pub mod string_processor;
pub mod variable_processor;

use super::Processor;
use crate::syntax::types::*;

#[derive(Debug, Clone)]
pub enum Processors {
    Integer(integer_type::IntegerTypeCollector),
    Float(float_type::FloatTypeCollector),
    Char(char_type::CharType),
    String(string_type::StringTypeCollector),
    Variable(variable_type::VariableTypeCollector),
}

impl Processors {
    pub fn is_complete(&self) -> bool {
        match self {
            Processors::Integer(e) => e.complete,
            Processors::Char(e) => e.complete,
            Processors::String(e) => e.complete,
            Processors::Variable(e) => e.complete,
            Processors::Float(e) => e.complete,
        }
    }
}

impl Default for Processors {
    fn default() -> Self {
        Processors::Variable(variable_type::VariableTypeCollector::default())
    }
}

#[derive(Default, Debug, Clone)]
pub struct TypeProcessor {
    pub current: Processors,
}

impl Processor for TypeProcessor {
    fn new() -> Self {
        TypeProcessor::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<ellie_core::error::Error>,
        cursor: ellie_core::defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        let not_initalized = matches!(&self.current, Processors::Variable(x) if x.data.value == "");

        if letter_char == '{' && not_initalized {
            todo!("Collective not supported yet");
        } else if letter_char == '[' {
            if not_initalized {
                todo!("Collective not supported yet");
            } else {
                todo!("BraceReference not supported yet");
            }
        } else if letter_char == '('
            && (not_initalized
                || matches!(&self.current, Processors::Variable(x) if x.data.value != ""))
        {
            if not_initalized {
                todo!("Cloak not supported yet");
            } else {
                todo!("FunctionCall not supported yet");
            }
        } else if letter_char == '\'' && not_initalized {
            self.current = Processors::Char(char_type::CharType::default());
        } else if letter_char == '"' && not_initalized {
            self.current = Processors::String(string_type::StringTypeCollector::default());
        } else if letter_char == '.'
            && (not_initalized || matches!(&self.current, Processors::Integer(_)))
        {
            self.current = Processors::Float(float_type::FloatTypeCollector {
                base: if let Processors::Integer(e) = &self.current {
                    e.raw.to_string()
                } else {
                    "0.".to_string()
                },
                data: float_type::FloatType {
                    raw: if let Processors::Integer(e) = &self.current {
                        e.raw.to_string() + "."
                    } else {
                        "0.".to_string()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
        } else if letter_char.to_string().parse::<i8>().is_ok() && not_initalized {
            self.current = Processors::Integer(integer_type::IntegerTypeCollector::default());
        } else if self.current.is_complete() && letter_char == '.' {
            panic!("Reference not supported yet")
        }

        match &mut self.current {
            Processors::Integer(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Char(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::String(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Variable(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Float(e) => e.iterate(errors, cursor, last_char, letter_char),
        };
    }
}
