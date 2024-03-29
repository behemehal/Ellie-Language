use alloc::{borrow::ToOwned, string::ToString, vec, vec::Vec};
#[cfg(feature = "standard_rules")]
use ellie_core::warning;
use ellie_core::{
    error,
    utils::{self, generate_hash_usize},
};
use ellie_tokenizer::{
    syntax::items::function::FunctionCollector,
    tokenizer::{FunctionPageType, PageType},
};

use ellie_core::definite::{
    definers,
    items::{function, native_function, Collecting},
};

use crate::processors::definer::{DefinerParserProcessor, DefinerParserProcessorOptions};

impl super::ItemParserProcessor for FunctionCollector {
    fn process(&self, options: &mut super::ItemParserProcessorOptions) -> bool {
        let (duplicate, found) = options.parser.is_duplicate(
            options.page_hash,
            self.data.name.clone(),
            self.data.hash,
            self.data.pos,
        );
        let page = options.parser.pages.nth(options.page_idx).unwrap().clone();

        let function_key_definings = options
            .parser
            .processed_pages
            .nth_mut(options.processed_page_idx)
            .unwrap()
            .unassigned_file_keys
            .clone();

        if utils::is_reserved(
            &self.data.name,
            function_key_definings
                .iter()
                .any(|x| x.key_name == "dont_fix_variant"),
        ) {
            options.parser.informations.push(
                &error::error_list::ERROR_S21.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: self.data.name.clone(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    page.path.clone(),
                    self.data.name_pos,
                ),
            );
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
                options.parser.informations.push(&err);
            } else {
                options.parser.informations.push(
                    &error::error_list::ERROR_S24.clone().build_with_path(
                        vec![error::ErrorBuildField::new("token", &self.data.name)],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        page.path.clone(),
                        self.data.name_pos,
                    ),
                )
            }
            true
        } else {
            let mut parameters: Vec<function::FunctionParameter> = Vec::new();
            let mut items = Vec::new();

            let inner_page_id: usize = ellie_core::utils::generate_hash_usize();

            let mut return_type = match self.data.return_type.definer_type.process(
                DefinerParserProcessorOptions::new(options.parser, options.page_hash).build(),
            ) {
                Ok(e) => e,
                Err(e) => {
                    options.parser.informations.extend(&e);
                    return false;
                }
            };

            if !self.data.no_return {
                match self.data.return_type.definer_type.process(
                    DefinerParserProcessorOptions::new(options.parser, options.page_hash).build(),
                ) {
                    Ok(found_type) => {
                        return_type = found_type;
                    }
                    Err(type_error) => options.parser.informations.extend(&type_error),
                }
            }

            match page.page_type {
                PageType::ClassBody(class_body) => {
                    let param_hash = generate_hash_usize();
                    items.push(
                        ellie_tokenizer::processors::items::Processors::FunctionParameter(
                            ellie_tokenizer::syntax::items::function_parameter::FunctionParameter {
                                name: "self".to_owned(),
                                reference: false,
                                rtype: ellie_core::definite::definers::DefinerCollecting::Generic(
                                    ellie_core::definite::definers::GenericType {
                                        rtype: "self".to_string(),
                                        pos: class_body.pos,
                                        hash: class_body.hash,
                                    },
                                ),
                                name_pos: class_body.pos,
                                rtype_pos: class_body.pos,
                                hash: param_hash,
                            },
                        ),
                    );
                    parameters.push(function::FunctionParameter {
                        name: "self".to_owned(),
                        rtype: definers::DefinerCollecting::Generic(
                            ellie_core::definite::definers::GenericType {
                                rtype: "self".to_string(),
                                pos: class_body.pos,
                                hash: class_body.hash,
                            },
                        ),
                        multi_capture: false,
                        name_pos: class_body.pos,
                        rtype_pos: class_body.pos,
                        is_mut: false,
                    });
                }
                _ => (),
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
                        options.parser.informations.push(&err);
                    }

                    let (duplicate, found) = options.parser.is_duplicate(
                        options.page_hash,
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
                            options.parser.informations.push(&err);
                        } else {
                            options.parser.informations.push(
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
                        match parameter.rtype.definer_type.process(
                            DefinerParserProcessorOptions::new(options.parser, options.page_hash)
                                .build(),
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
                                        && !options.parser.global_key_matches(
                                            options.page_hash,
                                            "allow",
                                            "FunctionParameterNameRule",
                                        )
                                    {
                                        options.parser.informations.push(
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
                                parameters.push(function::FunctionParameter {
                                    name: parameter.name.clone(),
                                    rtype: e,
                                    multi_capture: parameter.multi_capture,
                                    name_pos: parameter.name_pos,
                                    rtype_pos: parameter.rtype_pos,
                                    is_mut: parameter.is_mut,
                                });
                            }
                            Err(type_error) => options.parser.informations.extend(&type_error),
                        }
                    }
                }
            }

            #[cfg(feature = "standard_rules")]
            {
                let (is_correct, fixed) = (ellie_standard_rules::rules::FUNCTION_NAMING_ISSUE
                    .worker)(self.data.name.clone());
                if !is_correct
                    && !options.parser.global_key_matches(
                        options.page_hash,
                        "allow",
                        "FunctionNameRule",
                    )
                {
                    options.parser.informations.push(
                        &warning::warning_list::WARNING_S1.clone().build(
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
                        ),
                    )
                }
            }

            if self.data.defining {
                let processed_page = options
                    .parser
                    .processed_pages
                    .nth_mut(options.processed_page_idx)
                    .unwrap();
                processed_page.items.push(Collecting::NativeFunction(
                    native_function::NativeFunction {
                        name: self.data.name.clone(),
                        pos: self.data.pos,
                        parameters,
                        hash: self.data.hash,
                        return_type,
                        public: self.data.public,
                        name_pos: self.data.name_pos,
                        parameters_pos: self.data.parameters_pos,
                        return_pos: self.data.return_pos,
                        file_keys: processed_page.unassigned_file_keys.clone(),
                        no_return: self.data.no_return,
                        module_name: options.parser.module_info.name.clone(),
                    },
                ));
                processed_page.unassigned_file_keys = vec![];

                true
            } else {
                let mut dependencies = vec![ellie_tokenizer::tokenizer::Dependency {
                    hash: page.hash,
                    processed: false,
                    module: None,
                    deep_link: Some(page.hash),
                    public: false,
                }];
                dependencies.extend(page.dependencies);
                items.extend(self.data.body.clone());

                let inner = ellie_tokenizer::tokenizer::Page {
                    hash: inner_page_id,
                    inner: Some(page.hash),
                    path: page.path.clone(),
                    page_type: PageType::FunctionBody(FunctionPageType {
                        return_type: return_type.clone(),
                        return_pos: self.data.return_pos,
                    }),
                    items,
                    dependents: vec![],
                    dependencies,
                    ..Default::default()
                };
                options.parser.pages.push_page(inner);

                let processed_page = options
                    .parser
                    .processed_pages
                    .nth_mut(options.processed_page_idx)
                    .unwrap();

                processed_page
                    .items
                    .push(Collecting::Function(function::Function {
                        name: self.data.name.clone(),
                        pos: self.data.pos,
                        parameters,
                        hash: self.data.hash,
                        return_type: return_type.clone(),
                        file_keys: processed_page.unassigned_file_keys.clone(),
                        public: self.data.public,
                        name_pos: self.data.name_pos,
                        body_pos: self.data.body_pos,
                        parameters_pos: self.data.parameters_pos,
                        return_pos: self.data.return_pos,
                        no_return: self.data.no_return,
                        inner_page_id,
                    }));
                processed_page.unassigned_file_keys = vec![];
                true
            }
        }
    }
}
