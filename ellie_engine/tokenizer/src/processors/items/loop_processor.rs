use crate::{processors::EscapeCharEmitter, syntax::items::loop_type::Loop};
use ellie_core::{defs, error};

impl crate::processors::Processor for Loop {
    fn emits_line_endings(&self) -> EscapeCharEmitter {
        self.iterator.emits_line_endings()
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;
        if !self.condition_filled {
            if self.condition.is_complete() && letter_char == '{' {
                self.condition_filled = true;
                self.condition_pos = self.condition.current.get_pos();
            } else {
                hang = self
                    .condition
                    .iterate(errors, cursor, last_char, letter_char);
            }
        } else if letter_char == '}' && self.brace_count == 0 {
            self.complete = true;
            self.body_pos.range_end = cursor;
            self.pos.range_end = cursor;
            self.iterator.finalize();
            errors.extend(self.iterator.errors.clone());
            self.body = self.iterator.collected.clone();
        } else {
            if letter_char == '{' {
                self.brace_count += 1;
            } else if letter_char == '}' && self.brace_count != 0 {
                self.brace_count -= 1;
            }
            self.iterator.pos = cursor;
            hang = self.iterator.iterate(last_char, letter_char);
        }
        hang
    }
}
