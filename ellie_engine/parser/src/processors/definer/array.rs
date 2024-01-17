use crate::{deep_search_extensions::resolve_type, parser::DeepSearchItems};
use alloc::{borrow::ToOwned, string::ToString, vec};
use ellie_core::{definite::definers, definite::definers::DefinerCollecting, error};
use ellie_tokenizer::syntax::items::definers::ArrayType;

impl super::DefinerParserProcessor for ArrayType {
    fn process(
        &self,
        options: &mut super::DefinerParserProcessorOptions,
    ) -> Result<DefinerCollecting, alloc::vec::Vec<ellie_core::error::Error>> {
        let mut errors = vec![];
        let mut found = DefinerCollecting::Dynamic;

        let deep_search_result = options.parser.deep_search(
            options.page_id,
            "array".to_string(),
            options.ignore_hash,
            vec![],
            0,
            None,
        );

        if deep_search_result.found {
            match deep_search_result.found_item {
                DeepSearchItems::Class(array_class) => match self.rtype.process(options) {
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
                                found =
                                    DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                                        parent_pos: array_class.pos,
                                        generics: vec![definers::GenericParameter {
                                            value: inner_type,
                                            pos: deep_search_result
                                                .found_pos
                                                .unwrap_or_default(),
                                        }],
                                        hash: array_class.hash,
                                        rtype: "array".to_string(),
                                    });
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
                            found = DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                                parent_pos: array_class.pos,
                                generics: vec![definers::GenericParameter {
                                    value: inner_type,
                                    pos: deep_search_result.found_pos.unwrap_or_default(),
                                }],
                                hash: array_class.hash,
                                rtype: "array".to_string(),
                            });
                        }
                    },
                    Err(e) => errors.extend(e),
                },
                _ => match deep_search_result.found_pos {
                    Some(ref_pos) => {
                        let mut error = error::error_list::ERROR_S45.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
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
                },
            }
        } else {
            errors.push(
                error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
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
