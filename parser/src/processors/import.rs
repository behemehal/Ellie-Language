use alloc::{borrow::ToOwned, vec};
use ellie_core::{definite::Converter, error};
use ellie_tokenizer::syntax::items::import::Import;

impl super::Processor for Import {
    fn process(self, parser: &mut super::Parser, page_id: u64) {
        let duplicate = if self.reference == "" {
            false
        } else {
            parser
                .deep_search(
                    page_id,
                    self.reference.clone(),
                    Some(self.hash.clone()),
                    vec![],
                    0,
                )
                .found
        };

        if duplicate {
            parser
                .informations
                .push(&error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.reference,
                    }],
                    "imp_0x14".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    self.reference_pos,
                ));
        } else {
            parser.find_processed_page(page_id).unwrap().items.push(
                ellie_core::definite::items::Collecting::Import(self.to_definite()),
            )
            /*
            match self.hash.parse::<u64>() {
                Ok(hash) => {
                    parser.process_page(hash);

                }
                Err(_) => {
                    panic!("Import's hash is not valid");
                }
            }
            */
        }
    }
}
