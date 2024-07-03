use alloc::{borrow::ToOwned, vec};
use ellie_core::{definite::Converter, error};
use ellie_tokenizer::syntax::items::file_key::FileKey;

impl super::ItemParserProcessor for FileKey {
    fn process(&self, options: &mut super::ItemParserProcessorOptions) -> bool {
        if self.value.is_static() {
            if self.is_global {
                options
                    .parser
                    .processed_pages
                    .nth_mut(options.processed_page_idx)
                    .unwrap()
                    .global_file_keys
                    .push(self.clone().to_definite());
            } else {
                options
                    .parser
                    .processed_pages
                    .nth_mut(options.processed_page_idx)
                    .unwrap()
                    .unassigned_file_keys
                    .push(self.clone().to_definite());
            }
        } else {
            let path = options
                .parser
                .pages
                .nth(options.page_idx)
                .unwrap()
                .path
                .clone();
            options
                .parser
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
