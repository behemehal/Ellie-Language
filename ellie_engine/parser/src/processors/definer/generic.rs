use alloc::{borrow::ToOwned, string::ToString, vec};
use ellie_core::{
    definite::{definers, definers::DefinerCollecting, items::Collecting},
    error,
};
use ellie_tokenizer::{processors::items::Processors, syntax::items::definers::GenericType};

use crate::extra::{item_search::item_search, utils::DeepSearchOptions};

impl super::DefinerParserProcessor for GenericType {
    fn process(
        &self,
        options: &mut super::DefinerParserProcessorOptions,
    ) -> Result<DefinerCollecting, alloc::vec::Vec<ellie_core::error::Error>> {
        let mut errors = vec![];
        let mut found = DefinerCollecting::Dynamic;

        let generic_search = item_search(
            &mut &mut &mut &mut DeepSearchOptions::new()
                .parser(options.parser)
                .page_id(options.page_id)
                .search_on_all()
                .name(self.rtype.clone())
                .build(),
        );

        if generic_search.found() {
            if let Some(processed_items) = generic_search.found_item.processed() {
                match processed_items {
                    Collecting::Class(class) => {
                        if class.generic_definings.is_empty() {
                            found = DefinerCollecting::Generic(definers::GenericType {
                                rtype: class.name.clone(),
                                hash: class.hash,
                                pos: class.pos,
                            });
                        } else {
                            errors.push(
                                error::error_list::ERROR_S44.clone().build_with_path(
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token".to_owned(),
                                            value: class.generic_definings.len().to_string(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_owned(),
                                            value: "0".to_string(),
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
                                    self.pos,
                                ),
                            );
                        }
                    }
                    Collecting::Generic(generic_data) => {
                        found = DefinerCollecting::Generic(definers::GenericType {
                            rtype: generic_data.name.clone(),
                            hash: generic_data.hash,
                            pos: generic_data.pos,
                        });
                    }
                    Collecting::Enum(enum_data) => {
                        found = DefinerCollecting::Generic(definers::GenericType {
                            rtype: enum_data.name.clone(),
                            hash: enum_data.hash,
                            pos: enum_data.pos,
                        });
                    }
                    _ => {
                        errors.push(
                            error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: self.rtype.clone(),
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
                    Processors::Class(class) => {
                        if class.generic_definings.is_empty() {
                            found = DefinerCollecting::Generic(definers::GenericType {
                                rtype: class.name.clone(),
                                hash: class.hash,
                                pos: class.pos,
                            });
                        } else {
                            errors.push(
                                error::error_list::ERROR_S44.clone().build_with_path(
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token".to_owned(),
                                            value: class.generic_definings.len().to_string(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_owned(),
                                            value: "0".to_string(),
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
                                    self.pos,
                                ),
                            );
                        }
                    }
                    Processors::GenericItem(generic_data) => {
                        found = DefinerCollecting::Generic(definers::GenericType {
                            rtype: generic_data.generic_name.clone(),
                            hash: generic_data.hash,
                            pos: generic_data.pos,
                        });
                    }
                    Processors::Enum(enum_data) => {
                        found = DefinerCollecting::Generic(definers::GenericType {
                            rtype: enum_data.name.clone(),
                            hash: enum_data.hash,
                            pos: enum_data.pos,
                        });
                    }
                    _ => {
                        errors.push(
                            error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: self.rtype.clone(),
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
                        value: self.rtype.clone(),
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

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(found)
        }
    }
}
