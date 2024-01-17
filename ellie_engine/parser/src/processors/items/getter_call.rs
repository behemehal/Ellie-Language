use ellie_tokenizer::syntax::items::getter_call::GetterCall;

use crate::processors::types::{TypeParserProcessor, TypeParserProcessorOptions};

impl super::ItemParserProcessor for GetterCall {
    fn process(&self, options: &mut super::ItemParserProcessorOptions) -> bool {
        match self.data.process(
            &mut TypeParserProcessorOptions::new(options.parser, options.page_hash)
                .variable_pos(self.pos)
                .build(),
        ) {
            Ok(data) => {
                let page = options
                    .parser
                    .processed_pages
                    .nth_mut(options.processed_page_idx)
                    .unwrap();
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
                options.parser.informations.extend(&e);
                true
            }
        }
    }
}
