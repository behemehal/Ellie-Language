use ellie_tokenizer::syntax::items::getter_call::GetterCall;

impl super::Processor for GetterCall {
    fn process(
        &self,
        parser: &mut super::Parser,
        _page_idx: usize,
        processed_page_idx: usize,
        page_hash: u64,
    ) -> bool {
        match super::type_processor::process(
            self.data.clone(),
            parser,
            page_hash,
            None,
            false,
            false,
        ) {
            Ok(data) => {
                let page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();
                page.items
                    .push(ellie_core::definite::items::Collecting::GetterCall(
                        ellie_core::definite::items::getter_call::GetterCall {
                            data,
                            pos: self.pos,
                        },
                    ));
                true
            }
            Err(e) => {
                parser.informations.extend(&e);
                true
            }
        }
    }
}
