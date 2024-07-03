use ellie_tokenizer::syntax::items::go::Go;

impl super::ItemParserProcessor for Go {
    fn process(&self, options: &mut super::ItemParserProcessorOptions) -> bool {
        let page = options
            .parser
            .processed_pages
            .nth_mut(options.processed_page_idx)
            .unwrap();
        page.items.push(ellie_core::definite::items::Collecting::Go(
            ellie_core::definite::items::go::Go { pos: self.pos },
        ));
        true
    }
}
