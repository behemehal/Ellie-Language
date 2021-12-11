use alloc::string::String;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, vec};
use ellie_core::{
    definite::definers::DefinerCollecting,
    definite::{definers, items},
    defs, error,
};
use ellie_tokenizer::syntax::items::definers::DefinerTypes;

pub fn process(
    from: DefinerTypes,
    parser: &mut super::Parser,
    page_id: u64,
    pos: defs::Cursor,
) -> Result<DefinerCollecting, Vec<error::Error>> {
    let mut errors = vec![];
    let mut found = DefinerCollecting::Dynamic;
    match from.clone() {
        DefinerTypes::Cloak(_) => todo!(),
        DefinerTypes::Array(_) => todo!(),
        DefinerTypes::Collective(_) => todo!(),
        DefinerTypes::Vector(_) => todo!(),
        DefinerTypes::Nullable(_) => todo!(),
        DefinerTypes::ParentGeneric(_) => todo!(),
        DefinerTypes::Generic(generic) => {
            match parser.deep_search(page_id, generic.rtype.clone(), None, vec![], 0) {
                Some(result) => match result {
                    crate::parser::DeepSearchResult::Class(e) => {
                        if e.generic_definings.len() == 0 {
                            found = DefinerCollecting::Generic(definers::GenericType {
                                rtype: e.name,
                                hash: e.hash,
                            });
                        } else {
                            found = DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                                rtype: e.name,
                                hash: e.hash,
                                generics: e.generic_definings.into_iter().map(|x| {
                                    definers::GenericParameter {
                                        value: DefinerCollecting::Generic(definers::GenericType {
                                            rtype: x.name,
                                            hash: String::new(),
                                        }),
                                        pos: x.pos,
                                    }
                                }).collect::<Vec<_>>(),
                            });
                        }
                    }
                    crate::parser::DeepSearchResult::Variable(_) => todo!(),
                    crate::parser::DeepSearchResult::Function(_) => todo!(),
                    crate::parser::DeepSearchResult::ImportReference(_) => {
                        todo!("{:#?} {:#?}", generic, result)
                    }
                    crate::parser::DeepSearchResult::BrokenPageGraph => todo!(),
                    crate::parser::DeepSearchResult::MixUp(_) => todo!(),
                    crate::parser::DeepSearchResult::None => {
                        errors.push(error::errorList::error_s6.clone().build(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: generic.rtype,
                            }],
                            "def_pr_0x39".to_owned(),
                            pos,
                        ));
                    }
                },
                None => {
                    errors.push(error::errorList::error_s6.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: generic.rtype,
                        }],
                        "def_pr_0x50".to_owned(),
                        pos,
                    ));
                }
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
