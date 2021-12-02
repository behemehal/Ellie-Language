use crate::syntax::items::for_loop::ForLoop;
use ellie_core::{defs, error};

impl crate::processors::Processor for ForLoop {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if self.parameter.is_zero() {
            self.parameter.range_start = cursor;
        }
        if !self.condition_filled {
            if !self.variable_filled {
                if self.variable.is_complete() && letter_char == ':' {
                    self.variable_filled = true;
                } else {
                    self.variable
                        .iterate(errors, cursor, last_char, letter_char);
                }
            } else {
                if self.target_iterator.is_complete() && letter_char == '{' {
                    self.condition_filled = true;
                    self.parameter.range_end = cursor;
                    self.body_pos.range_start = cursor;
                } else {
                    self.target_iterator
                        .iterate(errors, cursor, last_char, letter_char);
                }
            }
        } else if letter_char == '}' && self.brace_count == 0 {
            self.complete = true;
            self.body_pos.range_end = cursor;
            self.iterator.finalize();
            errors.extend(self.iterator.errors.clone());
            self.body = self.iterator.collected.clone();
        } else {
            if letter_char == '{' {
                self.brace_count += 1;
            } else if letter_char == '}' && self.brace_count != 0 {
                self.brace_count -= 1;
            }
            self.iterator.iterate(last_char, letter_char);
        }
    }
}
