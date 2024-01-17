use crate::{deep_search_extensions::resolve_type, parser::DeepSearchItems};
use alloc::{borrow::ToOwned, boxed::Box, string::ToString, vec, vec::Vec};
use ellie_core::{definite::definers, definite::definers::DefinerCollecting, defs::Cursor, error};
use ellie_tokenizer::syntax::items::definers::GenericType;

impl super::DefinerParserProcessor for GenericType {
    fn process(
        &self,
        options: &mut super::DefinerParserProcessorOptions,
    ) -> Result<DefinerCollecting, alloc::vec::Vec<ellie_core::error::Error>> {
        let mut errors = vec![];
        let mut found = DefinerCollecting::Dynamic;

        let deep_search_result = options.parser.deep_search(
            options.page_id,
            self.rtype.clone(),
            options.ignore_hash,
            vec![],
            0,
            None,
        );

        if deep_search_result.found {
            match deep_search_result.found_item {
                crate::parser::DeepSearchItems::Class(e) => {
                    if e.generic_definings.is_empty() {
                        found = DefinerCollecting::Generic(definers::GenericType {
                            rtype: e.name,
                            hash: e.hash,
                            pos: e.pos,
                        });
                    } else {
                        errors.push(
                            error::error_list::ERROR_S44.clone().build_with_path(
                                vec![
                                    error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: e.generic_definings.len().to_string(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_owned(),
                                        value: "0".to_string(),
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
                            ),
                        );
                    }
                }
                crate::parser::DeepSearchItems::BrokenPageGraph => todo!(),
                crate::parser::DeepSearchItems::MixUp(_) => todo!(),
                crate::parser::DeepSearchItems::None => {
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
                crate::parser::DeepSearchItems::GenericItem(generic) => {
                    found = DefinerCollecting::Generic(definers::GenericType {
                        rtype: generic.generic_name.clone(),
                        hash: generic.hash,
                        pos: generic.pos,
                    });
                }
                crate::parser::DeepSearchItems::Enum(enum_data) => {
                    found = DefinerCollecting::Generic(definers::GenericType {
                        rtype: enum_data.name,
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
