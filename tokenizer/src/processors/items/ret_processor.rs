use crate::syntax::items::ret::Ret;
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for Ret {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if self.value.is_complete() && letter_char == ';' {
            self.complete = true;
            self.value_position.range_end = cursor.clone().skip_char(1);
            self.pos.range_end = cursor.clone().skip_char(1);
        } else {
            if self.value.current.is_not_initialized() {
                self.value_position.range_start = cursor;
            }
            self.value.iterate(errors, cursor, last_char, letter_char)
        }
    }
}
