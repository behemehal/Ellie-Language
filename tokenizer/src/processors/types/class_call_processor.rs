use crate::syntax::{items::definers::DefinerCollector, types::class_call_type};
use ellie_core::{defs, error};

use super::TypeProcessor;

impl super::Processor for class_call_type::ClassCallCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.base_collected {
            if (letter_char == '(' || letter_char == '<') && self.itered_cache.is_complete() {
                self.base_collected = true;
                self.data.target = Box::new(self.itered_cache.current.clone());
                if letter_char == '(' {
                    self.param_collected = true;
                    self.generic_collected = true;
                } else {
                    self.data
                        .generic_parameters
                        .push(class_call_type::ClassCallGenericParameter {
                            pos: defs::Cursor::build_with_skip_char(cursor),
                            ..Default::default()
                        });
                }
                self.itered_cache = Box::new(TypeProcessor::default());
            } else {
                self.itered_cache
                    .iterate(errors, cursor, last_char, letter_char);
            }
        } else if !self.generic_collected {
            if (letter_char == ',' || letter_char == '>') && self.generic_cache.complete {
                let len = self.data.generic_parameters.len();
                self.data.generic_parameters[len - 1].pos.range_end = cursor;
                self.data.generic_parameters[len - 1].value =
                    self.generic_cache.definer_type.clone();
                if letter_char == '>' {
                    self.generic_collected = true;
                    self.generic_cache = DefinerCollector::default();
                } else {
                    self.data.generic_parameters[len - 1].pos.range_start = cursor;
                    self.data
                        .generic_parameters
                        .push(class_call_type::ClassCallGenericParameter::default());
                    self.generic_cache = DefinerCollector::default();
                }
            } else {
                self.generic_cache
                    .iterate(errors, cursor, last_char, letter_char);
            }
        } else if !self.param_collected {
            if letter_char == '(' {
                self.param_collected = true;
            } else if letter_char != ' ' {
                errors.push(error::errorList::error_s1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "val".to_owned(),
                        value: letter_char.to_string(),
                    }],
                    "brace_refence_0x36".to_owned(),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            }
        } else if !self.complete {
            if self.itered_cache.is_complete() && letter_char == ',' {
                self.data
                    .parameters
                    .push(class_call_type::ClassCallParameter::default());
                self.itered_cache = Box::new(super::TypeProcessor::default());
            } else if (self.itered_cache.is_complete() || self.data.parameters.is_empty())
                && letter_char == ')'
            {
                self.itered_cache = Box::new(super::TypeProcessor::default());
                self.complete = true;
            } else {
                self.itered_cache
                    .iterate(errors, cursor, last_char, letter_char);

                let param_len = self.data.parameters.len();
                if param_len == 0 {
                    self.data
                        .parameters
                        .push(class_call_type::ClassCallParameter {
                            value: self.itered_cache.current.clone(),
                            pos: defs::Cursor::build_with_skip_char(cursor),
                        });
                } else {
                    self.data.parameters[param_len - 1].value = self.itered_cache.current.clone();
                    self.data.parameters[param_len - 1].pos.range_end = cursor.clone().skip_char(1);
                }
            }
        } else if letter_char != ' ' {
            errors.push(error::errorList::error_s1.clone().build(
                vec![error::ErrorBuildField {
                    key: "val".to_owned(),
                    value: letter_char.to_string(),
                }],
                "class_call_0x101".to_owned(),
                defs::Cursor::build_with_skip_char(cursor),
            ));
        }
    }
}
