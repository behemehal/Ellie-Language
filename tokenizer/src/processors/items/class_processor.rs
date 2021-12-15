use crate::syntax::items::class;
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for class::Class {
    fn iterate(
        &mut self,
        errors: &mut Vec<ellie_core::error::Error>,
        cursor: ellie_core::defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
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
                        "class_0x24".to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                }
                self.name_pos.range_end = cursor.clone().skip_char(1);
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
                    "class_0x41".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
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
                            "class_0x66".to_owned(),
                            defs::Cursor::build_with_skip_char(cursor),
                        ));
                    }
                    self.generic_definings[generic_len - 1].pos.range_end =
                        cursor.clone().skip_char(1);
                    self.generic_definings[generic_len - 1].name += &letter_char.to_string();
                }
            } else if letter_char == ','
                && generic_len > 0
                && self.generic_definings[generic_len - 1].name != ""
            {
                self.generic_definings[generic_len - 1].pos.range_end = cursor;
                self.generic_definings
                    .push(class::GenericDefining::default());
            } else if letter_char == '>'
                && generic_len > 0
                && self.generic_definings[generic_len - 1].name != ""
            {
                self.generic_definings[generic_len - 1].pos.range_end = cursor;
                self.generics_collected = true;
            } else if letter_char != ' ' {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "class_0x89".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
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
                    "class_0x102".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else if letter_char == '}' && self.brace_count == 0 {
            self.hash = ellie_core::utils::generate_hash();
            self.pos.range_end = cursor.clone().skip_char(1);
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
            self.iterator.iterate(last_char, letter_char);
        }
    }
}
