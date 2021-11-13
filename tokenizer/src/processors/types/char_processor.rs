use crate::processors::Processor;
use ellie_core::{definite::types::integer, defs, error, utils::reliable_name_range};

#[derive(Default, Clone, Debug)]
pub struct CharProcessor {
    pub comma_start_pos: defs::Cursor,
    pub comma_end_pos: defs::Cursor,
    pub comma_started: bool,
    pub expect_end: bool,
    pub errors: Vec<error::Error>,
    pub cursor: defs::CursorPosition,
    pub forward: Option<ellie_core::definite::types::Types>,
    pub complete: bool,
}

impl Processor for CharProcessor {
    fn new() -> Self {
        CharProcessor::default()
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
        if !self.comma_started {
            if letter_char == '\'' {
                self.comma_started = true;
                self.comma_start_pos = defs::Cursor::build_with_skip_char(cursor);
            } else if letter_char != ' ' {
                self.errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "0x36".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else {
            if letter_char == '\'' && last_char != '\\' {
                self.complete = true;
                self.comma_end_pos = defs::Cursor::build_with_skip_char(cursor);
            } else {
            }
        }
    }
}
