use ellie_core::defs;
use ellie_tokenizer::syntax::items::ret::Ret;

use crate::processors::types::{TypeParserProcessor, TypeParserProcessorOptions};

impl super::ItemParserProcessor for Ret {
    fn process(&self, options: &mut super::ItemParserProcessorOptions) -> bool {
        match self.value.current.process(
            TypeParserProcessorOptions::new(options.parser, options.page_hash)
                .variable_pos(self.pos)
                .build(),
        ) {
            Ok(value) => {
                let unprocessed_page = options.parser.pages.nth_mut(options.page_idx).unwrap();
                unprocessed_page.unreachable = true;
                unprocessed_page.unreachable_range.range_start =
                    defs::CursorPosition(self.pos.range_end.0 + 1, 0);
                let page = options
                    .parser
                    .processed_pages
                    .nth_mut(options.processed_page_idx)
                    .unwrap();
                page.items
                    .push(ellie_core::definite::items::Collecting::Ret(
                        ellie_core::definite::items::ret::Ret {
                            value,
                            keyword_pos: self.keyword_pos,
                            value_position: self.value_position,
                            pos: self.pos,
                        },
                    ));
                true
            }
            Err(type_error) => {
                options.parser.informations.extend(&type_error);
                false
            }
        }
    }
}
