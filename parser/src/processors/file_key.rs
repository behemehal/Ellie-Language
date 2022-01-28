use alloc::{borrow::ToOwned, vec};
use ellie_core::{definite::Converter, error};
use ellie_tokenizer::syntax::items::file_key::FileKey;

impl super::Processor for FileKey {
    fn process(self, parser: &mut super::Parser, page_id: u64) {
        if self.value.is_static() {
            parser.find_processed_page(page_id).unwrap().items.push(
                ellie_core::definite::items::Collecting::FileKey(self.to_definite()),
            );
        } else {
            let path = parser.find_page(page_id).unwrap().path.clone();
            parser
                .informations
                .push(&error::error_list::ERROR_S9.clone().build_with_path(
                    vec![],
                    file!().to_owned(),
                    path,
                    self.value_location,
                ));
        }
    }
}
