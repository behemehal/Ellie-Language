use crate::{syntax::items::getter, processors::EscapeCharEmitter};
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for getter::Getter {
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

        if !self.name_collected {
            if utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
                .reliable
            {
                if self.name == "" {
                    self.name_pos.range_start = cursor;
                } else if last_char == ' ' {
                    errors.push(error::error_list::ERROR_S1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        defs::Cursor::build_from_cursor(cursor),
                    ));
                }
                self.name_pos.range_end = cursor;
                self.name += &letter_char.to_string();
            } else if letter_char == ':' && self.name != "" {
                self.name_collected = true;
            } else if letter_char != ' ' {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_from_cursor(cursor),
                ));
            }
        } else if !self.return_collected {
            if self.return_type.complete && letter_char == '{' {
                self.return_collected = true;
                self.body_pos.range_start = cursor;
            } else {
                if self.return_pos.range_start.is_zero() && letter_char != ' ' {
                    self.return_pos.range_start = cursor;
                }
                if letter_char != ' ' {
                    self.return_pos.range_end = cursor;
                }
                hang = self
                    .return_type
                    .iterate(errors, cursor, last_char, letter_char);
            }
        } else if letter_char == '}' && self.brace_count == 0 {
            self.complete = true;
            self.body_pos.range_end = cursor;
            self.pos.range_end = cursor;
            self.hash = ellie_core::utils::generate_hash_usize();
            self.iterator.finalize();
            errors.extend(self.iterator.errors.clone());
            self.body = self.iterator.collected.clone();
            let contains_ret = self.body.iter().any(|x| match x {
                super::Processors::Ret(_) => true,
                _ => false,
            });

            if !contains_ret {
                let mut error = error::error_list::ERROR_S2.clone().build(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    self.body_pos,
                );

                error.reference_message = "Defined here".to_string();
                error.reference_block = Some((self.return_pos, "<fill>".to_string()));

                errors.push(error);
            }
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
