
use alloc::{borrow::ToOwned, boxed::Box, string::ToString, vec};
use ellie_core::{definite::definers, definite::definers::DefinerCollecting, error};
use ellie_tokenizer::syntax::items::definers::FunctionType;

impl super::DefinerParserProcessor for FunctionType {
    fn process(
        &self,
        options: &mut super::DefinerParserProcessorOptions,
    ) -> Result<DefinerCollecting, alloc::vec::Vec<ellie_core::error::Error>> {
        let mut errors = vec![];
        let mut found = DefinerCollecting::Dynamic;

        let deep_search_result = options.parser.deep_search(
            options.page_id,
            "function".to_string(),
            options.ignore_hash,
            vec![],
            0,
            None,
        );

        if deep_search_result.found {
            match deep_search_result.found_item {
                crate::parser::DeepSearchItems::Class(_) => {
                    let params = self
                        .params
                        .iter()
                        .filter_map(|x| match x.process(options) {
                            Ok(e) => Some(e),
                            Err(err) => {
                                errors.extend(err);
                                None
                            }
                        })
                        .collect();

                    let returning = match self.returning.process(options) {
                        Ok(e) => Some(e),
                        Err(e) => {
                            errors.extend(e);
                            None
                        }
                    };

                    if errors.is_empty() {
                        found = DefinerCollecting::Function(definers::FunctionType {
                            params,
                            returning: Box::new(returning.unwrap()),
                        });
                    } else {
                        return Err(errors);
                    }
                }
                _ => match deep_search_result.found_pos {
                    Some(ref_pos) => {
                        let mut error = error::error_list::ERROR_S45.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: "function".to_string(),
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
                                    value: "function".to_string(),
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
                        value: "function".to_string(),
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
