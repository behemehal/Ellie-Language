pub use crate::syntax::items::setter_call::SetterCall;

impl crate::processors::Processor for SetterCall {
    fn iterate(
        &mut self,
        errors: &mut Vec<ellie_core::error::Error>,
        cursor: ellie_core::defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;
        if self.cache.is_complete() && letter_char == ';' {
            self.complete = true;
            self.value = self.cache.current.clone();
            self.pos.range_end = cursor.clone().skip_char(1);
        } else {
            if self.cache.current.is_not_initialized() {
                self.pos.range_start = cursor;
            }
            hang = self.cache.iterate(errors, cursor, last_char, letter_char);
        }
        hang
    }
}
