use crate::{processors::EscapeCharEmitter, syntax::items::class};
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for class::Class {
    fn emits_line_endings(&self) -> EscapeCharEmitter {
        self.iterator.emits_line_endings()
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<ellie_core::error::Error>,
        cursor: ellie_core::defs::CursorPosition,
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
            } else if letter_char == '{' {
                self.name_collected = true;
                self.generics_collected = true;
                self.continuum_collected = true;
            } else if letter_char == '<' {
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
        } else if !self.generics_collected {
            let generic_len = self.generic_definings.len();
            if utils::reliable_name_range(utils::ReliableNameRanges::Type, letter_char).reliable {
                if generic_len == 0 {
                    self.generic_definings.push(class::GenericDefining {
                        pos: defs::Cursor {
                            range_start: cursor,
                            ..Default::default()
                        },
                        name: letter_char.to_string(),
                        hash: utils::generate_hash_usize(),
                    });
                } else {
                    if self.generic_definings[generic_len - 1].name == "" {
                        self.generic_definings[generic_len - 1].pos.range_start = cursor;
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
                    self.generic_definings[generic_len - 1].pos.range_end = cursor;
                    self.generic_definings[generic_len - 1].name += &letter_char.to_string();
                }
            } else if letter_char == ','
                && generic_len > 0
                && self.generic_definings[generic_len - 1].name != ""
            {
                self.generic_definings.last_mut().unwrap().pos.range_end =
                    cursor.clone().pop_char(1);
                self.generic_definings
                    .push(class::GenericDefining::default());
            } else if letter_char == '>'
                && generic_len > 0
                && self.generic_definings[generic_len - 1].name != ""
            {
                self.generic_definings.last_mut().unwrap().pos.range_end =
                    cursor.clone().pop_char(1);
                self.generics_collected = true;
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
        } else if !self.continuum_collected {
            if letter_char == '{' {
                self.continuum_collected = true;
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
        } else if letter_char == '}' && self.brace_count == 0 {
            self.hash = ellie_core::utils::generate_hash_usize();
            self.pos.range_end = cursor;
            self.complete = true;
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
