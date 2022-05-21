use crate::deep_search_extensions::resolve_type;
use alloc::{borrow::ToOwned, boxed::Box, string::ToString, vec, vec::Vec};
use ellie_core::{
    definite::{definers::DefinerCollecting, types::Types},
    defs::{self, Cursor},
    error,
};
use ellie_tokenizer::{
    syntax::items::condition::{Condition, ConditionType},
    tokenizer::PageType,
};

impl super::Processor for Condition {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        processed_page_idx: usize,
        page_hash: u64,
    ) -> bool {
        let mut common_return: Option<(
            Cursor,
            Option<(DefinerCollecting, Cursor)>,
            ellie_tokenizer::syntax::items::condition::ConditionType,
        )> = None;

        let path = parser.pages.nth(page_idx).unwrap().path.clone();
        //(processed_page_id, conditionType)
        let mut chains: Vec<(u64, Types)> = Vec::new();

        for chain in &self.chains {
            let condition_type: Types;
            let mut errors = Vec::new();
            if chain.rtype == ConditionType::Else {
                condition_type = Types::Void;
            } else {
                let condition_pos = chain.condition.clone().current.get_pos();
                match crate::processors::type_processor::process(
                    chain.condition.clone().current.clone(),
                    parser,
                    page_hash,
                    None,
                    false,
                ) {
                    Ok(condition) => {
                        condition_type = condition.clone();
                        let condition_type = resolve_type(
                            condition,
                            page_hash,
                            parser,
                            &mut errors,
                            Some(condition_pos),
                        );
                        if errors.len() > 0 {
                            parser.informations.extend(&errors);
                            return false;
                        }
                        let condition_type = condition_type.unwrap();

                        //If condition type is not boolean, we can't continue
                        if condition_type.to_string() != "bool" {
                            parser.informations.push(
                                &error::error_list::ERROR_S3.clone().build_with_path(
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: "bool".to_string(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: condition_type.to_string(),
                                        },
                                    ],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    path.clone(),
                                    condition_pos,
                                ),
                            );
                        }
                    }
                    Err(e) => {
                        parser.informations.extend(&e);
                        return false;
                    }
                }
            }

            let page = parser.pages.nth_mut(page_idx).unwrap().clone();
            let inner_page_id: u64 = ellie_core::utils::generate_hash_u64();
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
                items: chain.code.clone(),
                dependents: vec![],
                dependencies,
                ..Default::default()
            };
            parser.pages.push_page(inner);
            chains.push((inner_page_id, condition_type));

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
                match resolve_type(ret.value, inner_page_id, parser, &mut errors, Some(ret.pos)) {
                    Some(ret_type) => match common_return {
                        Some(e) => {
                            match e.1 {
                                Some(previous_type) => {
                                    if previous_type.0 != ret_type {
                                        let mut error =
                                            error::error_list::ERROR_S13.clone().build_with_path(
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "token".to_string(),
                                                        value: match e.2 {
                                                            ConditionType::If => "if",
                                                            ConditionType::ElseIf => "else if",
                                                            ConditionType::Else => "else",
                                                        }
                                                        .to_string(),
                                                    },
                                                    error::ErrorBuildField {
                                                        key: "token1".to_string(),
                                                        value: match chain.rtype {
                                                            ConditionType::If => "if",
                                                            ConditionType::ElseIf => "else if",
                                                            ConditionType::Else => "else",
                                                        }
                                                        .to_string(),
                                                    },
                                                ],
                                                alloc::format!(
                                                    "{}:{}:{}",
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!()
                                                ),
                                                path.clone(),
                                                e.0,
                                            );

                                        error.reference_block =
                                            Some((previous_type.1, path.clone()));
                                        error.reference_message = "Type mismatch".to_string();

                                        parser.informations.push(&error);
                                    }
                                }
                                None => {
                                    let mut error =
                                        error::error_list::ERROR_S13.clone().build_with_path(
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: match e.2 {
                                                        ConditionType::If => "if",
                                                        ConditionType::ElseIf => "else if",
                                                        ConditionType::Else => "else",
                                                    }
                                                    .to_string(),
                                                },
                                                error::ErrorBuildField {
                                                    key: "token1".to_string(),
                                                    value: match chain.rtype {
                                                        ConditionType::If => "if",
                                                        ConditionType::ElseIf => "else if",
                                                        ConditionType::Else => "else",
                                                    }
                                                    .to_string(),
                                                },
                                            ],
                                            alloc::format!(
                                                "{}:{}:{}",
                                                file!().to_owned(),
                                                line!(),
                                                column!()
                                            ),
                                            path.clone(),
                                            e.0,
                                        );
                                    error.reference_block = Some((ret.pos, path.clone()));
                                    error.reference_message = "Type mismatch".to_string();
                                    parser.informations.push(&error);
                                }
                            }

                            common_return = Some((
                                chain.keyword_pos,
                                Some((ret_type, ret.pos)),
                                chain.rtype.clone(),
                            ));
                        }
                        None => {
                            common_return = Some((
                                chain.keyword_pos,
                                Some((ret_type, ret.pos)),
                                chain.rtype.clone(),
                            ));
                        }
                    },
                    None => {
                        //Parser should prevent this
                        unreachable!()
                    }
                };
            } else {
                match common_return.clone() {
                    Some(e) => {
                        match e.1 {
                            Some(f) => {
                                let mut error =
                                    error::error_list::ERROR_S13.clone().build_with_path(
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: match e.2 {
                                                    ConditionType::If => "if",
                                                    ConditionType::ElseIf => "else if",
                                                    ConditionType::Else => "else",
                                                }
                                                .to_string(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token1".to_string(),
                                                value: match chain.rtype {
                                                    ConditionType::If => "if",
                                                    ConditionType::ElseIf => "else if",
                                                    ConditionType::Else => "else",
                                                }
                                                .to_string(),
                                            },
                                        ],
                                        alloc::format!(
                                            "{}:{}:{}",
                                            file!().to_owned(),
                                            line!(),
                                            column!()
                                        ),
                                        path.clone(),
                                        e.0,
                                    );
                                error.reference_block = Some((f.1, path.clone()));
                                error.reference_message = "Type mismatch".to_string();
                                parser.informations.push(&error);
                            }
                            None => (),
                        }

                        common_return = Some((chain.keyword_pos, None, chain.rtype.clone()));
                    }
                    None => {
                        common_return = Some((chain.keyword_pos, None, chain.rtype.clone()));
                    }
                }
            }
        }

        let processed_page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();

        processed_page
            .items
            .push(ellie_core::definite::items::Collecting::Condition(
                ellie_core::definite::items::condition::Condition {
                    returns: match common_return {
                        Some(e) => match e.1 {
                            Some(e) => Some(e.0),
                            None => None,
                        },
                        None => None,
                    },
                    chains: self
                        .chains
                        .clone()
                        .iter()
                        .enumerate()
                        .map(|(index, chain)| {
                            ellie_core::definite::items::condition::ConditionChain {
                            rtype: match chain.rtype {
                                ConditionType::If => {
                                    ellie_core::definite::items::condition::ConditionType::If
                                }
                                ConditionType::ElseIf => {
                                    ellie_core::definite::items::condition::ConditionType::ElseIf
                                }
                                ConditionType::Else => {
                                    ellie_core::definite::items::condition::ConditionType::Else
                                }
                            },
                            condition: Box::new(chains[index].clone().1),
                            inner_page_id: chains[index].0,
                            keyword_pos: chain.keyword_pos,
                        }
                        })
                        .collect(),
                    pos: defs::Cursor::default(),
                },
            ));

        true
    }
}
