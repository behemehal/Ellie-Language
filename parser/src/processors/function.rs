use std::println;

use alloc::{borrow::ToOwned, vec, vec::Vec};
use ellie_core::{
    error,
    utils::{self, generate_hash_usize},
    warning,
};
use ellie_tokenizer::{syntax::items::function, tokenizer::PageType};

impl super::Processor for function::FunctionCollector {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        processed_page_idx: usize,
        page_hash: usize,
    ) -> bool {
        let (duplicate, found) = parser.is_duplicate(
            page_hash,
            self.data.name.clone(),
            self.data.hash.clone(),
            self.data.pos,
        );
        let page = parser.pages.nth(page_idx).unwrap().clone();

        let function_key_definings = parser
            .processed_pages
            .nth_mut(processed_page_idx)
            .unwrap()
            .unassigned_file_keys
            .clone();

        if utils::is_reserved(
            &self.data.name,
            function_key_definings
                .iter()
                .find(|x| x.key_name == "dont_fix_variant")
                .is_some(),
        ) {
            parser
                .informations
                .push(&error::error_list::ERROR_S21.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.data.name.clone(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    page.path.clone(),
                    self.data.name_pos,
                ));
        }

        if duplicate {
            if let Some((found_page, cursor_pos)) = found {
                let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField::new("token", &self.data.name)],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    page.path.clone(),
                    self.data.name_pos,
                );
                err.reference_block = Some((cursor_pos, found_page.path));
                err.reference_message = "Prime is here".to_owned();
                err.semi_assist = true;
                parser.informations.push(&err);
            } else {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S24.clone().build_with_path(
                        vec![error::ErrorBuildField::new("token", &self.data.name)],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        page.path.clone(),
                        self.data.name_pos,
                    ))
            }
            true
        } else {
            let mut parameters: Vec<ellie_core::definite::items::function::FunctionParameter> =
                Vec::new();
            let mut items = Vec::new();

            let inner_page_id: usize = ellie_core::utils::generate_hash_usize();
            let mut return_type = match super::definer_processor::process(
                self.data.return_type.definer_type.clone(),
                parser,
                page_hash,
                None,
            ) {
                Ok(e) => e,
                Err(e) => {
                    parser.informations.extend(&e);
                    return false;
                }
            };

            if !self.data.no_return {
                match super::definer_processor::process(
                    self.data.return_type.definer_type.clone(),
                    parser,
                    page_hash,
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
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            page.path.clone(),
                            ellie_core::defs::Cursor {
                                range_start: parameter.name_pos.range_start,
                                range_end: parameter.rtype_pos.range_end,
                            },
                        );
                        err.reference_block = Some((
                            ellie_core::defs::Cursor {
                                range_start: self.data.parameters[other_index].name_pos.range_start,
                                range_end: self.data.parameters[other_index].rtype_pos.range_end,
                            },
                            page.path.clone(),
                        ));
                        err.reference_message = "Prime is here".to_owned();
                        err.semi_assist = true;
                        parser.informations.push(&err);
                    }

                    let (duplicate, found) = parser.is_duplicate(
                        page_hash,
                        parameter.name.clone(),
                        0,
                        parameter.name_pos,
                    );

                    if duplicate {
                        if let Some((page, cursor_pos)) = found {
                            let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: parameter.name.clone(),
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                page.path.clone(),
                                parameter.name_pos,
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
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    page.path.clone(),
                                    parameter.name_pos,
                                ),
                            )
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
                                        (ellie_standard_rules::rules::FUNCTION_PARAM_NAMING_ISSUE
                                            .worker)(
                                            parameter.name.clone()
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
                                        hash: generate_hash_usize()
                                    },
                                ));
                                parameters.push(
                                    ellie_core::definite::items::function::FunctionParameter {
                                        name: parameter.name.clone(),
                                        rtype: e,
                                        multi_capture: parameter.multi_capture,
                                        name_pos: parameter.name_pos,
                                        rtype_pos: parameter.rtype_pos,
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
                if !is_correct && !parser.global_key_matches(page_hash, "allow", "FunctionNameRule")
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
                            page.path.clone(),
                            self.data.name_pos,
                        ))
                }
            }

            if self.data.defining {
                let processed_page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();
                processed_page
                    .items
                    .push(ellie_core::definite::items::Collecting::NativeFunction(
                        ellie_core::definite::items::native_function::NativeFunction {
                            name: self.data.name.clone(),
                            pos: self.data.pos,
                            parameters: parameters,
                            hash: self.data.hash.clone(),
                            return_type: return_type,
                            public: self.data.public,
                            name_pos: self.data.name_pos,
                            parameters_pos: self.data.parameters_pos,
                            return_pos: self.data.return_pos,
                            file_keys: processed_page.unassigned_file_keys.clone(),
                            no_return: self.data.no_return,
                            module_name: parser.module_info.name.clone(),
                        },
                    ));
                processed_page.unassigned_file_keys = vec![];

                true
            } else {
                let mut dependencies = vec![ellie_tokenizer::tokenizer::Dependency {
                    hash: page.hash.clone(),
                    processed: false,
                    module: None,
                    deep_link: Some(page.hash.clone()),
                    public: false,
                }];
                dependencies.extend(page.dependencies);
                items.extend(self.data.body.clone());

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

                processed_page
                    .items
                    .push(ellie_core::definite::items::Collecting::Function(
                        ellie_core::definite::items::function::Function {
                            name: self.data.name.clone(),
                            pos: self.data.pos,
                            parameters: parameters,
                            hash: self.data.hash.clone(),
                            return_type: return_type.clone(),
                            file_keys: processed_page.unassigned_file_keys.clone(),
                            public: self.data.public,
                            name_pos: self.data.name_pos,
                            body_pos: self.data.body_pos,
                            parameters_pos: self.data.parameters_pos,
                            return_pos: self.data.return_pos,
                            no_return: self.data.no_return,
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
                        match parser.compare_defining_with_type(
                            return_type,
                            ret.value,
                            inner_page_id,
                        ) {
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
                                            page.path.clone(),
                                            ret.pos,
                                        ),
                                    );
                                    let mut err =
                                        error::error_list::ERROR_S3.clone().build_with_path(
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
                                            page.path.clone(),
                                            ret.pos,
                                        );
                                    err.reference_block = Some((self.data.return_pos, page.path));
                                    err.reference_message = "Defined here".to_owned();
                                    err.semi_assist = true;
                                    parser.informations.push(&err);
                                    return false;
                                }

                                if !result.same {
                                    let mut err =
                                        error::error_list::ERROR_S3.clone().build_with_path(
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
                                            page.path.clone(),
                                            ret.pos,
                                        );
                                    err.reference_block = Some((self.data.return_pos, page.path));
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
}
