use ellie_tokenizer::syntax::items::brk::Brk;

impl super::Processor for Brk {
    fn process(self, parser: &mut super::Parser, page_id: u64) -> bool {
        let page = parser.find_processed_page(page_id).unwrap();
        page.items
            .push(ellie_core::definite::items::Collecting::Brk(
                ellie_core::definite::items::brk::Brk { pos: self.pos },
            ));
        true
    }
}
