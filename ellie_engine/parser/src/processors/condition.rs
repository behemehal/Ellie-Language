use crate::deep_search_extensions::resolve_type;
use alloc::{borrow::ToOwned, boxed::Box, string::ToString, vec, vec::Vec};
use ellie_core::{definite::types::Types, defs, error, utils::generate_hash_usize};
use ellie_tokenizer::{
    syntax::items::condition::{Condition, ConditionType},
    tokenizer::{ConditionPageType, PageType},
};

impl super::Processor for Condition {
    fn process(
        &self,
        parser: &mut super::Parser,
        page_idx: usize,
        processed_page_idx: usize,
        page_hash: usize,
    ) -> bool {
        let path = parser.pages.nth(page_idx).unwrap().path.clone();
        //(processed_page_id, conditionType)
        let mut chains: Vec<(usize, Types)> = Vec::new();
        let condition_hash = generate_hash_usize();
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
                    false,
                    false,
                    Some(self.pos),
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

                        let condition_type = match condition_type {
                            Some(e) => e,
                            None => {
                                return false;
                            }
                        };

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
            let inner_page_id: usize = ellie_core::utils::generate_hash_usize();
            let mut dependencies = vec![ellie_tokenizer::tokenizer::Dependency {
                hash: page.hash.clone(),
                processed: false,
                module: None,
                deep_link: Some(page.hash.clone()),
                public: false,
            }];
            dependencies.extend(page.dependencies);

            let inner = ellie_tokenizer::tokenizer::Page {
                hash: inner_page_id,
                inner: Some(page.hash),
                path: page.path.clone(),
                page_type: PageType::ConditionBody(ConditionPageType {
                    page_hash,
                    condition_hash,
                    chain_type: chain.rtype,
                    keyword_pos: chain.keyword_pos,
                }),
                items: chain.code.clone(),
                dependents: vec![],
                dependencies,
                ..Default::default()
            };
            parser.pages.push_page(inner);
            chains.push((inner_page_id, condition_type));
        }

        let processed_page = parser.processed_pages.nth_mut(processed_page_idx).unwrap();

        processed_page
            .items
            .push(ellie_core::definite::items::Collecting::Condition(
                ellie_core::definite::items::condition::Condition {
                    hash: condition_hash,
                    returns: None,
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
