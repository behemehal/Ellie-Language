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
        if !self.path_filled {
            if letter_char == ':' && self.path != "" {
                self.path_filled = true;
            } else if letter_char == ';' && self.path != "" {
                self.pos.range_end = cursor.clone().skip_char(1);
                self.complete = true;
            } else {
                if self.path == "" {
                    if letter_char == '@' {
                        self.link_module = true;
                    }
                    self.path_pos.range_start = cursor;
                }
                if self.link_module {
                    if utils::reliable_name_range(
                        utils::ReliableNameRanges::VariableName,
                        letter_char,
                    )
                    .reliable
                    {
                        if last_char == ' ' {
                            errors.push(error::error_list::ERROR_S1.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char.to_string(),
                                }],
                                file!().to_owned(),
                                defs::Cursor::build_with_skip_char(cursor),
                            ));
                        }
                        self.path_pos.range_end = cursor;
                        self.path += &letter_char.to_string();
                    }
                } else {
                    if last_char == ' ' && self.path != "" {
                        errors.push(error::error_list::ERROR_S1.clone().build(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                            file!().to_owned(),
                            defs::Cursor::build_with_skip_char(cursor),
                        ));
                    } else if self.path != "" || (self.path == "" && letter_char != ' ') {
                        self.path_pos.range_end = cursor;
                        self.path += &letter_char.to_string();
                    }
                }
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
                        file!().to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                }
                self.reference_pos.range_end = cursor;
                self.reference += &letter_char.to_string();
            } else if letter_char == ';' && self.path != "" {
                self.pos.range_end = cursor.clone().skip_char(1);
                self.complete = true;
            }
        }
        false
    }
}
