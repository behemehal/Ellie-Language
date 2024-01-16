use crate::syntax::items::import::Import;
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for Import {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        if !self.path_starter_collected {
            if letter_char == '"' || letter_char == '@' {
                self.path_starter_collected = true;
                self.path_module = letter_char == '"';
                self.link_module = letter_char == '@';
                self.path_pos.range_start = cursor;
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
        } else if self.path_module && !self.import_filled {
            if letter_char == '"' {
                self.import_filled = true;
            } else {
                self.path_pos.range_end = cursor;
                self.path += &letter_char.to_string();
            }
        } else if self.link_module && !self.import_filled {
            if utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
                .reliable
            {
                self.path_pos.range_end = cursor;
                self.path += &letter_char.to_string();
            } else if (letter_char == ' ' || letter_char == ':' || letter_char == ';')
                && self.reference != ""
            {
                self.complete = letter_char == ';';
                self.reference_starter_collected = letter_char == ':';
                self.import_filled = true;
                self.pos.range_end = cursor;
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
        } else if !self.reference_starter_collected {
            if letter_char == ':' {
                self.reference_starter_collected = true;
            } else if letter_char == ';' {
                self.pos.range_end = cursor;
                self.complete = true;
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
        } else {
            if utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
                .reliable
            {
                if self.reference == "" {
                    self.reference_pos.range_start = cursor;
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
                self.reference_pos.range_end = cursor;
                self.reference += &letter_char.to_string();
            } else if letter_char == ';' && self.path != "" {
                self.pos.range_end = cursor;
                self.complete = true;
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
        }
        false
    }
}
