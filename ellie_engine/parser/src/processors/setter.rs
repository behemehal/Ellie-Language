use alloc::{borrow::ToOwned, vec};
use ellie_core::error;
use ellie_tokenizer::syntax::items::setter;

impl super::Processor for setter::Setter {
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

        return false;
        /*
        let (duplicate, found) =
            parser.is_duplicate(page_hash, self.name.clone(), self.hash.clone(), self.pos);

        let setter_key_definings = parser
            .processed_pages
            .nth_mut(processed_page_idx)
            .unwrap()
            .unassigned_file_keys
            .clone();

        if utils::is_reserved(
            &self.name,
            setter_key_definings
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

        if duplicate {
            if let Some((page, cursor_pos)) = found {
                let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField::new("token", &self.name)],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    path,
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
            let mut setter_parameter: Option<
                ellie_core::definite::items::function::FunctionParameter,
            > = None;
            let mut items = Vec::new();

            let inner_page_id: usize = ellie_core::utils::generate_hash_usize();

            let parameter = self.parameters.first().unwrap().clone();

            let (duplicate, found) =
                parser.is_duplicate(page_hash, parameter.name.clone(), 0, parameter.name_pos);

            if duplicate {
                if let Some((page, cursor_pos)) = found {
                    let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: parameter.name.clone(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path,
                        parameter.name_pos,
                    );
                    err.reference_block = Some((cursor_pos, page.path));
                    err.reference_message = "Prime is here".to_owned();
                    err.semi_assist = true;
                    parser.informations.push(&err);
                    return true;
                } else {
                    parser
                        .informations
                        .push(&error::error_list::ERROR_S24.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: parameter.name.clone(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            path.clone(),
                            parameter.name_pos,
                        ))
                }
            } else {
                match super::definer_processor::process(
                    parameter.rtype.definer_type.clone(),
                    parser,
                    page_hash,
                    None,
                ) {
                    Ok(e) => {
                        #[cfg(feature = "standard_rules")]
                        {
                            let (is_correct, fixed) =
                                (ellie_standard_rules::rules::FUNCTION_PARAM_NAMING_ISSUE.worker)(
                                    parameter.name.clone(),
                                );
                            if !is_correct
                                && !parser.global_key_matches(
                                    page_hash,
                                    "allow",
                                    "FunctionParameterNameRule",
                                )
                            {
                                parser.informations.push(
                                    &warning::warning_list::WARNING_S3.clone().build(
                                        vec![
                                            warning::WarningBuildField {
                                                key: "current".to_owned(),
                                                value: parameter.name.clone(),
                                            },
                                            warning::WarningBuildField {
                                                key: "correct".to_owned(),
                                                value: fixed,
                                            },
                                        ],
                                        path.clone(),
                                        parameter.name_pos,
                                    ),
                                )
                            }
                        }

                        items.push(ellie_tokenizer::processors::items::Processors::FunctionParameter(
                                        ellie_tokenizer::syntax::items::function_parameter::FunctionParameter {
                                            name: parameter.name.clone(),
                                            reference: false,
                                            rtype: e.clone(),
                                            name_pos: parameter.name_pos,
                                            rtype_pos: parameter.rtype_pos,
                                            hash: generate_hash_usize()
                                        },
                                    ));
                        setter_parameter =
                            Some(ellie_core::definite::items::function::FunctionParameter {
                                name: parameter.name.clone(),
                                rtype: e,
                                multi_capture: parameter.multi_capture,
                                name_pos: parameter.name_pos,
                                rtype_pos: parameter.rtype_pos,
                            });
                    }
                    Err(type_error) => {
                        parser.informations.extend(&type_error);
                        return false;
                    }
                }
            }

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

            let page = parser.pages.nth_mut(page_idx).unwrap();

            let mut dependencies = vec![ellie_tokenizer::tokenizer::Dependency {
                hash: page_hash,
                processed: false,
                module: None,
                deep_link: None,
                public: false,
            }];
            dependencies.extend(page.dependencies.clone());
            items.extend(self.body.clone());

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
            parser.pages.push_page(inner);
            let processed_page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();

            if setter_parameter.is_none() {
                panic!("{:?}", parser.informations);
            }

            processed_page
                .items
                .push(ellie_core::definite::items::Collecting::Setter(
                    ellie_core::definite::items::setter::Setter {
                        name: self.name.clone(),
                        pos: self.pos,
                        rtype: setter_parameter.clone().unwrap().rtype.clone(),
                        param_name: setter_parameter.clone().unwrap().name.clone(),
                        file_keys: processed_page.unassigned_file_keys.clone(),
                        hash: self.hash.clone(),
                        public: self.public,
                        name_pos: self.name_pos,
                        body_pos: self.body_pos,
                        parameters_pos: self.parameters_pos,
                        inner_page_id,
                        rtype_pos: setter_parameter.clone().unwrap().rtype_pos,
                        param_name_pos: setter_parameter.unwrap().name_pos,
                    },
                ));
            processed_page.unassigned_file_keys = vec![];

            parser.process_page(inner_page_id);
            true
        }
        */
    }
}
