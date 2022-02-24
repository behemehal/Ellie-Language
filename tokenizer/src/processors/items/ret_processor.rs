use crate::syntax::items::ret::Ret;
use ellie_core::{defs, error};

impl crate::processors::Processor for Ret {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;
        if self.value.is_complete() && letter_char == ';' {
            self.complete = true;
            self.value_position.range_end = cursor.clone().skip_char(1);
            self.pos.range_end = cursor.clone().skip_char(1);
        } else {
            if letter_char != ' ' && self.value_position.range_start.is_zero() {
                self.value_position.range_start = cursor.clone().skip_char(1);
            }
            hang = self.value.iterate(errors, cursor, last_char, letter_char);
        }
        hang
    }
}
