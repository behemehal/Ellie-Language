use crate::processors::Processor;
use ellie_core::{defs, error, utils};

#[derive(Default)]
pub struct VariableProcessor {
    pub name_collected: bool,
    pub name: String,
    pub type_collected: bool,
    pub rtype: String,
    pub value_collected: bool,
    pub value: String,
    pub errors: Vec<ellie_core::error::Error>,
    pub forward: ellie_core::definite::items::Collecting,
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

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.name_collected {
            if utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
                .reliable
            {
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
