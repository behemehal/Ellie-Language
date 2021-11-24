use crate::processors::Processor;
use crate::syntax::types::function_call_type;
use ellie_core::{defs, error};

use super::TypeProcessor;

impl Processor for function_call_type::FunctionCallCollector {
    fn new() -> Self {
        function_call_type::FunctionCallCollector::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        if !self.param_started {
            if letter_char == '(' {
                self.param_started = true;
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
        } else {
            if self.itered_cache.is_complete() && letter_char == ',' {
                self.data
                    .parameters
                    .push(function_call_type::FunctionCallParameter::default());
                self.itered_cache = Box::new(TypeProcessor::default())
            } else if (self.itered_cache.is_complete() || self.data.parameters.is_empty())
                && letter_char == ')'
            {
                self.complete = true;
            } else {
                self.itered_cache
                    .iterate(errors, cursor, last_char, letter_char);

                let param_len = self.data.parameters.len();
                if param_len == 0 {
                    self.data
                        .parameters
                        .push(function_call_type::FunctionCallParameter {
                            value: self.itered_cache.current.clone(),
                            pos: defs::Cursor::build_with_skip_char(cursor),
                        });
                } else {
                    self.data.parameters[param_len - 1].value = self.itered_cache.current.clone();
                    self.data.parameters[param_len - 1].pos =
                        defs::Cursor::build_with_skip_char(cursor);
                }
            }
        }
    }
}
