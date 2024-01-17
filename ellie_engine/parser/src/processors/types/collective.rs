use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use alloc::{string::ToString, vec};
use ellie_core::{definite::types, error};
use ellie_tokenizer::syntax::types::collective_type;

impl super::TypeParserProcessor for collective_type::CollectiveTypeCollector {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        Err(vec![error::error_list::ERROR_S59.clone().build_with_path(
            vec![error::ErrorBuildField {
                key: "token".to_string(),
                value: "collective".to_string(),
            }],
            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
            options
                .parser
                .find_page(options.page_id)
                .unwrap()
                .path
                .clone(),
            self.data.pos,
        )])
    }
}
