use alloc::{borrow::ToOwned, string::ToString, vec};
use ellie_core::{
    definite::{definers, definers::DefinerCollecting},
    error,
};
use ellie_tokenizer::syntax::items::definers::CollectiveType;

impl super::DefinerParserProcessor for CollectiveType {
    fn process(
        &self,
        options: &mut super::DefinerParserProcessorOptions,
    ) -> Result<DefinerCollecting, alloc::vec::Vec<ellie_core::error::Error>> {
        let mut errors = vec![];
        let mut found = DefinerCollecting::Dynamic;

        let mut key = DefinerCollecting::Dynamic;
        let mut value = DefinerCollecting::Dynamic;
        match self.key.process(options) {
            Ok(k) => {
                key = k;
            }
            Err(err) => errors.extend(err),
        }

        match self.value.process(options) {
            Ok(v) => {
                value = v;
            }
            Err(err) => errors.extend(err),
        }

        let deep_search_result = options.parser.deep_search(
            options.page_id,
            "collective".to_string(),
            options.ignore_hash,
            vec![],
            0,
            None,
        );

        if deep_search_result.found && errors.is_empty() {
            match deep_search_result.found_item {
                crate::parser::DeepSearchItems::Class(collective_class) => {
                    found = DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                        parent_pos: collective_class.pos,
                        generics: vec![
                            definers::GenericParameter {
                                value: key,
                                pos: deep_search_result.found_pos.unwrap_or_default(),
                            },
                            definers::GenericParameter {
                                value,
                                pos: deep_search_result.found_pos.unwrap_or_default(),
                            },
                        ],
                        hash: collective_class.hash,
                        rtype: "collective".to_string(),
                    });
                }
                _ => match deep_search_result.found_pos {
                    Some(ref_pos) => {
                        let mut error = error::error_list::ERROR_S45.clone().build_with_path(
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
                                    value: "collective".to_string(),
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
                        value: "collective".to_string(),
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
