use alloc::{borrow::ToOwned, boxed::Box, string::ToString, vec, vec::Vec};
#[cfg(feature = "standard_rules")]
use ellie_core::warning;
use ellie_core::{
    definite::{
        definers::DefinerCollecting,
        types::{class_call, Types},
    },
    defs, error,
};
use ellie_tokenizer::{syntax::items::variable::VariableCollector, tokenizer::PageType};

use crate::processors::{
    definer::{DefinerParserProcessor, DefinerParserProcessorOptions},
    types::{TypeParserProcessor, TypeParserProcessorOptions},
};

impl super::ItemParserProcessor for VariableCollector {
    fn process(&self, options: &mut super::ItemParserProcessorOptions) -> bool {
        let path = options
            .parser
            .pages
            .nth(options.page_idx)
            .unwrap()
            .path
            .clone();
        let (duplicate, found) = options.parser.is_variable_duplicate(
            options.page_hash,
            self.data.name.clone(),
            self.data.hash,
            self.data.pos,
        );

        if duplicate {
            if let Some((page, cursor_pos)) = found {
                let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField::new("token", &self.data.name)],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    options
                        .parser
                        .pages
                        .nth(options.page_idx)
                        .unwrap()
                        .path
                        .clone(),
                    self.data.name_pos,
                );
                err.reference_block = Some((cursor_pos, page.path));
                err.reference_message = "Prime is here".to_owned();
                err.semi_assist = true;
                options.parser.informations.push(&err);
            } else {
                options.parser.informations.push(
                    &error::error_list::ERROR_S24.clone().build_with_path(
                        vec![error::ErrorBuildField::new("token", &self.data.name)],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path.clone(),
                        self.data.name_pos,
                    ),
                )
            }
            false
        } else {
            let deep_cast = options
                .parser
                .processed_pages
                .nth(options.processed_page_idx)
                .unwrap()
                .unassigned_file_keys
                .clone()
                .into_iter()
                .find(|key| key.key_name == "ellie_deep_cast");

            if let Some(key) = deep_cast {
                let processed = ellie_core::definite::items::Collecting::Variable(
                    ellie_core::definite::items::variable::Variable {
                        name: self.data.name.clone(),
                        constant: self.data.constant,
                        public: self.data.public,
                        value: match key.value {
                            Types::Byte(e) => Types::Byte(e),
                            Types::Integer(e) => Types::Integer(e),
                            Types::Decimal(e) => Types::Decimal(e),
                            Types::Bool(e) => Types::Bool(e),
                            Types::String(e) => Types::String(e),
                            Types::Char(e) => Types::Char(e),
                            Types::VariableType(e) => {
                                if e.value == "true" || e.value == "false" {
                                    Types::Bool(ellie_core::definite::types::bool::BoolType {
                                        value: e.value == "true",
                                    })
                                } else if e.value == "null" {
                                    Types::Null
                                } else {
                                    options.parser.informations.push(
                                        &error::error_list::ERROR_S50.clone().build_with_path(
                                            vec![
                                                error::ErrorBuildField::new(
                                                    "target",
                                                    &self.data.name,
                                                ),
                                                error::ErrorBuildField::new(
                                                    "type",
                                                    &self.data.name,
                                                ),
                                            ],
                                            alloc::format!(
                                                "{}:{}:{}",
                                                file!().to_owned(),
                                                line!(),
                                                column!()
                                            ),
                                            path.clone(),
                                            self.data.name_pos,
                                        ),
                                    );
                                    return false;
                                }
                            }
                            _ => unreachable!(),
                        },
                        pos: self.data.pos,
                        name_pos: self.data.name_pos,
                        file_keys: options
                            .parser
                            .processed_pages
                            .nth(options.processed_page_idx)
                            .unwrap()
                            .unassigned_file_keys
                            .clone(),

                        value_pos: self.data.value_pos,
                        type_pos: self.data.type_pos,
                        rtype: DefinerCollecting::Dynamic,
                        hash: self.data.hash,
                        has_type: self.data.has_type,
                        has_value: self.data.has_value,
                    },
                );

                let processed_page = options
                    .parser
                    .processed_pages
                    .nth_mut(options.processed_page_idx)
                    .unwrap();
                processed_page.unassigned_file_keys = Vec::new();
                processed_page.items.push(processed);
                return true;
            }

            let resolved_defining = if !self.data.has_type {
                Ok(DefinerCollecting::Dynamic)
            } else {
                match self.data.rtype.definer_type.process(
                    &mut DefinerParserProcessorOptions::new(options.parser, options.page_hash)
                        .ignore_hash(self.data.hash)
                        .build(),
                ) {
                    Err(e) => {
                        options.parser.informations.extend(&e);
                        return false;
                    }
                    e => e,
                }
            };

            let resolved_type = if !self.data.has_value {
                let null_able_class = crate::deep_search_extensions::find_type(
                    "nullAble".to_string(),
                    options.page_hash,
                    options.parser,
                );

                match null_able_class {
                    Some(null_able_class) => {
                        let nullable_parameter = if resolved_defining.is_ok()
                            && matches!(resolved_defining.clone().unwrap(), DefinerCollecting::ParentGeneric(parent_generic) if parent_generic.rtype == "nullAble")
                        {
                            resolved_defining
                                .clone()
                                .unwrap()
                                .as_parent_generic()
                                .unwrap()
                                .generics[0]
                                .clone()
                                .value
                        } else {
                            resolved_defining.clone().unwrap()
                        };

                        Ok(Types::ClassCall(
                            ellie_core::definite::types::class_call::ClassCall {
                                target: Box::new(Types::VariableType(
                                    ellie_core::definite::types::variable::VariableType {
                                        value: null_able_class.rtype.clone(),
                                        reference: null_able_class.hash,
                                        pos: defs::Cursor::default(),
                                    },
                                )),
                                resolved_generics: vec![],
                                generic_parameters: vec![class_call::ClassCallGenericParameter {
                                    value: nullable_parameter,
                                    pos: defs::Cursor::default(),
                                }],
                                keyword_pos: defs::Cursor::default(),
                                pos: defs::Cursor::default(),
                                target_pos: defs::Cursor::default(),
                                params: vec![],
                            },
                        ))
                    }
                    None => {
                        options.parser.informations.push(
                            &error::error_list::ERROR_S38.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: "nullAble".to_string(),
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                options
                                    .parser
                                    .processed_pages
                                    .nth_mut(options.processed_page_idx)
                                    .unwrap()
                                    .path
                                    .clone(),
                                self.data.pos,
                            ),
                        );
                        return false;
                    }
                }
            } else {
                self.data.value.process(
                    &mut TypeParserProcessorOptions::new(options.parser, options.page_hash)
                        .variable_pos(self.data.pos)
                        .ignore_hash(self.data.hash)
                        .ignore_type()
                        .build(),
                )
            };

            if resolved_type.is_err() || resolved_defining.is_err() {
                let mut type_error = resolved_type.err().unwrap_or(vec![]);
                let defining_error = resolved_defining.err().unwrap_or(vec![]);
                type_error.extend(defining_error);
                options.parser.informations.extend(&type_error);
                false
            } else {
                #[cfg(feature = "standard_rules")]
                {
                    let (is_correct, fixed) = (ellie_standard_rules::rules::VARIABLE_NAMING_ISSUE
                        .worker)(
                        self.data.name.clone()
                    );
                    if !is_correct
                        && !options.parser.global_key_matches(
                            options.page_hash,
                            "allow",
                            "VariableNameRule",
                        )
                    {
                        options.parser.informations.push(
                            &warning::warning_list::WARNING_S2.clone().build(
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
                                path.clone(),
                                self.data.name_pos,
                            ),
                        )
                    }
                }

                let processed = ellie_core::definite::items::Collecting::Variable(
                    ellie_core::definite::items::variable::Variable {
                        name: self.data.name.clone(),
                        constant: self.data.constant,
                        public: self.data.public,
                        value: resolved_type.clone().unwrap(),
                        pos: self.data.pos,
                        name_pos: self.data.name_pos,
                        file_keys: options
                            .parser
                            .processed_pages
                            .nth(options.processed_page_idx)
                            .unwrap()
                            .unassigned_file_keys
                            .clone(),

                        value_pos: self.data.value_pos,
                        type_pos: self.data.type_pos,
                        rtype: resolved_defining.clone().unwrap(),
                        hash: self.data.hash,
                        has_type: self.data.has_type,
                        has_value: self.data.has_value,
                    },
                );

                if self.data.has_value && self.data.has_type {
                    let comperable = options.parser.compare_defining_with_type(
                        resolved_defining.unwrap(),
                        resolved_type.unwrap().clone(),
                        options.page_hash,
                    );

                    let current_page = options
                        .parser
                        .processed_pages
                        .nth_mut(options.processed_page_idx)
                        .unwrap();
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
                                    current_page.path.clone(),
                                    self.data.value_pos,
                                ),
                            );
                                let mut err = error::error_list::ERROR_S3.clone().build_with_path(
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: result.first,
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: result.second,
                                        },
                                    ],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    current_page.path.clone(),
                                    self.data.value_pos,
                                );
                                err.reference_block =
                                    Some((self.data.type_pos, current_page.path.clone()));
                                err.reference_message = "Defined here".to_owned();
                                err.semi_assist = true;
                                options.parser.informations.push(&err);
                                return false;
                            }
                            if !result.same {
                                let mut err = error::error_list::ERROR_S3.clone().build_with_path(
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: result.first,
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: result.second,
                                        },
                                    ],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    current_page.path.clone(),
                                    self.data.value_pos,
                                );
                                err.reference_block =
                                    Some((self.data.type_pos, current_page.path.clone()));
                                err.reference_message = "Defined here".to_owned();
                                err.semi_assist = true;
                                options.parser.informations.push(&err);
                                false
                            } else {
                                current_page.items.push(processed);
                                true
                            }
                        }
                        Err(err) => {
                            options.parser.informations.extend(&err);
                            false
                        }
                    }
                } else {
                    let processed_page = options
                        .parser
                        .processed_pages
                        .nth_mut(options.processed_page_idx)
                        .unwrap();
                    processed_page.unassigned_file_keys = Vec::new();
                    processed_page.items.push(processed);
                    true
                }
            }
        }
    }
}
