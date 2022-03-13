use crate::syntax::types::reference_type;
use ellie_core::{defs, error, utils};

impl crate::processors::Processor for reference_type::ReferenceTypeCollector {
    fn iterate(
        &mut self,
        errors: &mut Vec<ellie_core::error::Error>,
        cursor: ellie_core::defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool {
        let chain_len = self.data.chain.clone().len();
        if letter_char == '.' && !self.on_dot {
            self.complete = false;
            self.data.chain.push(reference_type::Chain::default());
            let chain_len = self.data.chain.clone().len();
            self.data.chain[chain_len - 1].pos.range_start = cursor;
            self.on_dot = true;
        } else if utils::reliable_name_range(utils::ReliableNameRanges::VariableName, letter_char)
            .reliable
        {
            if last_char == ' ' && self.data.chain[chain_len - 1].value != "" {
                errors.push(error::error_list::ERROR_S1.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: letter_char.to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    defs::Cursor::build_with_skip_char(cursor),
                ));
            } else {
                self.data.chain[chain_len - 1].pos.range_end = cursor;
                self.on_dot = false;
                self.complete = true;
                self.data.chain[chain_len - 1].value += &letter_char.to_string();
            }
        } else if letter_char != ' ' {
            errors.push(error::error_list::ERROR_S1.clone().build(
                vec![error::ErrorBuildField {
                    key: "token".to_string(),
                    value: letter_char.to_string(),
                }],
                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                defs::Cursor::build_with_skip_char(cursor),
            ));
        }
        false
    }
}
