use crate::processors::types::Processor;
use crate::syntax::items::variable::VariableCollector;
use ellie_core::{defs, error, utils};

impl super::Processor for VariableCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.name_collected {
            if utils::reliable_name_range(utils::ReliableNameRanges::Type, letter_char).reliable {
                if self.data.name == "" {
                    self.data.name_pos.range_start = cursor;
                } else if last_char == ' ' {
                    errors.push(error::errorList::error_s1.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                        "var_0x23".to_owned(),
                        defs::Cursor::build_with_skip_char(cursor),
                    ));
                }
                self.data.name_pos.range_end = cursor;
                self.data.name += &letter_char.to_string();
            } else if letter_char == ':' {
                self.name_collected = true;
            } else if letter_char == '=' {
                self.name_collected = true;
                self.type_collected = true;
            } else if letter_char != ' ' {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    "var_0x40".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else if !self.type_collected {
            if self.type_cache.complete && letter_char == '=' {
                self.type_collected = true;
                self.data.rtype = self.type_cache.clone();
            } else {
                self.type_cache
                    .iterate(errors, cursor, last_char, letter_char)
            }
        } else if !self.value_collected {
            if self.value_cache.is_complete() && letter_char == ';' {
                self.complete = true;
                self.data.value = self.value_cache.current.clone();
            } else {
                self.value_cache
                    .iterate(errors, cursor, last_char, letter_char)
            }
        }
        self.data.pos.range_end = cursor;
    }
}
