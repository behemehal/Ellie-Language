use crate::{
    deep_search_extensions::resolve_type,
    extra::{item_search::item_search, utils::DeepSearchOptions},
};
use alloc::{borrow::ToOwned, string::ToString, vec};
use ellie_core::{
    definite::{definers, definers::DefinerCollecting, items::Collecting},
    error,
};
use ellie_tokenizer::{processors::items::Processors, syntax::items::definers::ArrayType};

impl super::DefinerParserProcessor for ArrayType {
    fn process(
        &self,
        options: &mut super::DefinerParserProcessorOptions,
    ) -> Result<DefinerCollecting, alloc::vec::Vec<ellie_core::error::Error>> {
        let mut errors = vec![];
        let mut found = DefinerCollecting::Dynamic;

        let generic_search = item_search(
            &mut DeepSearchOptions::new()
                .parser(options.parser)
                .page_id(options.page_id)
                .search_on_all()
                .name("array".to_string())
                .build(),
        );

        if generic_search.found() {
            if let Some(processed_items) = generic_search.found_item.processed() {
                match processed_items {
                    Collecting::Class(array_class) => match self.rtype.process(options) {
                        Ok(inner_type) => match self.size.clone() {
                            Some(array_size) => {
                                let resolved_deep_size = match resolve_type(
                                    *array_size,
                                    options.page_id,
                                    options.parser,
                                    &mut errors,
                                    Some(self.size_pos),
                                ) {
                                    Some(e) => e,
                                    None => return Err(errors),
                                };

                                if matches!(resolved_deep_size.clone(), DefinerCollecting::Generic(x) if x.rtype == "int")
                                {
                                    found = DefinerCollecting::ParentGeneric(
                                        definers::ParentGenericType {
                                            parent_pos: array_class.pos,
                                            generics: vec![definers::GenericParameter {
                                                value: inner_type,
                                                pos: array_class.pos,
                                            }],
                                            hash: array_class.hash,
                                            rtype: "array".to_string(),
                                        },
                                    );
                                } else {
                                    errors.push(
                                        error::error_list::ERROR_S3.clone().build_with_path(
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "token1".to_string(),
                                                    value: "int".to_string(),
                                                },
                                                error::ErrorBuildField {
                                                    key: "token2".to_string(),
                                                    value: resolved_deep_size.to_string(),
                                                },
                                            ],
                                            alloc::format!(
                                                "{}:{}:{}",
                                                file!().to_owned(),
                                                line!(),
                                                column!()
                                            ),
                                            options
                                                .parser
                                                .find_page(options.page_id)
                                                .unwrap()
                                                .path
                                                .clone(),
                                            self.size_pos,
                                        ),
                                    );
                                    return Err(errors);
                                }
                            }
                            None => {
                                found =
                                    DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                                        parent_pos: array_class.pos,
                                        generics: vec![definers::GenericParameter {
                                            value: inner_type,
                                            pos: array_class.pos,
                                        }],
                                        hash: array_class.hash,
                                        rtype: "array".to_string(),
                                    });
                            }
                        },
                        Err(e) => errors.extend(e),
                    },
                    _ => {
                        errors.push(
                            error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: "array".to_string(),
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
                }
            } else if let Some(raw_items) = generic_search.found_item.raw() {
                match raw_items {
                    Processors::Class(array_class) => match self.rtype.process(options) {
                        Ok(inner_type) => match self.size.clone() {
                            Some(array_size) => {
                                let resolved_deep_size = match resolve_type(
                                    *array_size,
                                    options.page_id,
                                    options.parser,
                                    &mut errors,
                                    Some(self.size_pos),
                                ) {
                                    Some(e) => e,
                                    None => return Err(errors),
                                };

                                if matches!(resolved_deep_size.clone(), DefinerCollecting::Generic(x) if x.rtype == "int")
                                {
                                    found = DefinerCollecting::ParentGeneric(
                                        definers::ParentGenericType {
                                            parent_pos: array_class.pos,
                                            generics: vec![definers::GenericParameter {
                                                value: inner_type,
                                                pos: array_class.pos,
                                            }],
                                            hash: array_class.hash,
                                            rtype: "array".to_string(),
                                        },
                                    );
                                } else {
                                    errors.push(
                                        error::error_list::ERROR_S3.clone().build_with_path(
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "token1".to_string(),
                                                    value: "int".to_string(),
                                                },
                                                error::ErrorBuildField {
                                                    key: "token2".to_string(),
                                                    value: resolved_deep_size.to_string(),
                                                },
                                            ],
                                            alloc::format!(
                                                "{}:{}:{}",
                                                file!().to_owned(),
                                                line!(),
                                                column!()
                                            ),
                                            options
                                                .parser
                                                .find_page(options.page_id)
                                                .unwrap()
                                                .path
                                                .clone(),
                                            self.size_pos,
                                        ),
                                    );
                                    return Err(errors);
                                }
                            }
                            None => {
                                found =
                                    DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                                        parent_pos: array_class.pos,
                                        generics: vec![definers::GenericParameter {
                                            value: inner_type,
                                            pos: array_class.pos,
                                        }],
                                        hash: array_class.hash,
                                        rtype: "array".to_string(),
                                    });
                            }
                        },
                        Err(e) => errors.extend(e),
                    },
                    _ => {
                        errors.push(
                            error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: "array".to_string(),
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
                }
            } else {
                unreachable!("Something is wrong with item_search, search returned positive but no items were found");
            }
        } else {
            errors.push(
                error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: "array".to_string(),
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
