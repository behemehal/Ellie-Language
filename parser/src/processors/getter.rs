use alloc::{borrow::ToOwned, vec};
use ellie_core::{definite::Converter, error, warning};
use ellie_tokenizer::{syntax::items::getter, tokenizer::PageType};

impl super::Processor for getter::Getter {
    fn process(self, parser: &mut crate::parser::Parser, page_id: u64) -> bool {
        let (duplicate, found) =
            parser.is_duplicate(page_id, self.name.clone(), self.hash.clone(), self.pos);
        let page = parser.find_page(page_id).unwrap().clone();

        if duplicate {
            if let Some((page, cursor_pos)) = found {
                let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.name,
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    parser.find_page(page_id).unwrap().path.clone(),
                    self.name_pos,
                );
                err.reference_block = Some((cursor_pos, page.path));
                err.reference_message = "Prime is here".to_owned();
                err.semi_assist = true;
                parser.informations.push(&err);
            } else {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S24.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: self.name,
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        page.path.clone(),
                        self.name_pos,
                    ))
            }
            true
        } else {
            let items = self.body.clone();
            let inner_page_id: u64 = ellie_core::utils::generate_hash_u64();
            let return_type = self.return_type.definer_type.clone().to_definite();

            #[cfg(feature = "standard_rules")]
            {
                let (is_correct, fixed) =
                    (ellie_standard_rules::rules::FUNCTION_NAMING_ISSUE.worker)(self.name.clone());
                if !is_correct
                    && !parser.page_has_file_key_with(page_id, "allow", "FunctionNameRule")
                {
                    parser
                        .informations
                        .push(&warning::warning_list::WARNING_S1.clone().build(
                            vec![
                                warning::WarningBuildField {
                                    key: "current".to_owned(),
                                    value: self.name.clone(),
                                },
                                warning::WarningBuildField {
                                    key: "correct".to_owned(),
                                    value: fixed,
                                },
                            ],
                            page.path.clone(),
                            self.name_pos,
                        ))
                }
            }

            let mut dependencies = vec![ellie_tokenizer::tokenizer::Dependency {
                hash: page.hash.clone(),
                processed: false,
                module: None,
                deep_link: None,
                public: false,
            }];
            dependencies.extend(page.dependencies);

            let inner = ellie_tokenizer::tokenizer::Page {
                hash: inner_page_id,
                inner: Some(page.hash),
                path: page.path.clone(),
                page_type: PageType::FunctionBody,
                items,
                dependents: vec![],
                dependencies,
                ..Default::default()
            };
            parser.pages.push(inner);

            let processed_page = parser.find_processed_page(page_id).unwrap();

            processed_page
                .items
                .push(ellie_core::definite::items::Collecting::Getter(
                    ellie_core::definite::items::getter::Getter {
                        name: self.name.clone(),
                        pos: self.pos,
                        hash: self.hash.clone(),
                        return_type: return_type.clone(),
                        public: self.public,
                        name_pos: self.name_pos,
                        body_pos: self.body_pos,
                        return_pos: self.return_pos,
                        inner_page_id,
                    },
                ));
            parser.process_page(inner_page_id);
            let found_ret = parser
                .find_processed_page(inner_page_id)
                .unwrap()
                .items
                .clone()
                .into_iter()
                .find_map(|item| match item {
                    ellie_core::definite::items::Collecting::Ret(e) => Some(e),
                    _ => None,
                });

            if let Some(ret) = found_ret {
                if parser.informations.has_no_errors() {
                    match parser.compare_defining_with_type(return_type, ret.value, inner_page_id) {
                        Ok((same, defined, rtype)) => {
                            if !same {
                                let mut err = error::error_list::ERROR_S3.clone().build_with_path(
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_owned(),
                                            value: defined,
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_owned(),
                                            value: rtype,
                                        },
                                    ],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    parser.find_page(page_id).unwrap().path.clone(),
                                    ret.pos,
                                );
                                err.reference_block = Some((self.return_pos, page.path));
                                err.reference_message = "Defined here".to_owned();
                                err.semi_assist = true;
                                parser.informations.push(&err);
                                return false;
                            }
                        }
                        Err(e) => {
                            parser.informations.extend(&e);
                            return false;
                        }
                    }
                }
            }

            true
        }
    }
}
