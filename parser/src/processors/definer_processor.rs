use alloc::boxed::Box;
use alloc::format;
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
        DefinerTypes::Cloak(e) => {
            let deep_search_result =
                parser.deep_search(page_id, "cloak".to_string(), ignore_hash.clone(), vec![], 0);

            if deep_search_result.found {
                match deep_search_result.found_item {
                    crate::parser::DeepSearchItems::Class(cloak_class) => {
                        found = DefinerCollecting::Generic(definers::GenericType {
                            hash: cloak_class.hash,
                            rtype: "cloak".to_string(),
                            pos: cloak_class.pos,
                        });
                    }
                    _ => match deep_search_result.found_pos {
                        Some(ref_pos) => {
                            let mut error = error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            );
                            error.reference_message = "Defined here".to_string();
                            error.reference_block =
                                Some((ref_pos, deep_search_result.found_page.path));
                            errors.push(error);
                        }
                        None => {
                            errors.push(error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            ));
                        }
                    },
                }
            } else {
                errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "vector".to_string(),
                    }],
                    "def_pr_0x57".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    e.pos,
                ));
            }
        }
        DefinerTypes::Array(e) => {
            let deep_search_result =
                parser.deep_search(page_id, "array".to_string(), ignore_hash.clone(), vec![], 0);

            if deep_search_result.found {
                match deep_search_result.found_item {
                    crate::parser::DeepSearchItems::Class(array_class) => {
                        match process(*e.rtype.clone(), parser, page_id, ignore_hash) {
                            Ok(inner_type) => {
                                if array_class.generic_definings.len() != 1 {
                                    let mut error =
                                        error::error_list::ERROR_S44.clone().build_with_path(
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: array_class
                                                        .generic_definings
                                                        .len()
                                                        .to_string(),
                                                },
                                                error::ErrorBuildField {
                                                    key: "token2".to_string(),
                                                    value: 1.to_string(),
                                                },
                                            ],
                                            "def_pr_0x57".to_owned(),
                                            parser.find_page(page_id).unwrap().path.clone(),
                                            e.rtype.get_pos(),
                                        );
                                    error.reference_message =
                                        "Does not have required generic parameters".to_string();
                                    error.reference_block =
                                        Some((array_class.pos, deep_search_result.found_page.path));
                                    errors.push(error);
                                } else {
                                    found = DefinerCollecting::ParentGeneric(
                                        definers::ParentGenericType {
                                            parent_pos: array_class.pos,
                                            generics: vec![definers::GenericParameter {
                                                value: inner_type,
                                                pos: deep_search_result
                                                    .found_pos
                                                    .unwrap_or(ellie_core::defs::Cursor::default()),
                                            }],
                                            hash: array_class.hash,
                                            rtype: "vector".to_string(),
                                        },
                                    );
                                }
                            }
                            Err(e) => errors.extend(e),
                        }
                    }
                    _ => match deep_search_result.found_pos {
                        Some(ref_pos) => {
                            let mut error = error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            );
                            error.reference_message = "Defined here".to_string();
                            error.reference_block =
                                Some((ref_pos, deep_search_result.found_page.path));
                            errors.push(error);
                        }
                        None => {
                            errors.push(error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            ));
                        }
                    },
                }
            } else {
                errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "vector".to_string(),
                    }],
                    "def_pr_0x57".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    e.pos,
                ));
            }
        }
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
            let deep_search_result = parser.deep_search(
                page_id,
                "nullAble".to_string(),
                ignore_hash.clone(),
                vec![],
                0,
            );

            if deep_search_result.found && errors.is_empty() {
                match deep_search_result.found_item {
                    crate::parser::DeepSearchItems::Class(nullable_class) => {
                        found = DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                            parent_pos: nullable_class.pos,
                            generics: vec![
                                definers::GenericParameter {
                                    value: key,
                                    pos: deep_search_result
                                        .found_pos
                                        .unwrap_or(ellie_core::defs::Cursor::default()),
                                },
                                definers::GenericParameter {
                                    value,
                                    pos: deep_search_result
                                        .found_pos
                                        .unwrap_or(ellie_core::defs::Cursor::default()),
                                },
                            ],
                            hash: nullable_class.hash,
                            rtype: "vector".to_string(),
                        });
                    }
                    _ => match deep_search_result.found_pos {
                        Some(ref_pos) => {
                            let mut error = error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            );
                            error.reference_message = "Defined here".to_string();
                            error.reference_block =
                                Some((ref_pos, deep_search_result.found_page.path));
                            errors.push(error);
                        }
                        None => {
                            errors.push(error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            ));
                        }
                    },
                }
            } else {
                errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "vector".to_string(),
                    }],
                    "def_pr_0x57".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    e.pos,
                ));
            }
        }
        DefinerTypes::Vector(e) => {
            let deep_search_result = parser.deep_search(
                page_id,
                "vector".to_string(),
                ignore_hash.clone(),
                vec![],
                0,
            );

            if deep_search_result.found {
                match deep_search_result.found_item {
                    crate::parser::DeepSearchItems::Class(vector_class) => {
                        match process(*e.rtype.clone(), parser, page_id, ignore_hash) {
                            Ok(inner_type) => {
                                if vector_class.generic_definings.len() != 1 {
                                    let mut error =
                                        error::error_list::ERROR_S44.clone().build_with_path(
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: vector_class
                                                        .generic_definings
                                                        .len()
                                                        .to_string(),
                                                },
                                                error::ErrorBuildField {
                                                    key: "token2".to_string(),
                                                    value: 1.to_string(),
                                                },
                                            ],
                                            "def_pr_0x57".to_owned(),
                                            parser.find_page(page_id).unwrap().path.clone(),
                                            e.rtype.get_pos(),
                                        );
                                    error.reference_message =
                                        "Does not have required generic parameters".to_string();
                                    error.reference_block = Some((
                                        vector_class.pos,
                                        deep_search_result.found_page.path,
                                    ));
                                    errors.push(error);
                                } else {
                                    found = DefinerCollecting::ParentGeneric(
                                        definers::ParentGenericType {
                                            parent_pos: vector_class.pos,
                                            generics: vec![definers::GenericParameter {
                                                value: inner_type,
                                                pos: deep_search_result
                                                    .found_pos
                                                    .unwrap_or(ellie_core::defs::Cursor::default()),
                                            }],
                                            hash: vector_class.hash,
                                            rtype: "vector".to_string(),
                                        },
                                    );
                                }
                            }
                            Err(e) => errors.extend(e),
                        }
                    }
                    _ => match deep_search_result.found_pos {
                        Some(ref_pos) => {
                            let mut error = error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            );
                            error.reference_message = "Defined here".to_string();
                            error.reference_block =
                                Some((ref_pos, deep_search_result.found_page.path));
                            errors.push(error);
                        }
                        None => {
                            errors.push(error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            ));
                        }
                    },
                }
            } else {
                errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "vector".to_string(),
                    }],
                    "def_pr_0x57".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    e.pos,
                ));
            }
        }
        DefinerTypes::Nullable(e) => {
            let deep_search_result = parser.deep_search(
                page_id,
                "nullAble".to_string(),
                ignore_hash.clone(),
                vec![],
                0,
            );

            if deep_search_result.found {
                match deep_search_result.found_item {
                    crate::parser::DeepSearchItems::Class(nullable_class) => {
                        match process(*e.rtype.clone(), parser, page_id, ignore_hash) {
                            Ok(inner_type) => {
                                if nullable_class.generic_definings.len() != 1 {
                                    let mut error =
                                        error::error_list::ERROR_S44.clone().build_with_path(
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: nullable_class
                                                        .generic_definings
                                                        .len()
                                                        .to_string(),
                                                },
                                                error::ErrorBuildField {
                                                    key: "token2".to_string(),
                                                    value: 1.to_string(),
                                                },
                                            ],
                                            "def_pr_0x57".to_owned(),
                                            parser.find_page(page_id).unwrap().path.clone(),
                                            e.rtype.get_pos(),
                                        );
                                    error.reference_message =
                                        "Does not have required generic parameters".to_string();
                                    error.reference_block = Some((
                                        nullable_class.pos,
                                        deep_search_result.found_page.path,
                                    ));
                                    errors.push(error);
                                } else {
                                    found = DefinerCollecting::ParentGeneric(
                                        definers::ParentGenericType {
                                            parent_pos: nullable_class.pos,
                                            generics: vec![definers::GenericParameter {
                                                value: inner_type,
                                                pos: deep_search_result
                                                    .found_pos
                                                    .unwrap_or(ellie_core::defs::Cursor::default()),
                                            }],
                                            hash: nullable_class.hash,
                                            rtype: "vector".to_string(),
                                        },
                                    );
                                }
                            }
                            Err(e) => errors.extend(e),
                        }
                    }
                    _ => match deep_search_result.found_pos {
                        Some(ref_pos) => {
                            let mut error = error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            );
                            error.reference_message = "Defined here".to_string();
                            error.reference_block =
                                Some((ref_pos, deep_search_result.found_page.path));
                            errors.push(error);
                        }
                        None => {
                            errors.push(error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            ));
                        }
                    },
                }
            } else {
                errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "vector".to_string(),
                    }],
                    "def_pr_0x57".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    e.pos,
                ));
            }
        }
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
            let deep_search_result = parser.deep_search(
                page_id,
                generic.rtype.clone(),
                ignore_hash.clone(),
                vec![],
                0,
            );

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
                    crate::parser::DeepSearchItems::GenericItem(generic) => {
                        found = DefinerCollecting::Generic(definers::GenericType {
                            rtype: generic.generic_name.clone(),
                            hash: format!("<virtual={}>", generic.generic_name),
                            pos: generic.pos,
                        });
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
        DefinerTypes::Function(e) => {
            let deep_search_result =
                parser.deep_search(page_id, "cloak".to_string(), ignore_hash.clone(), vec![], 0);

            if deep_search_result.found {
                match deep_search_result.found_item {
                    crate::parser::DeepSearchItems::Class(cloak_class) => {
                        found = DefinerCollecting::Generic(definers::GenericType {
                            hash: cloak_class.hash,
                            rtype: "function".to_string(),
                            pos: cloak_class.pos,
                        });
                    }
                    _ => match deep_search_result.found_pos {
                        Some(ref_pos) => {
                            let mut error = error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            );
                            error.reference_message = "Defined here".to_string();
                            error.reference_block =
                                Some((ref_pos, deep_search_result.found_page.path));
                            errors.push(error);
                        }
                        None => {
                            errors.push(error::error_list::ERROR_S45.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "vector".to_string(),
                                }],
                                "def_pr_0x57".to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            ));
                        }
                    },
                }
            } else {
                errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "vector".to_string(),
                    }],
                    "def_pr_0x57".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    e.pos,
                ));
            }
        }
        DefinerTypes::Dynamic => {
            panic!("Unexpected behaviour")
        }
    }
    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(found)
    }
}
