use ellie_tokenizer::syntax::items::go::Go;

impl super::Processor for Go {
    fn process(self, parser: &mut super::Parser, page_id: u64) -> bool {
        let page = parser.find_processed_page(page_id).unwrap();
        page.items.push(ellie_core::definite::items::Collecting::Go(
            ellie_core::definite::items::go::Go { pos: self.pos },
        ));
        true
    }
}
