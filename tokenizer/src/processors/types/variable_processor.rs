use crate::processors::Processor;
use crate::syntax::types::variable_type;
use ellie_core::{
    defs, error,
    utils::{reliable_name_range, ReliableNameRanges},
};

impl Processor for variable_type::VariableTypeCollector {
    fn new() -> Self {
        variable_type::VariableTypeCollector::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if reliable_name_range(ReliableNameRanges::VariableName, letter_char).reliable {
            if last_char == ' ' && self.data.value != "" {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "var_0x38".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            } else {
                self.complete = true;
                self.data.value += &letter_char.to_string();
            }
        } else if letter_char != ' ' {
            errors.push(error::errorList::error_s1.clone().build(
                vec![error::ErrorBuildField {
                    key: "token".to_string(),
                    value: letter_char.to_string(),
                }],
                "var_0x47".to_owned(),
                defs::Cursor::build_with_skip_char(cursor),
            ));
        }
    }
}