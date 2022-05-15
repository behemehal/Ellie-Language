use alloc::{borrow::ToOwned, vec, vec::Vec};
use ellie_core::{definite::Converter, error, warning};
use ellie_tokenizer::{
    syntax::items::{function, setter},
    tokenizer::PageType,
};

impl super::Processor for setter::Setter {
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
            let mut setter_parameter: Option<
                ellie_core::definite::items::function::FunctionParameter,
            > = None;
            let mut items = self.body.clone();

            let inner_page_id: u64 = ellie_core::utils::generate_hash_u64();

            let parameter = self.parameters.first().unwrap().clone();

            let (duplicate, found) =
                parser.is_duplicate(page_id, parameter.name.clone(), 0, parameter.name_pos);

            if duplicate {
                if let Some((page, cursor_pos)) = found {
                    let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: parameter.name.clone(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        parser.find_page(page_id).unwrap().path.clone(),
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
                            page.path.clone(),
                            parameter.name_pos,
                        ))
                }
            } else {
                match super::definer_processor::process(
                    parameter.rtype.definer_type.clone(),
                    parser,
                    page_id,
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
                                && !parser.page_has_file_key_with(
                                    page_id,
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
                                        page.path.clone(),
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
            parser.process_page(inner_page_id);
            true
        }
    }
}
