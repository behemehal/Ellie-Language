use alloc::{borrow::ToOwned, string::ToString, vec, vec::Vec};
use ellie_core::{definite::Converter, error, warning};
use ellie_tokenizer::{syntax::items::function, tokenizer::PageType};

impl super::Processor for function::FunctionCollector {
    fn process(self, parser: &mut crate::parser::Parser, page_id: u64) {
        let (duplicate, found) = parser.is_duplicate(
            page_id,
            self.data.name.clone(),
            self.data.hash.clone(),
            self.data.pos,
        );

        if duplicate {
            if let Some((page, cursor_pos)) = found {
                let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.data.name,
                    }],
                    "pfn_0x22".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    self.data.name_pos,
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
                            value: self.data.name,
                        }],
                        "pfn_0x38".to_owned(),
                        parser.find_page(page_id).unwrap().path.clone(),
                        self.data.name_pos,
                    ))
            }
        } else {
            let page = parser.find_page(page_id).unwrap().clone();

            let mut parameters: Vec<ellie_core::definite::items::function::FunctionParameter> =
                Vec::new();
            let mut items = self.data.body.clone();

            let inner_page_id: u64 = ellie_core::utils::generate_hash_u64();
            let mut return_type = self.data.return_type.definer_type.clone().to_definite();
            if !self.data.no_return {
                match super::definer_processor::process(
                    self.data.return_type.definer_type,
                    parser,
                    page_id,
                    None,
                ) {
                    Ok(found_type) => {
                        return_type = found_type;
                    }
                    Err(type_error) => parser.informations.extend(&type_error),
                }
            }

            for (index, parameter) in self.data.parameters.iter().enumerate() {
                if let Some(other_index) = self
                    .data
                    .parameters
                    .iter()
                    .position(|g| g.name == parameter.name)
                {
                    if other_index < index {
                        let mut err = error::error_list::ERROR_S10.clone().build_with_path(
                            vec![],
                            "pfn_0x58".to_owned(),
                            parser.find_page(page_id).unwrap().path.clone(),
                            parameter.pos,
                        );
                        err.reference_block =
                            Some((self.data.parameters[other_index].pos, page.path.clone()));
                        err.reference_message = "Prime is here".to_owned();
                        err.semi_assist = true;
                        parser.informations.push(&err);
                    }

                    let (duplicate, found) = parser.is_duplicate(
                        page_id,
                        parameter.name.clone(),
                        "".to_string(),
                        parameter.pos,
                    );

                    if duplicate {
                        if let Some((page, cursor_pos)) = found {
                            let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: parameter.name.clone(),
                                }],
                                "pfn_0x83".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                parameter.pos,
                            );
                            err.reference_block = Some((cursor_pos, page.path));
                            err.reference_message = "Prime is here".to_owned();
                            err.semi_assist = true;
                            parser.informations.push(&err);
                        } else {
                            parser.informations.push(
                                &error::error_list::ERROR_S24.clone().build_with_path(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: parameter.name.clone(),
                                    }],
                                    "pfn_0x98".to_owned(),
                                    parser.find_page(page_id).unwrap().path.clone(),
                                    parameter.pos,
                                ),
                            )
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
                                        (ellie_standard_rules::rules::CLASS_NAMING_ISSUE.worker)(
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
                                                parser.find_page(page_id).unwrap().path.clone(),
                                                parameter.pos,
                                            ),
                                        )
                                    }
                                }

                                items.push(ellie_tokenizer::processors::items::Processors::FunctionParameter(
                                    ellie_tokenizer::syntax::items::function_parameter::FunctionParameter {
                                        name: parameter.name.clone(),
                                        reference: false,
                                        rtype: e.clone(),
                                        pos: parameter.pos,
                                    },
                                ));
                                parameters.push(
                                    ellie_core::definite::items::function::FunctionParameter {
                                        name: parameter.name.clone(),
                                        pos: parameter.pos,
                                        rtype: e,
                                        multi_capture: parameter.multi_capture,
                                    },
                                );
                            }
                            Err(type_error) => parser.informations.extend(&type_error),
                        }
                    }
                }
            }

            #[cfg(feature = "standard_rules")]
            {
                let (is_correct, fixed) = (ellie_standard_rules::rules::FUNCTION_NAMING_ISSUE
                    .worker)(self.data.name.clone());
                if !is_correct
                    && !parser.page_has_file_key_with(page_id, "allow", "FunctionNameRule")
                {
                    parser
                        .informations
                        .push(&warning::warning_list::WARNING_S1.clone().build(
                            vec![
                                warning::WarningBuildField {
                                    key: "current".to_owned(),
                                    value: self.data.name.clone(),
                                },
                                warning::WarningBuildField {
                                    key: "correct".to_owned(),
                                    value: fixed,
                                },
                            ],
                            parser.find_page(page_id).unwrap().path.clone(),
                            self.data.name_pos,
                        ))
                }
            }

            let inner = ellie_tokenizer::tokenizer::Page {
                hash: inner_page_id,
                inner: Some(page.hash),
                path: page.path.clone(),
                page_type: PageType::FunctionBody,
                items,
                dependents: vec![],
                dependencies: vec![ellie_tokenizer::tokenizer::Dependency {
                    hash: page.hash.clone(),
                    public: false,
                }],
            };
            parser.pages.push(inner);
            parser.process_page(inner_page_id);
            let processed_page = parser.find_processed_page(page_id).unwrap();

            processed_page
                .items
                .push(ellie_core::definite::items::Collecting::Function(
                    ellie_core::definite::items::function::Function {
                        name: self.data.name.clone(),
                        pos: self.data.pos,
                        parameters: parameters,
                        body: processed_page.items.clone(),
                        hash: self.data.hash.clone(),
                        return_type: return_type,
                        public: self.data.public,
                        name_pos: self.data.name_pos,
                        body_pos: self.data.body_pos,
                        parameters_pos: self.data.parameters_pos,
                        return_pos: self.data.return_pos,
                        no_return: self.data.no_return,
                        inner_page_id,
                    },
                ));
        }
    }
}
