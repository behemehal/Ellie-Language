use alloc::{borrow::ToOwned, string::ToString, vec, vec::Vec};
#[cfg(feature = "standard_rules")]
use ellie_core::warning;
use ellie_core::{
    definite::{definers::DefinerCollecting, types::Types},
    error,
};
use ellie_tokenizer::syntax::items::variable::VariableCollector;

impl super::Processor for VariableCollector {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        processed_page_idx: usize,
        page_hash: usize,
    ) -> bool {
        let path = parser.pages.nth(page_idx).unwrap().path.clone();
        let (duplicate, found) = parser.is_variable_duplicate(
            page_hash,
            self.data.name.clone(),
            self.data.hash,
            self.data.pos,
        );

        if duplicate {
            if let Some((page, cursor_pos)) = found {
                let mut err = error::error_list::ERROR_S24.clone().build_with_path(
                    vec![error::ErrorBuildField::new("token", &self.data.name)],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    parser.pages.nth(page_idx).unwrap().path.clone(),
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
                        vec![error::ErrorBuildField::new("token", &self.data.name)],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path.clone(),
                        self.data.name_pos,
                    ))
            }
            false
        } else {
            let deep_cast = parser
                .processed_pages
                .nth(processed_page_idx)
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
                                Types::Bool(ellie_core::definite::types::bool::BoolType {
                                    value: if e.value == "true" {
                                        true
                                    } else if e.value == "false" {
                                        false
                                    } else {
                                        //This will give me headache later
                                        unreachable!()
                                    },
                                })
                            }
                            _ => unreachable!(),
                        },
                        pos: self.data.pos,
                        name_pos: self.data.name_pos,
                        file_keys: parser
                            .processed_pages
                            .nth(processed_page_idx)
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

                let processed_page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();
                processed_page.unassigned_file_keys = Vec::new();
                processed_page.items.push(processed);
                return true;
            }

            let resolved_defining = if !self.data.has_type {
                Ok(DefinerCollecting::Dynamic)
            } else {
                super::definer_processor::process(
                    self.data.rtype.definer_type.clone(),
                    parser,
                    page_hash,
                    Some(self.data.hash),
                )
            };

            let resolved_type = if !self.data.has_value {
                Ok(Types::Null)
            } else {
                super::type_processor::process(
                    self.data.value.clone(),
                    parser,
                    page_hash,
                    Some(self.data.hash),
                    false,
                    false,
                    true,
                    Some(self.data.pos),
                )
            };

            if resolved_type.is_err() || resolved_defining.is_err() {
                let mut type_error = resolved_type.err().unwrap_or(vec![]);
                let defining_error = resolved_defining.err().unwrap_or(vec![]);
                type_error.extend(defining_error);
                parser.informations.extend(&type_error);
                false
            } else {
                #[cfg(feature = "standard_rules")]
                {
                    let (is_correct, fixed) = (ellie_standard_rules::rules::VARIABLE_NAMING_ISSUE
                        .worker)(
                        self.data.name.clone()
                    );
                    if !is_correct
                        && !parser.global_key_matches(page_hash, "allow", "VariableNameRule")
                    {
                        parser
                            .informations
                            .push(&warning::warning_list::WARNING_S2.clone().build(
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
                            ))
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
                        file_keys: parser
                            .processed_pages
                            .nth(processed_page_idx)
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
                    let comperable = parser.compare_defining_with_type(
                        resolved_defining.unwrap(),
                        resolved_type.unwrap().clone(),
                        page_hash,
                    );

                    let current_page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();
                    match comperable {
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
                                parser.informations.push(&err);
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
                                parser.informations.push(&err);
                                false
                            } else {
                                current_page.items.push(processed);
                                true
                            }
                        }
                        Err(err) => {
                            parser.informations.extend(&err);
                            false
                        }
                    }
                } else {
                    let processed_page =
                        parser.processed_pages.nth_mut(processed_page_idx).unwrap();
                    processed_page.unassigned_file_keys = Vec::new();
                    processed_page.items.push(processed);
                    true
                }
            }
        }
    }
}
