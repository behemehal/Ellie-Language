use ellie_core::defs;
use ellie_tokenizer::syntax::items::ret::Ret;

impl super::Processor for Ret {
    fn process(self, parser: &mut super::Parser, page_id: u64) {
        match super::type_processor::process(self.value.current, parser, page_id, None) {
            Ok(value) => {
                let unprocessed_page = parser.find_page(page_id).unwrap();
                unprocessed_page.unreachable = true;
                unprocessed_page.unreachable_range.range_start =
                defs::CursorPosition(self.pos.range_end.0 + 1, 0);
                let page = parser.find_processed_page(page_id).unwrap();
                page.items
                    .push(ellie_core::definite::items::Collecting::Ret(
                        ellie_core::definite::items::ret::Ret {
                            value: value,
                            keyword_pos: self.keyword_pos,
                            value_position: self.value_position,
                            pos: self.pos,
                        },
                    ));
            }
            Err(type_error) => parser.informations.extend(&type_error),
        }
    }
}
