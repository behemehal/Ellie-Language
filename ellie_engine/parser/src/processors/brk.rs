use ellie_tokenizer::syntax::items::brk::Brk;

impl super::Processor for Brk {
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
            .push(ellie_core::definite::items::Collecting::Brk(
                ellie_core::definite::items::brk::Brk { pos: self.pos },
            ));
        true
    }
}
