use ellie_tokenizer::syntax::items::getter_call::GetterCall;

impl super::Processor for GetterCall {
    fn process(self, parser: &mut super::Parser, page_id: u64) -> bool {
        match super::type_processor::process(self.data, parser, page_id, None, false) {
            Ok(data) => {
                let page = parser.find_processed_page(page_id).unwrap();
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
