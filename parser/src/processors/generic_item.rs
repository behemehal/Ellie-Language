use ellie_tokenizer::syntax::items::generic_item::GenericItem;

impl super::Processor for GenericItem {
    fn process(self, parser: &mut crate::parser::Parser, page_id: u64) -> bool {
        parser.find_processed_page(page_id).unwrap().items.push(
            ellie_core::definite::items::Collecting::Generic(
                ellie_core::definite::items::generic::Generic {
                    name: self.generic_name,
                    pos: self.pos,
                    hash: self.hash,
                },
            ),
        );
        true
    }
}
