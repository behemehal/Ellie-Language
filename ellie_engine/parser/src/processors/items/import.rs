use alloc::{borrow::ToOwned, vec};
use ellie_core::{definite::Converter, error, utils};
use ellie_tokenizer::syntax::items::import::Import;

impl super::ItemParserProcessor for Import {
    fn process(&self, options: &mut super::ItemParserProcessorOptions) -> bool {
        let path = options
            .parser
            .pages
            .nth(options.page_idx)
            .unwrap()
            .path
            .clone();

        if !self.reference.is_empty() {
            options.parser.informations.push(
                &error::error_list::ERROR_S59.clone().build_with_path(
                    vec![error::ErrorBuildField::new(
                        "token",
                        &"referenced imports".to_owned(),
                    )],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    path,
                    self.reference_pos,
                ),
            );

            return false;
        }

        let duplicate = if self.reference.is_empty() {
            false
        } else {
            options
                .parser
                .deep_search(
                    options.page_hash,
                    self.reference.clone(),
                    Some(self.hash),
                    vec![],
                    0,
                    None,
                )
                .found
        };

        if !self.reference.is_empty() && utils::is_reserved(&self.reference, false) {
            options.parser.informations.push(
                &error::error_list::ERROR_S21.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.reference.clone(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    path.clone(),
                    self.reference_pos,
                ),
            );
        }

        if duplicate {
            options.parser.informations.push(
                &error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField::new("token", &self.reference)],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    path.clone(),
                    self.reference_pos,
                ),
            );
        } else {
            options
                .parser
                .processed_pages
                .nth_mut(options.processed_page_idx)
                .unwrap()
                .items
                .push(ellie_core::definite::items::Collecting::Import(
                    self.clone().to_definite(),
                ));
        }
        true
    }
}
