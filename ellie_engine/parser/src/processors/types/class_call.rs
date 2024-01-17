use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite::definers::DefinerCollecting;
use ellie_core::definite::Converter;
use ellie_core::{definite::types, error};
use ellie_tokenizer::processors::types::Processors;
use ellie_tokenizer::syntax::types::class_call_type;

use crate::deep_search_extensions::{
    deep_search, deep_search_hash, find_type, resolve_type, ProcessedDeepSearchItems,
};
use crate::processors::definer::{DefinerParserProcessor, DefinerParserProcessorOptions};
use crate::processors::types::TypeParserProcessorOptions;

impl super::TypeParserProcessor for class_call_type::ClassCallCollector {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        let mut errors = Vec::new();

        let mut resolved_generics = self.data.resolved_generics.clone();
        let resolved_generics_defined = !self.data.resolved_generics.is_empty();

        match (*self.data.target).clone() {
            Processors::Integer(_) => {
                errors.push(
                    error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "int".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        options
                            .parser
                            .find_page(options.page_id)
                            .unwrap()
                            .path
                            .clone(),
                        self.data.keyword_pos,
                    ),
                );
                Err(errors)
            }
            Processors::Byte(_) => {
                errors.push(
                    error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "int".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        options
                            .parser
                            .find_page(options.page_id)
                            .unwrap()
                            .path
                            .clone(),
                        self.data.keyword_pos,
                    ),
                );
                Err(errors)
            }
            Processors::Decimal(e) => {
                errors.push(
                    error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: if e.data.is_double {
                                "double".to_string()
                            } else {
                                "float".to_string()
                            },
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        options
                            .parser
                            .find_page(options.page_id)
                            .unwrap()
                            .path
                            .clone(),
                        self.data.keyword_pos,
                    ),
                );
                Err(errors)
            }
            Processors::Char(_) => {
                errors.push(
                    error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "char".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        options
                            .parser
                            .find_page(options.page_id)
                            .unwrap()
                            .path
                            .clone(),
                        self.data.keyword_pos,
                    ),
                );
                Err(errors)
            }
            Processors::String(_) => {
                errors.push(
                    error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "string".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        options
                            .parser
                            .find_page(options.page_id)
                            .unwrap()
                            .path
                            .clone(),
                        self.data.keyword_pos,
                    ),
                );
                Err(errors)
            }
            Processors::Collective(_) => {
                errors.push(
                    error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "collective".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        options
                            .parser
                            .find_page(options.page_id)
                            .unwrap()
                            .path
                            .clone(),
                        self.data.keyword_pos,
                    ),
                );
                Err(errors)
            }
            Processors::Cloak(cloak) => {
                if cloak.data.collective.len() == 1 {
                    unimplemented!()
                } else {
                    errors.push(
                        error::error_list::ERROR_S11.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: "cloak".to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            options
                                .parser
                                .find_page(options.page_id)
                                .unwrap()
                                .path
                                .clone(),
                            self.data.keyword_pos,
                        ),
                    );
                    Err(errors)
                }
            }
            Processors::Array(_) => {
                errors.push(
                    error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "collective".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        options
                            .parser
                            .find_page(options.page_id)
                            .unwrap()
                            .path
                            .clone(),
                        self.data.keyword_pos,
                    ),
                );
                Err(errors)
            }
            Processors::Variable(variable) => {
                let deep_search_result = options.parser.deep_search(
                    options.page_id,
                    variable.data.value.clone(),
                    options.ignore_hash,
                    Vec::new(),
                    0,
                    options.variable_pos,
                );

                if deep_search_result.found {
                    match deep_search_result.found_item {
                        crate::parser::DeepSearchItems::Class(e) => {
                            let undefined_generics = self
                                .data
                                .generic_parameters
                                .iter()
                                .filter_map(|g| {
                                    match g.value.process(
                                        DefinerParserProcessorOptions::new(
                                            options.parser,
                                            options.page_id,
                                        )
                                        .optional_ignore_hash(options.ignore_hash)
                                        .build(),
                                    ) {
                                        Ok(e) => {
                                            if !resolved_generics_defined {
                                                resolved_generics.push(e);
                                            }
                                            None
                                        }
                                        Err(err) => {
                                            errors.extend(err);
                                            Some(g)
                                        }
                                    }
                                })
                                .collect::<Vec<_>>();
                            if e.generic_definings.len() != self.data.generic_parameters.len() {
                                let mut error =
                                    error::error_list::ERROR_S44.clone().build_with_path(
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: e.generic_definings.len().to_string(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_string(),
                                                value: self
                                                    .data
                                                    .generic_parameters
                                                    .len()
                                                    .to_string(),
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
                                        self.data.target_pos,
                                    );
                                error.builded_message.builded +=
                                    " (https://github.com/behemehal/Ellie-Language/issues/59)";
                                error.reference_block = Some((
                                    e.name_pos,
                                    options
                                        .parser
                                        .find_page(options.page_id)
                                        .unwrap()
                                        .path
                                        .clone(),
                                ));
                                error.reference_message = "Defined here".to_owned();
                                errors.push(error);
                                Err(errors)
                            } else if !undefined_generics.is_empty() {
                                for g in undefined_generics {
                                    errors.push(
                                        error::error_list::ERROR_S6.clone().build_with_path(
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: g.value.clone().to_definite().to_string(),
                                            }],
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
                                            g.pos,
                                        ),
                                    );
                                }
                                Err(errors)
                            } else {
                                let constructor = e.body.iter().find_map(|x| match x {
                                    ellie_tokenizer::processors::items::Processors::Constructor(
                                        e,
                                    ) => Some(e),
                                    _ => None,
                                });

                                if constructor.is_some() {
                                    if constructor.unwrap().parameters.len()
                                        != self.data.parameters.len()
                                    {
                                        errors.push(
                                            error::error_list::ERROR_S7.clone().build_with_path(
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "name".to_string(),
                                                        value: e.name.clone(),
                                                    },
                                                    error::ErrorBuildField {
                                                        key: "token".to_string(),
                                                        value: constructor
                                                            .unwrap()
                                                            .parameters
                                                            .len()
                                                            .to_string(),
                                                    },
                                                    error::ErrorBuildField {
                                                        key: "token2".to_string(),
                                                        value: self
                                                            .data
                                                            .parameters
                                                            .len()
                                                            .to_string(),
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
                                                self.data.target_pos,
                                            ),
                                        );
                                        return Err(errors);
                                    } else {
                                        let class_page = deep_search_hash(
                                            options.parser,
                                            options.page_id,
                                            e.hash,
                                            Vec::new(),
                                            0,
                                        );
                                        let belonging_class =
                                            class_page.found_item.as_class().unwrap_or_else(|| {
                                                unreachable!(
                                                    "options.parser should have prevented this"
                                                )
                                            });
                                        let constructor_elements: Vec<_> = constructor
                                            .unwrap()
                                            .parameters
                                            .iter()
                                            .enumerate()
                                            .filter_map(|(index, x)| {
                                                let attribute_search =  deep_search(
                                                    options.parser,
                                                    belonging_class.inner_page_id,
                                                    x.name.clone(),
                                                    Some(belonging_class.hash),
                                                    vec![],
                                                    0,
                                                );

                                                if attribute_search.found {
                                                    match attribute_search.found_item {
                                                        ProcessedDeepSearchItems::Variable(variable) => {
                                                            if variable.has_type {
                                                                if !belonging_class.generic_definings.is_empty() && belonging_class.generic_definings.len() > index && matches!(variable.rtype.clone(), DefinerCollecting::Generic(e) if e.hash == belonging_class.generic_definings[index].hash) {
                                                                    Some(
                                                                        resolved_generics[index].clone()
                                                                    )
                                                                } else if variable.has_type {
                                                                    Some(variable.rtype)
                                                                } else {
                                                                    resolve_type(variable.value, options.page_id, options.parser, &mut errors, options.variable_pos)
                                                                }
                                                            } else {
                                                                let mut errors = Vec::new();
                                                                let resolved = resolve_type(variable.value, belonging_class.inner_page_id, options.parser, &mut errors, Some(variable.value_pos));
                                                                if !errors.is_empty() {
                                                                    panic!("options.parser should have prevented this{:?}", errors);
                                                                }
                                                                resolved
                                                            }
                                                    },
                                                        ProcessedDeepSearchItems::Function(_) => todo!(),
                                                        ProcessedDeepSearchItems::Class(e) => {
                                                            Some(DefinerCollecting::Generic( ellie_core::definite::definers::GenericType { rtype: e.name, pos: e.pos, hash: e.hash }))
                                                        }
                                                        _ => unreachable!("options.parser should have prevented this: {:?}", attribute_search)
                                                    }
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect();

                                        //Ignore if length is not a match
                                        if constructor.unwrap().parameters.len()
                                            == constructor_elements.len()
                                        {
                                            for (index, element) in
                                                constructor_elements.iter().enumerate()
                                            {
                                                //element.convert_generic();

                                                let matching_param = &self.data.parameters[index];

                                                let element_to_be_compared =
                                                    match self.data.generic_parameters.get(index) {
                                                        Some(generic_param) => generic_param
                                                            .value
                                                            .process(
                                                                DefinerParserProcessorOptions::new(
                                                                    options.parser,
                                                                    options.page_id,
                                                                )
                                                                .optional_ignore_hash(
                                                                    options.ignore_hash,
                                                                )
                                                                .build(),
                                                            )
                                                            .unwrap(),
                                                        None => element.clone(),
                                                    };

                                                let mut _options = TypeParserProcessorOptions::new(
                                                    options.parser,
                                                    options.page_id,
                                                );

                                                match matching_param.value.process(
                                                    _options
                                                        .dont_exclude_getter()
                                                        .dont_ignore_type()
                                                        .dont_include_setter()
                                                        .optional_variable_pos(options.variable_pos)
                                                        .optional_ignore_hash(options.ignore_hash)
                                                        .build(),
                                                ) {
                                                    Ok(resolved_type) => {
                                                        let comperable = options
                                                            .parser
                                                            .compare_defining_with_type(
                                                                element_to_be_compared.clone(),
                                                                resolved_type.clone(),
                                                                options.page_id,
                                                            );
                                                        let path = options
                                                            .parser
                                                            .find_page(options.page_id)
                                                            .unwrap()
                                                            .path
                                                            .clone();
                                                        match comperable {
                                                            Ok(result) => {
                                                                if result.requires_cast {
                                                                    options.parser.informations.push(
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
                                                                        matching_param.pos,
                                                                        ),
                                                                    );
                                                                    let err = error::error_list::ERROR_S3
                                                                    .clone()
                                                                    .build_with_path(
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
                                                                        path,
                                                                        matching_param.pos,
                                                                    );
                                                                    errors.push(err);
                                                                    return Err(errors);
                                                                }

                                                                if !result.same {
                                                                    let err = error::error_list::ERROR_S3
                                                                        .clone()
                                                                        .build_with_path(
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
                                                                            options.parser.find_page(options.page_id).unwrap().path.clone(),
                                                                            matching_param.pos,
                                                                        );
                                                                    errors.push(err);
                                                                    return Err(errors);
                                                                }
                                                            }
                                                            Err(err) => errors.extend(err),
                                                        }
                                                    }
                                                    Err(err) => errors.extend(err),
                                                }
                                            }
                                        }
                                    }
                                }
                                Ok(types::Types::ClassCall(
                                    ellie_core::definite::types::class_call::ClassCall {
                                        target: Box::new(ellie_core::definite::types::Types::VariableType(
                                            ellie_core::definite::types::variable::VariableType {
                                                value: variable.data.value,
                                                pos: self.data.target_pos,
                                                reference: e.hash,
                                            },
                                        )),
                                        keyword_pos: self.data.keyword_pos,
                                        target_pos: self.data.target_pos,
                                        resolved_generics,
                                        generic_parameters: self.data.generic_parameters.iter().map(|x| {
                                            let definite_type = match x.value.clone() {
                                                ellie_tokenizer::syntax::items::definers::DefinerTypes::ParentGeneric(_) => todo!(),
                                                ellie_tokenizer::syntax::items::definers::DefinerTypes::Generic(generic) => {
                                                    //ellie_core::definite::Converter::to_definite(x.value.clone())
                                                    let found_type = find_type(generic.rtype.clone(), options.page_id, options.parser).unwrap();
                                                    ellie_core::definite::definers::DefinerCollecting::Generic(
                                                        ellie_core::definite::definers::GenericType {
                                                            rtype: generic.rtype.clone(),
                                                            pos: x.pos,
                                                            hash: found_type.hash,
                                                        }
                                                    )
                                                },
                                                _ => todo!(),
                                            };
                                            ellie_core::definite::types::class_call::ClassCallGenericParameter {
                                                value: definite_type,
                                                pos: x.pos
                                            }
                                        }).collect::<Vec<_>>(),
                                        params: self.data.parameters.iter().map(|x| types::class_call::ClassCallParameter { value: x.value.to_definite(), pos: x.pos }).collect::<Vec<_>>(),
                                        pos: self.data.pos,
                                    },
                                ))
                            }
                        }
                        crate::parser::DeepSearchItems::Variable(e) => {
                            errors.push(
                                error::error_list::ERROR_S31.clone().build_with_path(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: "variable".to_string(),
                                    }],
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
                                    e.pos,
                                ),
                            );
                            Err(errors)
                        }
                        crate::parser::DeepSearchItems::Function(e) => {
                            errors.push(
                                error::error_list::ERROR_S31.clone().build_with_path(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: "function".to_string(),
                                    }],
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
                                    e.name_pos,
                                ),
                            );
                            Err(errors)
                        }
                        crate::parser::DeepSearchItems::ImportReference(e) => {
                            errors.push(
                                error::error_list::ERROR_S31.clone().build_with_path(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: "variable".to_string(),
                                    }],
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
                                    e.reference_pos,
                                ),
                            );
                            Err(errors)
                        }
                        crate::parser::DeepSearchItems::ClassInstance(_) => todo!(),
                        crate::parser::DeepSearchItems::GenericItem(_) => todo!(),
                        crate::parser::DeepSearchItems::FunctionParameter(_) => {
                            unimplemented!()
                        }
                        crate::parser::DeepSearchItems::ConstructorParameter(_) => {
                            unimplemented!()
                        }
                        _ => {
                            errors.push(
                                error::error_list::ERROR_S31.clone().build_with_path(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: variable.data.value,
                                    }],
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
                                    variable.data.pos,
                                ),
                            );
                            Err(errors)
                        }
                    }
                } else {
                    errors.push(
                        error::error_list::ERROR_S6.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: variable.data.value,
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            options
                                .parser
                                .find_page(options.page_id)
                                .unwrap()
                                .path
                                .clone(),
                            variable.data.pos,
                        ),
                    );
                    Err(errors)
                }
            }
            Processors::Negative(_) => {
                errors.push(
                    error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "bool".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        options
                            .parser
                            .find_page(options.page_id)
                            .unwrap()
                            .path
                            .clone(),
                        self.data.keyword_pos,
                    ),
                );
                Err(errors)
            }
            Processors::ClassCall(_) => {
                errors.push(
                    error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "classCall".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        options
                            .parser
                            .find_page(options.page_id)
                            .unwrap()
                            .path
                            .clone(),
                        self.data.keyword_pos,
                    ),
                );
                Err(errors)
            }
            Processors::Operator(_) => todo!(),
            Processors::Reference(_) => todo!(),
            Processors::BraceReference(_) => todo!(),
            Processors::FunctionCall(_) => todo!(),
            Processors::AsKeyword(_) => todo!(),
            Processors::NullResolver(_) => todo!(),
            Processors::EnumData(_) => todo!(),
        }
    }
}
