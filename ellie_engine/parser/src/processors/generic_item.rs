use ellie_tokenizer::syntax::items::generic_item::GenericItem;

impl super::Processor for GenericItem {
    fn process(
        &self,
        parser: &mut super::Parser,
        _page_idx: usize,
        processed_page_idx: usize,
        _page_hash: usize,
    ) -> bool {
        parser
            .processed_pages
            .nth_mut(processed_page_idx)
            .unwrap()
            .items
            .push(ellie_core::definite::items::Collecting::Generic(
                ellie_core::definite::items::generic::Generic {
                    name: self.generic_name.clone(),
                    pos: self.pos,
                    hash: self.hash,
                },
            ));
        true
    }
}
