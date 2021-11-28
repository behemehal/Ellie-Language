use crate::processors::types::Processor;
pub use crate::syntax::items::getter_call::GetterCall;

impl super::Processor for GetterCall {
    fn iterate(
        &mut self,
        errors: &mut Vec<ellie_core::error::Error>,
        cursor: ellie_core::defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if self.cache.is_complete() && letter_char == ';' {
            self.complete = true;
            self.data = self.cache.current.clone();
        } else {
            if self.cache.current.is_not_initialized() {
                self.pos.range_start = cursor;
            }
            self.cache.iterate(errors, cursor, last_char, letter_char);
        }
        self.pos.range_end = cursor.clone().skip_char(1);
    }
}
