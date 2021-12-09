use alloc::vec;
use ellie_core::definite::{types::Types, definers::DefinerCollecting};
use ellie_tokenizer::syntax::items::variable::VariableCollector;

impl super::Processor for VariableCollector {
    fn process(self, parser: &mut super::Parser, page_id: u64) {
        let resolved_type = if !self.data.has_value {
            Ok(Types::Void)
        } else {
            super::type_processor::process(self.data.value, parser, page_id)
        };
        let resolved_defining = if !self.data.has_type {
            Ok(DefinerCollecting::Dynamic)
        } else {
            super::definer_processor::process(
                self.data.rtype.definer_type,
                parser,
                page_id,
                self.data.type_pos,
            )
        };
        if resolved_type.is_err() || resolved_defining.is_err() {
            let mut type_error = resolved_type.err().unwrap_or(vec![]);
            let defining_error = resolved_defining.err().unwrap_or(vec![]);
            type_error.extend(defining_error);
            parser.errors.extend(type_error);
        } else {
            let processed = ellie_core::definite::items::Collecting::Variable(
                ellie_core::definite::items::variable::Variable {
                    name: self.data.name,
                    constant: self.data.constant,
                    public: self.data.public,
                    value: resolved_type.unwrap(),
                    pos: self.data.pos,
                    name_pos: self.data.name_pos,
                    value_pos: self.data.value_pos,
                    type_pos: self.data.type_pos,
                    rtype: resolved_defining.unwrap(),
                    hash: self.data.hash,
                    has_type: self.data.has_type,
                    has_value: self.data.has_value,
                },
            );
            parser
                .find_processed_page(page_id)
                .unwrap()
                .items
                .push(processed)
        }
    }
}
