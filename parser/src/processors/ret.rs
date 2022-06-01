use ellie_core::defs;
use ellie_tokenizer::syntax::items::ret::Ret;

impl super::Processor for Ret {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        processed_page_idx: usize,
        page_hash: u64,
    ) -> bool {
        match super::type_processor::process(
            self.value.current.clone(),
            parser,
            page_hash,
            None,
            false,
            false,
        ) {
            Ok(value) => {
                let unprocessed_page = parser.pages.nth_mut(page_idx).unwrap();
                unprocessed_page.unreachable = true;
                unprocessed_page.unreachable_range.range_start =
                    defs::CursorPosition(self.pos.range_end.0 + 1, 0);
                let page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();
                page.items
                    .push(ellie_core::definite::items::Collecting::Ret(
                        ellie_core::definite::items::ret::Ret {
                            value: value,
                            keyword_pos: self.keyword_pos,
                            value_position: self.value_position,
                            pos: self.pos,
                        },
                    ));
                true
            }
            Err(type_error) => {
                parser.informations.extend(&type_error);
                false
            }
        }
    }
}
