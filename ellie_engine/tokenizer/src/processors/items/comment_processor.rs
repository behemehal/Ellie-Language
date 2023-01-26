use crate::syntax::items::comment::Comment;
use ellie_core::{defs, error};

impl crate::processors::Processor for Comment {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        if !self.type_collected {
            if self.first_char_collected {
                if letter_char == '/' {
                    self.line_comment = true;
                    self.type_collected = true;
                    self.content = Vec::new();
                } else if letter_char == '*' {
                    self.type_collected = true;
                    self.content = Vec::new();
                } else {
                    errors.push(error::error_list::ERROR_S1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        defs::Cursor::build_from_cursor(cursor),
                    ));
                }
            } else {
                self.first_char_collected = true;
                self.pos.range_start = cursor;
            }
        } else if self.content.len() != 0
            && matches!(self.content.last().unwrap().chars().last(), Some(last_letter) if last_letter == '*' && letter_char == '/')
        {
            self.complete = true;
            self.pos.range_end = cursor;
            let last_idx = self.content.len() - 1;
            let last = self.content[last_idx].clone();
            self.content[last_idx] = last[..last.len() - 1].to_string();
        } else {
            self.pos.range_end = cursor;
            if self.content.len() == 0 {
                self.content.push(String::from(letter_char));
            } else {
                self.content.last_mut().unwrap().push(letter_char);
            }
        }
        true
    }
}
