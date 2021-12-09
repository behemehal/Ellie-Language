use alloc::{borrow::ToOwned, vec};
use ellie_core::{definite::Converter, error};
use ellie_tokenizer::syntax::items::import::Import;

impl super::Processor for Import {
    fn process(self, parser: &mut super::Parser, page_id: u64) {
        let duplicate = if self.reference == "" {
            None
        } else {
            parser.deep_search(page_id, self.reference.clone(), Some(self.hash.clone()), vec![], 0)
        };

        if duplicate.is_some() {
            parser
                .errors
                .push(error::errorList::error_s24.clone().build(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.reference,
                    }],
                    "pimp_0x14".to_owned(),
                    self.reference_pos,
                ));
        } else {
            match self.hash.parse::<u64>() {
                Ok(hash) => {
                    parser.process_page(hash);
                    parser.find_processed_page(page_id).unwrap().items.push(
                        ellie_core::definite::items::Collecting::Import(self.to_definite()),
                    )
                }
                Err(_) => {
                    panic!("Import's hash is not valid");
                }
            }
        }
    }
}
