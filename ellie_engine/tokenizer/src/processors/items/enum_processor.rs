use crate::syntax::items::enum_type::{EnumItem, EnumType, GenericDefining};
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for EnumType {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let hang = false;
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
            } else if letter_char == '<' {
                self.name_collected = true;
            } else if letter_char == '{' {
                self.body_pos.range_start = cursor;
                self.name_collected = true;
                self.continuum_collected = true;
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
        } else if !self.generics_collected {
            let generic_len = self.generic_definings.len();
            if utils::reliable_name_range(utils::ReliableNameRanges::Type, letter_char).reliable {
                if generic_len == 0 {
                    self.generic_definings.push(GenericDefining {
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
                self.generic_definings.push(GenericDefining::default());
            } else if letter_char == '>'
                && generic_len > 0
                && self.generic_definings[generic_len - 1].name != ""
            {
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
        } else {
            let len = if self.items.is_empty() {
                0
            } else {
                self.items.len() - 1
            };
            let current_item = if self.items.is_empty() {
                self.items.push(EnumItem::default());
                &mut self.items[0]
            } else {
                &mut self.items[len]
            };

            if letter_char == '}' && current_item.identifier == "" {
                self.items.pop();
                self.complete = true;
            } else if current_item.type_complete {
                if letter_char == ',' {
                    self.items.push(EnumItem::default());
                } else if letter_char == '}' {
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
            } else if current_item.identifier_collected {
                if current_item.enum_type.complete && letter_char == ')' {
                    current_item.type_complete = true;
                } else {
                    current_item
                        .enum_type
                        .iterate(errors, cursor, last_char, letter_char);
                    current_item.type_pos.range_end = cursor;
                }
            } else {
                if utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
                    .reliable
                {
                    if current_item.identifier == "" {
                        current_item.identifier_pos.range_start = cursor;
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
                    current_item.identifier_pos.range_end = cursor;
                    current_item.identifier += &letter_char.to_string();
                } else if letter_char == '(' && !current_item.identifier.is_empty() {
                    current_item.identifier_collected = true;
                    current_item.has_type = true;

                    current_item.type_pos.range_start = cursor;
                } else if letter_char == '}' && !current_item.identifier.is_empty() {
                    current_item.identifier_collected = false;
                    self.complete = true;
                } else if letter_char == ',' && !current_item.identifier.is_empty() {
                    current_item.identifier_collected = false;
                    self.items.push(EnumItem::default());
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
        }
        hang
    }
}
