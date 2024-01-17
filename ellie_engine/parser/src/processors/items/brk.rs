use ellie_core::definite::items::{brk, Collecting};
use ellie_tokenizer::syntax::items::brk::Brk;

impl super::ItemParserProcessor for Brk {
    fn process(&self, options: &mut super::ItemParserProcessorOptions) -> bool {
        options
            .parser
            .processed_pages
            .nth_mut(options.processed_page_idx)
            .unwrap()
            .items
            .push(Collecting::Brk(brk::Brk { pos: self.pos }));
        true
    }
}
