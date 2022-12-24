pub use crate::syntax::items::getter_call::GetterCall;

impl crate::processors::Processor for GetterCall {
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
            self.data = self.cache.current.clone();
        } else {
            if self.cache.current.is_not_initialized() && letter_char != ' ' {
                self.pos.range_start = cursor;
            }
            hang = self.cache.iterate(errors, cursor, last_char, letter_char);
        }
        self.pos.range_end = cursor;
        hang
    }
}
