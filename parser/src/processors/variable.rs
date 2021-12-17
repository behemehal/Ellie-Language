use alloc::{borrow::ToOwned, string::ToString, vec};
use ellie_core::{
    definite::{definers::DefinerCollecting, types::Types},
    error, warning,
};
use ellie_tokenizer::syntax::items::variable::VariableCollector;

impl super::Processor for VariableCollector {
    fn process(self, parser: &mut super::Parser, page_id: u64) {
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
                    "pvr_0x23".to_owned(),
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
                        "pvr_0x23".to_owned(),
                        parser.find_page(page_id).unwrap().path.clone(),
                        self.data.name_pos,
                    ))
            }
        } else {
            let resolved_type = if !self.data.has_value {
                Ok(Types::Void)
            } else {
                super::type_processor::process(
                    self.data.value,
                    parser,
                    page_id,
                    Some(self.data.hash.clone()),
                )
            };

            let resolved_defining = if !self.data.has_type {
                Ok(DefinerCollecting::Dynamic)
            } else {
                super::definer_processor::process(
                    self.data.rtype.definer_type,
                    parser,
                    page_id,
                    Some(self.data.hash.clone()),
                )
            };

            if resolved_type.is_err() || resolved_defining.is_err() {
                let mut type_error = resolved_type.err().unwrap_or(vec![]);
                let defining_error = resolved_defining.err().unwrap_or(vec![]);
                type_error.extend(defining_error);
                parser.informations.extend(&type_error);
            } else {
                #[cfg(feature = "standard_rules")]
                {
                    let (is_correct, fixed) = (ellie_standard_rules::rules::VARIABLE_NAMING_ISSUE
                        .worker)(
                        self.data.name.clone()
                    );
                    if !is_correct
                        && !parser.page_has_file_key_with(page_id, "allow", "VariableNameRule")
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
                            parser.find_page(page_id).unwrap().path.clone(),
                            self.data.name_pos,
                        ))
                    }
                }

                let processed = ellie_core::definite::items::Collecting::Variable(
                    ellie_core::definite::items::variable::Variable {
                        name: self.data.name,
                        constant: self.data.constant,
                        public: self.data.public,
                        value: resolved_type.clone().unwrap(),
                        pos: self.data.pos,
                        name_pos: self.data.name_pos,
                        value_pos: self.data.value_pos,
                        type_pos: self.data.type_pos,
                        rtype: resolved_defining.clone().unwrap(),
                        hash: self.data.hash,
                        has_type: self.data.has_type,
                        has_value: self.data.has_value,
                    },
                );

                if self.data.has_value && self.data.has_type {
                    let defined = parser.resolve_definer_name(resolved_defining.unwrap());
                    let given = parser.resolve_type_name(resolved_type.unwrap());
                    let current_page = parser.find_processed_page(page_id).unwrap();

                    if defined != given {
                        let mut err = error::error_list::ERROR_S3.clone().build_with_path(
                            vec![
                                error::ErrorBuildField {
                                    key: "token1".to_string(),
                                    value: defined,
                                },
                                error::ErrorBuildField {
                                    key: "token2".to_string(),
                                    value: given,
                                },
                            ],
                            "pv_0x107".to_owned(),
                            current_page.path.clone(),
                            self.data.value_pos,
                        );
                        err.reference_block = Some((self.data.type_pos, current_page.path.clone()));
                        err.reference_message = "Defined here".to_owned();
                        err.semi_assist = true;
                        parser.informations.push(&err);
                    } else {
                        current_page.items.push(processed)
                    }
                } else {
                    parser
                        .find_processed_page(page_id)
                        .unwrap()
                        .items
                        .push(processed)
                }
            }
        }
    }
}
