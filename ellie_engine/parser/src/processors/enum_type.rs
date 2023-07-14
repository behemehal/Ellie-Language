#![allow(unused_variables)]
#![allow(unreachable_code)]
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, vec};
use ellie_core::{
    definite::items::enum_type::{EnumItem, EnumValue},
    error,
};
use ellie_core::{utils, warning};
use ellie_tokenizer::syntax::items::enum_type::EnumType;

impl super::Processor for EnumType {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        processed_page_idx: usize,
        //Ignore unused variables warning
        page_hash: usize,
    ) -> bool {
        let path = parser.pages.nth(page_idx).unwrap().path.clone();
        parser
            .informations
            .push(&error::error_list::ERROR_S59.clone().build_with_path(
                vec![error::ErrorBuildField::new("token", &"enum".to_owned())],
                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                path,
                self.name_pos,
            ));
        return false;
        let halt = true;
        let (duplicate, found) =
            parser.is_duplicate(page_hash, self.name.clone(), self.hash, self.pos);

        let enum_key_definings = parser
            .processed_pages
            .nth_mut(processed_page_idx)
            .unwrap()
            .unassigned_file_keys
            .clone();

        if utils::is_reserved(
            &self.name,
            enum_key_definings
                .iter()
                .any(|x| x.key_name == "dont_fix_variant"),
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
                        path,
                        self.name_pos,
                    ))
            }
            false
        } else {
            #[cfg(feature = "standard_rules")]
            {
                let (is_correct, fixed) =
                    (ellie_standard_rules::rules::ENUM_NAMING_ISSUE.worker)(self.name.clone());
                if !is_correct && !parser.global_key_matches(page_hash, "allow", "ItemNameRule") {
                    parser
                        .informations
                        .push(&warning::warning_list::WARNING_S6.clone().build(
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

            let mut resolved_items = Vec::new();

            for item in &self.items {
                let resolved_type = if item.has_type {
                    match super::definer_processor::process(
                        item.enum_type.definer_type.clone(),
                        parser,
                        page_hash,
                        None,
                    ) {
                        Ok(e) => Some(e),
                        Err(errors) => {
                            parser.informations.extend(&errors);
                            halt = false;
                            continue;
                        }
                    }
                } else {
                    None
                };

                #[cfg(feature = "standard_rules")]
                {
                    let (is_correct, fixed) = (ellie_standard_rules::rules::ENUM_ITEM_NAMING_ISSUE
                        .worker)(
                        item.identifier.clone()
                    );
                    if !is_correct
                        && !parser.global_key_matches(page_hash, "allow", "EnumItemNameRule")
                    {
                        parser
                            .informations
                            .push(&warning::warning_list::WARNING_S7.clone().build(
                                vec![
                                    warning::WarningBuildField {
                                        key: "current".to_owned(),
                                        value: item.identifier.clone(),
                                    },
                                    warning::WarningBuildField {
                                        key: "correct".to_owned(),
                                        value: fixed,
                                    },
                                ],
                                path.clone(),
                                item.identifier_pos,
                            ))
                    }
                }

                resolved_items.push(EnumItem {
                    identifier: item.identifier.clone(),
                    identifier_pos: item.identifier_pos,
                    type_pos: item.type_pos,
                    value: if item.has_type {
                        EnumValue::Value(resolved_type.unwrap())
                    } else {
                        EnumValue::NoValue
                    },
                })
            }

            let processed_page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();

            let processed = ellie_core::definite::items::Collecting::Enum(
                ellie_core::definite::items::enum_type::EnumType {
                    public: self.public,
                    name: self.name.clone(),
                    hash: self.hash,
                    name_pos: self.name_pos,
                    pos: self.pos,
                    file_keys: processed_page.unassigned_file_keys.clone(),
                    body_pos: self.body_pos,
                    items: resolved_items,
                },
            );

            processed_page.unassigned_file_keys = vec![];
            processed_page.items.push(processed);

            halt
        }
    }
}
