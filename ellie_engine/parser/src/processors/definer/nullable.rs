use crate::{deep_search_extensions::resolve_type, parser::DeepSearchItems};
use alloc::{borrow::ToOwned, boxed::Box, string::ToString, vec, vec::Vec};
use ellie_core::{definite::definers, definite::definers::DefinerCollecting, defs::Cursor, error};
use ellie_tokenizer::syntax::items::definers::NullableType;

impl super::DefinerParserProcessor for NullableType {
    fn process(
        &self,
        options: &mut super::DefinerParserProcessorOptions,
    ) -> Result<DefinerCollecting, alloc::vec::Vec<ellie_core::error::Error>> {
        let mut errors = vec![];
        let mut found = DefinerCollecting::Dynamic;

        let deep_search_result = options.parser.deep_search(
            options.page_id,
            "nullAble".to_string(),
            options.ignore_hash,
            vec![],
            0,
            None,
        );

        if deep_search_result.found {
            match deep_search_result.found_item {
                crate::parser::DeepSearchItems::Class(nullable_class) => {
                    match self.rtype.process(options) {
                        Ok(inner_type) => {
                            if nullable_class.generic_definings.len() != 1 {
                                let mut error =
                                    error::error_list::ERROR_S44.clone().build_with_path(
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: nullable_class
                                                    .generic_definings
                                                    .len()
                                                    .to_string(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_string(),
                                                value: 1.to_string(),
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
                                        self.rtype.get_pos(),
                                    );
                                error.reference_message =
                                    "Does not have required generic parameters".to_string();
                                error.reference_block =
                                    Some((nullable_class.pos, deep_search_result.found_page.path));
                                errors.push(error);
                            } else {
                                found =
                                    DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                                        parent_pos: nullable_class.pos,
                                        generics: vec![definers::GenericParameter {
                                            value: inner_type,
                                            pos: deep_search_result
                                                .found_pos
                                                .unwrap_or(ellie_core::defs::Cursor::default()),
                                        }],
                                        hash: nullable_class.hash,
                                        rtype: "nullAble".to_string(),
                                    });
                            }
                        }
                        Err(err) => errors.extend(err),
                    };
                }
                _ => match deep_search_result.found_pos {
                    Some(ref_pos) => {
                        let mut error = error::error_list::ERROR_S45.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: "nullAble".to_string(),
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
                                    value: "nullAble".to_string(),
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
                        value: "nullAble".to_string(),
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
