use alloc::{borrow::ToOwned, vec};
use ellie_core::error;
use ellie_tokenizer::syntax::items::getter;

impl super::Processor for getter::Getter {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        _processed_page_idx: usize,
        _page_hash: usize,
    ) -> bool {
        let path = parser.pages.nth(page_idx).unwrap().path.clone();
        parser
            .informations
            .push(&error::error_list::ERROR_S59.clone().build_with_path(
                vec![error::ErrorBuildField::new("token", &"getter".to_owned())],
                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                path,
                self.name_pos,
            ));
        false

        /*
            let getter_key_definings = parser
                .processed_pages
                .nth_mut(processed_page_idx)
                .unwrap()
                .unassigned_file_keys
                .clone();

            if utils::is_reserved(
                &self.name,
                getter_key_definings
                    .iter()
                    .find(|x| x.key_name == "dont_fix_variant")
                    .is_some(),
            ) {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S21.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: self.name.clone(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path.clone(),
                        self.name_pos,
                    ));
            }

            let (duplicate, found) =
                parser.is_duplicate(page_hash, self.name.clone(), self.hash.clone(), self.pos);

            if duplicate {
                if let Some((page, cursor_pos)) = found {
                    let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                        vec![error::ErrorBuildField::new("token", &self.name)],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path.clone(),
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
                            vec![error::ErrorBuildField::new("token", &self.name)],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            path.clone(),
                            self.name_pos,
                        ))
                }
                true
            } else {
                let mut items = Vec::new();
                let inner_page_id: usize = ellie_core::utils::generate_hash_usize();
                let return_type = self.return_type.definer_type.clone().to_definite();

                #[cfg(feature = "standard_rules")]
                {
                    let (is_correct, fixed) =
                        (ellie_standard_rules::rules::FUNCTION_NAMING_ISSUE.worker)(self.name.clone());
                    if !is_correct && !parser.global_key_matches(page_hash, "allow", "FunctionNameRule")
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
                                path.clone(),
                                self.name_pos,
                            ))
                    }
                }
                let inner = {
                    let page = parser.pages.nth(page_idx).unwrap();
                    let mut dependencies = vec![ellie_tokenizer::tokenizer::Dependency {
                        hash: page.hash.clone(),
                        processed: false,
                        module: None,
                        deep_link: None,
                        public: false,
                    }];
                    dependencies.extend(page.dependencies.clone());
                    items.extend(self.body.clone());

                    ellie_tokenizer::tokenizer::Page {
                        hash: inner_page_id,
                        inner: Some(page.hash),
                        path: path.clone(),
                        page_type: PageType::FunctionBody,
                        items,
                        dependents: vec![],
                        dependencies,
                        ..Default::default()
                    }
                };
                parser.pages.push_page(inner);

                let processed_page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();

                processed_page
                    .items
                    .push(ellie_core::definite::items::Collecting::Getter(
                        ellie_core::definite::items::getter::Getter {
                            name: self.name.clone(),
                            pos: self.pos,
                            hash: self.hash.clone(),
                            file_keys: processed_page.unassigned_file_keys.clone(),
                            return_type: return_type.clone(),
                            public: self.public,
                            name_pos: self.name_pos,
                            body_pos: self.body_pos,
                            return_pos: self.return_pos,
                            inner_page_id,
                        },
                    ));
                processed_page.unassigned_file_keys = vec![];
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
                            Ok(result) => {
                                if result.requires_cast {
                                    parser.informations.push(
                                        &error::error_list::ERROR_S41.clone().build_with_path(
                                            vec![error::ErrorBuildField {
                                                key: "token".to_owned(),
                                                value: "Type helpers are not completely implemented yet. Next error is result of this. Follow progress here (https://github.com/behemehal/EllieWorks/issues/8)".to_owned(),
                                            }],
                                            alloc::format!(
                                                "{}:{}:{}",
                                                file!().to_owned(),
                                                line!(),
                                                column!()
                                            ),
                                            path.clone(),
                                            ret.pos,
                                        ),
                                    );
                                    let mut err = error::error_list::ERROR_S3.clone().build_with_path(
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token1".to_owned(),
                                                value: result.first,
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_owned(),
                                                value: result.second,
                                            },
                                        ],
                                        alloc::format!(
                                            "{}:{}:{}",
                                            file!().to_owned(),
                                            line!(),
                                            column!()
                                        ),
                                        path.clone(),
                                        ret.pos,
                                    );
                                    err.reference_block = Some((self.return_pos, path.clone()));
                                    err.reference_message = "Defined here".to_owned();
                                    err.semi_assist = true;
                                    parser.informations.push(&err);
                                    return false;
                                }

                                if !result.same {
                                    let mut err = error::error_list::ERROR_S3.clone().build_with_path(
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token1".to_owned(),
                                                value: result.first,
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_owned(),
                                                value: result.second,
                                            },
                                        ],
                                        alloc::format!(
                                            "{}:{}:{}",
                                            file!().to_owned(),
                                            line!(),
                                            column!()
                                        ),
                                        path.clone(),
                                        ret.pos,
                                    );
                                    err.reference_block = Some((self.return_pos, path.clone()));
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
        */
    }
}
