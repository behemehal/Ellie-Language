use crate::processors::{reliable_char, Processor};
use ellie_core::{error, defs};

#[derive(Default)]
pub struct VariableProcessor {
    pub name_collected: bool,
    pub name: String,
    pub type_collected: bool,
    pub rtype: String,
    pub value_collected: bool,
    pub value: String,
    pub errors: Vec<ellie_core::error::Error>,
}

impl Processor for VariableProcessor {
    fn new() -> Self {
        VariableProcessor::default()
    }

    fn keyword(&self) -> &str {
        "v"
    }

    fn has_accessibility(&self) -> bool {
        true
    }

    fn is_complete(&self) -> bool {
        self.name_collected && self.type_collected && self.value_collected
    }

    fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }

    fn errors(&self) -> Vec<error::Error> {
        self.errors.clone()
    }

    fn iterate(&mut self, cursor: defs::CursorPosition, last_char: char, letter_char: char) {
        if !self.name_collected {
            if reliable_char(&letter_char) {
                if last_char == ' ' {
                    panic!("Error: UNEXPECTED TOKEN, {:#?}", letter_char);
                } else {
                    self.name += &letter_char.to_string();
                }
            } else if self.name != "" {
                if letter_char == ':' {
                    panic!("COLLECTED: GOTO TYPE");
                } else if letter_char == '=' {
                    panic!("COLLECTED: GOTO VALUE");
                } else if letter_char != ' ' {
                    panic!("?Error: UNEXPECTED TOKEN, {:?}", letter_char);
                }
            }
        } else if !self.type_collected {
        } else if !self.value_collected {
        }
    }
}
