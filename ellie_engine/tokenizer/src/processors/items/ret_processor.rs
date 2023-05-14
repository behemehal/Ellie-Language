use crate::{syntax::items::ret::Ret, processors::EscapeCharEmitter};
use ellie_core::{defs, error};

impl crate::processors::Processor for Ret {
    fn emits_line_endings(&self) -> EscapeCharEmitter {
        self.value.emits_line_endings()
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;
        if (self.value.is_complete() || !self.not_empty) && letter_char == ';' {
            self.complete = true;
            self.pos.range_end = cursor;
        } else {
            if letter_char != ' ' && self.value_position.range_start.is_zero() {
                self.value_position.range_start = cursor;
                self.value_position.range_end = cursor;
            }
            if letter_char != ' ' && !self.not_empty {
                self.not_empty = true;
            }
            hang = self.value.iterate(errors, cursor, last_char, letter_char);
        }
        hang
    }
}
