use ellie_core::definite::items::{generic, Collecting};
use ellie_tokenizer::syntax::items::generic_item::GenericItem;

impl super::ItemParserProcessor for GenericItem {
    fn process(&self, options: &mut super::ItemParserProcessorOptions) -> bool {
        options
            .parser
            .processed_pages
            .nth_mut(options.processed_page_idx)
            .unwrap()
            .items
            .push(Collecting::Generic(generic::Generic {
                name: self.generic_name.clone(),
                pos: self.pos,
                hash: self.hash,
            }));
        true
    }
}
