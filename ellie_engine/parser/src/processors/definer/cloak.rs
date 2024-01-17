use alloc::borrow::ToOwned;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{definite::definers, definite::definers::DefinerCollecting, error};
use ellie_tokenizer::syntax::items::definers::CloakType;

impl super::DefinerParserProcessor for CloakType {
    fn process(
        &self,
        options: &mut super::DefinerParserProcessorOptions,
    ) -> Result<DefinerCollecting, Vec<ellie_core::error::Error>> {
        let mut errors = vec![];
        let mut found = DefinerCollecting::Dynamic;

        let deep_search_result = options.parser.deep_search(
            options.page_id,
            "cloak".to_string(),
            options.ignore_hash,
            vec![],
            0,
            None,
        );

        if deep_search_result.found {
            match deep_search_result.found_item {
                crate::parser::DeepSearchItems::Class(cloak_class) => {
                    found = DefinerCollecting::Generic(definers::GenericType {
                        hash: cloak_class.hash,
                        rtype: "cloak".to_string(),
                        pos: cloak_class.pos,
                    });
                }
                _ => match deep_search_result.found_pos {
                    Some(ref_pos) => {
                        let mut error = error::error_list::ERROR_S45.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: "cloak".to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            options
                                .parser
                                .find_page(options.page_id)
                                .unwrap()
                                .path
                                .clone(),
                            self.pos,
                        );
                        error.reference_message = "Defined here".to_string();
                        error.reference_block = Some((ref_pos, deep_search_result.found_page.path));
                        errors.push(error);
                    }
                    None => {
                        errors.push(
                            error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "cloak".to_string(),
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                options
                                    .parser
                                    .find_page(options.page_id)
                                    .unwrap()
                                    .path
                                    .clone(),
                                self.pos,
                            ),
                        );
                    }
                },
            }
        } else {
            errors.push(
                error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "cloak".to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    options
                        .parser
                        .find_page(options.page_id)
                        .unwrap()
                        .path
                        .clone(),
                    self.pos,
                ),
            );
        }

        if errors.is_empty() {
            Ok(found)
        } else {
            Err(errors)
        }
    }
}
