use crate::{processors::Processor, syntax::types::reference_type};
use ellie_core::{defs, error, utils};

impl Processor for reference_type::ReferenceTypeCollector {
    fn new() -> Self {
        reference_type::ReferenceTypeCollector::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<ellie_core::error::Error>,
        cursor: ellie_core::defs::CursorPosition,
        _last_char: char,
        letter_char: char,
    ) {
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
            self.on_dot = false;
            self.complete = true;
            self.data.chain[chain_len - 1].pos.range_end = cursor;
            self.data.chain[chain_len - 1].value += &letter_char.to_string();
        } else {
            errors.push(error::errorList::error_s1.clone().build(
                vec![error::ErrorBuildField {
                    key: "token".to_string(),
                    value: letter_char.to_string(),
                }],
                "ref_0x40".to_owned(),
                defs::Cursor::build_with_skip_char(cursor),
            ));
        }
    }
}
