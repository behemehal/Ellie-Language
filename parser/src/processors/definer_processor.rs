use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, vec};
use ellie_core::{definite::definers, definite::definers::DefinerCollecting, error};
use ellie_tokenizer::syntax::items::definers::DefinerTypes;

pub fn process(
    from: DefinerTypes,
    parser: &mut super::Parser,
    page_id: u64,
    ignore_hash: Option<String>,
) -> Result<DefinerCollecting, Vec<error::Error>> {
    let mut errors = vec![];
    let mut found = DefinerCollecting::Dynamic;
    match from.clone() {
        DefinerTypes::Cloak(_) => todo!(),
        DefinerTypes::Array(_) => todo!(),
        DefinerTypes::Collective(e) => {
            let mut key = DefinerCollecting::Dynamic;
            let mut value = DefinerCollecting::Dynamic;
            match process(*e.key, parser, page_id, ignore_hash.clone()) {
                Ok(e) => {
                    key = e;
                }
                Err(e) => errors.extend(e),
            }
            match process(*e.value, parser, page_id, ignore_hash.clone()) {
                Ok(e) => {
                    value = e;
                }
                Err(e) => errors.extend(e),
            }
            found = DefinerCollecting::Collective(definers::CollectiveType {
                key: Box::new(key),
                value: Box::new(value),
            });
        }
        DefinerTypes::Vector(_) => todo!(),
        DefinerTypes::Nullable(_) => todo!(),
        DefinerTypes::ParentGeneric(generic) => {
            let deep_search_result = parser.deep_search(
                page_id,
                generic.parent.clone(),
                ignore_hash.clone(),
                vec![],
                0,
            );

            if deep_search_result.found {
                match deep_search_result.found_item {
                    crate::parser::DeepSearchItems::Class(e) => {
                        if e.generic_definings.len() != generic.generics.len() {
                            let mut err = error::error_list::ERROR_S44.clone().build_with_path(
                                vec![
                                    error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: e.generic_definings.len().to_string(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_owned(),
                                        value: generic.generics.len().to_string(),
                                    },
                                ],
                                "def_pr_0x67".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                generic.pos,
                            );
                            err.reference_block =
                                Some((e.name_pos, deep_search_result.found_page.path));
                            errors.push(err);
                        } else {
                            let mut resolved_generics = Vec::new();
                            for i in generic.generics {
                                match process(i.value, parser, page_id, ignore_hash.clone()) {
                                    Ok(e) => resolved_generics.push(definers::GenericParameter {
                                        value: e,
                                        pos: i.pos,
                                    }),
                                    Err(e) => errors.extend(e),
                                }
                            }
                            found = DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                                rtype: e.name,
                                hash: e.hash,
                                generics: resolved_generics,
                                parent_pos: generic.parent_pos,
                            });
                        }
                    }
                    crate::parser::DeepSearchItems::BrokenPageGraph => todo!(),
                    crate::parser::DeepSearchItems::MixUp(_) => todo!(),
                    crate::parser::DeepSearchItems::None => {
                        errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: generic.parent,
                            }],
                            "def_pr_0x71".to_owned(),
                            parser.find_page(page_id).unwrap().path.clone(),
                            generic.parent_pos,
                        ));
                    }
                    _ => {
                        errors.push(error::error_list::ERROR_S45.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: generic.parent,
                            }],
                            "def_pr_0x48".to_owned(),
                            parser.find_page(page_id).unwrap().path.clone(),
                            generic.parent_pos,
                        ));
                    }
                }
            } else {
                errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: generic.parent,
                    }],
                    "def_pr_0x94".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    generic.parent_pos,
                ));
            }
        }
        DefinerTypes::Generic(generic) => {
            let deep_search_result =
                parser.deep_search(page_id, generic.rtype.clone(), ignore_hash, vec![], 0);

            if deep_search_result.found {
                match deep_search_result.found_item {
                    crate::parser::DeepSearchItems::Class(e) => {
                        if e.generic_definings.len() == 0 {
                            found = DefinerCollecting::Generic(definers::GenericType {
                                rtype: e.name,
                                hash: e.hash,
                                pos: e.pos,
                            });
                        } else {
                            errors.push(error::error_list::ERROR_S44.clone().build_with_path(
                                vec![
                                    error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: e.generic_definings.len().to_string(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_owned(),
                                        value: "0".to_string(),
                                    },
                                ],
                                "def_pr_0x121".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                generic.pos,
                            ));
                        }
                    }
                    crate::parser::DeepSearchItems::BrokenPageGraph => todo!(),
                    crate::parser::DeepSearchItems::MixUp(_) => todo!(),
                    crate::parser::DeepSearchItems::None => {
                        errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: generic.rtype,
                            }],
                            "def_pr_0x134".to_owned(),
                            parser.find_page(page_id).unwrap().path.clone(),
                            generic.pos,
                        ));
                    }
                    _ => {
                        errors.push(error::error_list::ERROR_S45.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: generic.rtype,
                            }],
                            "def_pr_0x145".to_owned(),
                            parser.find_page(page_id).unwrap().path.clone(),
                            generic.pos,
                        ));
                    }
                }
            } else {
                errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: generic.rtype,
                    }],
                    "def_pr_0x159".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    generic.pos,
                ));
            }
        }
        DefinerTypes::Function(_) => todo!(),
        DefinerTypes::Dynamic => todo!(),
    }
    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(found)
    }
}
