use alloc::{borrow::ToOwned, vec};
use ellie_core::{definite::Converter, error};
use ellie_tokenizer::syntax::items::file_key::FileKey;

impl super::Processor for FileKey {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        processed_page_idx: usize,
        _page_hash: usize,
    ) -> bool {
        if self.value.is_static() {
            if self.is_global {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .global_file_keys
                    .push(self.clone().to_definite());
            } else {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .unassigned_file_keys
                    .push(self.clone().to_definite());
            }
        } else {
            let path = parser.pages.nth(page_idx).unwrap().path.clone();
            parser
                .informations
                .push(&error::error_list::ERROR_S9.clone().build_with_path(
                    vec![],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    path,
                    self.value_location,
                ));
        }
        true
    }
}
