use ellie_tokenizer::syntax::items::go::Go;

impl super::Processor for Go {
    fn process(
        &self,
        parser: &mut super::Parser,
        _page_idx: usize,
        processed_page_idx: usize,
        _page_hash: u64,
    ) -> bool {
        let page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();
        page.items.push(ellie_core::definite::items::Collecting::Go(
            ellie_core::definite::items::go::Go { pos: self.pos },
        ));
        true
    }
}
