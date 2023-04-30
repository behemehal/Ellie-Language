use crate::syntax::items::constructor;
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for constructor::Constructor {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let mut hang = false;
        if !self.parameter_collected {
            let parameter_len = self.parameters.len();

            if utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
                .reliable
            {
                if parameter_len == 0 {
                    self.parameters.push(constructor::ConstructorParameter {
                        name: letter_char.to_string(),
                        pos: defs::Cursor {
                            range_start: cursor,
                            ..Default::default()
                        },
                    })
                } else {
                    if self.parameters[parameter_len - 1].name == "" {
                        self.comma = false;
                        self.parameters[parameter_len - 1].pos.range_start = cursor;
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
                    self.parameters[parameter_len - 1].pos.range_end = cursor;
                    self.parameters[parameter_len - 1].name += &letter_char.to_string();
                }
            } else if letter_char == ',' && parameter_len != 0 && !self.comma {
                self.comma = true;
                self.parameters
                    .push(constructor::ConstructorParameter::default());
            } else if letter_char == ')'
                && (parameter_len == 0 || (self.parameters[parameter_len - 1].name != ""))
            {
                self.parameter_collected = true;
            }
        } else if !self.continuum_collected {
            if letter_char == '{' {
                self.continuum_collected = true;
            } else if letter_char == ';' {
                self.pos.range_end = cursor;
                self.continuum_collected = true;
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
        } else if letter_char == '}' && self.brace_count == 0 {
            self.complete = true;
            self.pos.range_end = cursor;
            self.iterator.finalize();
            errors.extend(self.iterator.errors.clone());
            self.inside_code = self.iterator.collected.clone();
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
