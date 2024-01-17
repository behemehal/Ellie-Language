use alloc::{borrow::ToOwned, string::ToString, vec, vec::Vec};
use ellie_core::{definite::definers, definite::definers::DefinerCollecting, error};
use ellie_tokenizer::syntax::items::definers::ParentGenericType;

impl super::DefinerParserProcessor for ParentGenericType {
    fn process(
        &self,
        options: &mut super::DefinerParserProcessorOptions,
    ) -> Result<DefinerCollecting, alloc::vec::Vec<ellie_core::error::Error>> {
        let mut errors = vec![];
        let mut found = DefinerCollecting::Dynamic;

        let deep_search_result = options.parser.deep_search(
            options.page_id,
            self.parent.clone(),
            options.ignore_hash,
            vec![],
            0,
            None,
        );

        if deep_search_result.found {
            match deep_search_result.found_item {
                crate::parser::DeepSearchItems::Class(e) => {
                    if e.generic_definings.len() != self.generics.len() {
                        let mut err = error::error_list::ERROR_S44.clone().build_with_path(
                            vec![
                                error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: e.generic_definings.len().to_string(),
                                },
                                error::ErrorBuildField {
                                    key: "token2".to_owned(),
                                    value: self.generics.len().to_string(),
                                },
                            ],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            options
                                .parser
                                .find_page(options.page_id)
                                .unwrap()
                                .path
                                .clone(),
                            self.pos,
                        );
                        err.reference_block =
                            Some((e.name_pos, deep_search_result.found_page.path));
                        errors.push(err);
                    } else {
                        let mut resolved_generics = Vec::new();
                        for i in &self.generics {
                            match i.value.process(options) {
                                Ok(e) => resolved_generics.push(definers::GenericParameter {
                                    value: e,
                                    pos: i.pos,
                                }),
                                Err(e) => errors.extend(e),
                            }
                        }
                        found = DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                            rtype: e.name,
                            hash: e.hash,
                            generics: resolved_generics,
                            parent_pos: self.parent_pos,
                        });
                    }
                }
                crate::parser::DeepSearchItems::BrokenPageGraph => todo!(),
                crate::parser::DeepSearchItems::MixUp(_) => todo!(),
                crate::parser::DeepSearchItems::None => {
                    errors.push(
                        error::error_list::ERROR_S6.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: self.parent.clone(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            options
                                .parser
                                .find_page(options.page_id)
                                .unwrap()
                                .path
                                .clone(),
                            self.parent_pos,
                        ),
                    );
                }
                _ => {
                    errors.push(
                        error::error_list::ERROR_S45.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: self.parent.clone(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            options
                                .parser
                                .find_page(options.page_id)
                                .unwrap()
                                .path
                                .clone(),
                            self.parent_pos,
                        ),
                    );
                }
            }
        } else {
            errors.push(
                error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.parent.clone(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    options
                        .parser
                        .find_page(options.page_id)
                        .unwrap()
                        .path
                        .clone(),
                    self.parent_pos,
                ),
            );
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(found)
        }
    }
}
