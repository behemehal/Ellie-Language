use alloc::{borrow::ToOwned, vec};
use ellie_core::error;
use ellie_tokenizer::syntax::items::class::Class;

impl super::Processor for Class {
    fn process(self, parser: &mut super::Parser, page_id: u64) {
        let duplicate = parser.deep_search(page_id, self.name.clone(), Some(self.hash.clone()), vec![], 0);
        if duplicate.is_some() {
            parser
                .errors
                .push(error::errorList::error_s24.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.name,
                    }],
                    "pcls_0x14".to_owned(),
                    self.name_pos,
                ));
        } else {
            ()
            //("{:#?}", self)
        }
    }
}
