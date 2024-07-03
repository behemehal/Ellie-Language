use crate::processors::EscapeCharEmitter;
pub use crate::syntax::items::setter_call::SetterCall;

impl crate::processors::Processor for SetterCall {
    fn emits_line_endings(&self) -> EscapeCharEmitter {
        self.cache.emits_line_endings()
    }

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
        } else {
            if self.cache.current.is_not_initialized() {
                self.value_pos.range_start = cursor;
            }
            self.value_pos.range_end = cursor;
            self.hash = ellie_core::utils::generate_hash_usize();
            hang = self.cache.iterate(errors, cursor, last_char, letter_char);
        }
        hang
    }
}
