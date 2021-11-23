pub mod array_processor;
pub mod char_processor;
pub mod float_processor;
pub mod integer_processor;
pub mod operator_processor;
pub mod string_processor;
pub mod variable_processor;

use super::Processor;
use crate::syntax::types::*;
use ellie_core::{definite, utils};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Processors {
    Integer(integer_type::IntegerTypeCollector),
    Float(float_type::FloatTypeCollector),
    Char(char_type::CharType),
    String(string_type::StringTypeCollector),
    Variable(variable_type::VariableTypeCollector),
    Array(array_type::ArrayTypeCollector),
    Operator(operator_type::OperatorTypeCollector),
}
impl Processors {
    pub fn to_definite(&self) -> ellie_core::definite::types::Types {
        match self.clone() {
            Processors::Integer(e) => ellie_core::definite::types::Types::Integer(e.to_definite()),
            Processors::Float(e) => ellie_core::definite::types::Types::Float(e.to_definite()),
            Processors::Char(e) => ellie_core::definite::types::Types::Char(e.to_definite()),
            Processors::String(e) => ellie_core::definite::types::Types::String(e.to_definite()),
            Processors::Variable(e) => {
                ellie_core::definite::types::Types::VariableType(e.to_definite())
            }
            Processors::Array(e) => ellie_core::definite::types::Types::Array(e.to_definite()),
            Processors::Operator(e) => {
                ellie_core::definite::types::Types::Operator(e.to_definite())
            }
        }
    }

    pub fn from_definite(self, from: definite::types::Types) -> Self {
        match from {
            definite::types::Types::Integer(e) => {
                Processors::Integer(integer_type::IntegerTypeCollector::default().from_definite(e))
            }
            definite::types::Types::Float(e) => {
                Processors::Float(float_type::FloatTypeCollector::default().from_definite(e))
            }
            definite::types::Types::String(e) => {
                Processors::String(string_type::StringTypeCollector::default().from_definite(e))
            }
            definite::types::Types::Char(e) => {
                Processors::Char(char_type::CharType::default().from_definite(e))
            }
            definite::types::Types::VariableType(e) => Processors::Variable(
                variable_type::VariableTypeCollector::default().from_definite(e),
            ),
            definite::types::Types::Array(e) => {
                Processors::Array(array_type::ArrayTypeCollector::default().from_definite(e))
            }
            _ => panic!("NOT SUPPORTED"),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self.clone() {
            Processors::Integer(e) => e.complete,
            Processors::Char(e) => e.complete,
            Processors::String(e) => e.complete,
            Processors::Variable(e) => e.complete,
            Processors::Float(e) => e.complete,
            Processors::Array(e) => e.complete,
            Processors::Operator(e) => e.data.second.is_complete(),
        }
    }
}

impl Default for Processors {
    fn default() -> Self {
        Processors::Variable(variable_type::VariableTypeCollector::default())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TypeProcessor {
    pub current: Processors,
    pub ignore: bool,
}

impl TypeProcessor {
    pub fn is_complete(&self) -> bool {
        self.current.is_complete() && !self.ignore
    }
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
                self.current = Processors::Array(array_type::ArrayTypeCollector::default());
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
        } else if self.is_complete() && letter_char == '.' {
        } else if self.is_complete() && utils::is_operator_start(letter_char) {
            //Operator priority
            if let Processors::Operator(operator) = self.current.clone() {
                if letter_char == '/' || letter_char == '*' || letter_char == '%' {
                    self.current = Processors::Operator(operator_type::OperatorTypeCollector {
                        data: operator_type::OperatorType {
                            first: operator.data.first,
                            operator: operator.data.operator,
                            ..Default::default()
                        },
                        itered_cache: Box::new(TypeProcessor {
                            current: Processors::Operator(operator_type::OperatorTypeCollector {
                                data: operator_type::OperatorType {
                                    first: operator.data.second,
                                    //operator: operator.data.operator.clone(),
                                    ..Default::default()
                                },
                                first_filled: true,
                                ..Default::default()
                            }),
                            ignore: false,
                        }),
                        operator_collected: true,
                        first_filled: true,
                        ..Default::default()
                    });
                } else {
                    self.current = Processors::Operator(operator_type::OperatorTypeCollector {
                        data: operator_type::OperatorType {
                            first: Box::new(self.current.clone()),
                            ..Default::default()
                        },
                        first_filled: true,
                        ..Default::default()
                    });
                }
            } else {
                self.current = Processors::Operator(operator_type::OperatorTypeCollector {
                    data: operator_type::OperatorType {
                        first: Box::new(self.current.clone()),
                        ..Default::default()
                    },
                    first_filled: true,
                    ..Default::default()
                });
            }
        }

        match &mut self.current {
            Processors::Integer(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Char(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::String(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Variable(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Float(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Array(e) => e.iterate(errors, cursor, last_char, letter_char),
            Processors::Operator(e) => e.iterate(errors, cursor, last_char, letter_char),
        };
    }
}
