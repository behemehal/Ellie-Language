use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use ellie_core::{
    definite::{items::Collecting, types::ellie_char},
    defs, error,
};
use ellie_tokenizer::tokenizer::Dependency;

use crate::parser::ProcessedPage;

/*
    This folder contains parser extensions for deep search.
*/

#[derive(Debug)]
pub enum DeepTypeResult {
    Integer(ellie_core::definite::types::integer::IntegerType),
    Float(ellie_core::definite::types::float::FloatType),
    Bool(ellie_core::definite::types::bool::BoolType),
    String(ellie_core::definite::types::string::StringType),
    Char(ellie_core::definite::types::ellie_char::CharType),
    Collective(ellie_core::definite::types::collective::CollectiveType),
    Operator(ellie_core::definite::types::operator::OperatorType),
    Cloak(ellie_core::definite::types::cloak::CloakType),
    Array(ellie_core::definite::types::array::ArrayType),
    Vector(ellie_core::definite::types::vector::VectorType),
    ClassCall(ellie_core::definite::types::class_call::ClassCall),
    FunctionCall(ellie_core::definite::types::function_call::FunctionCall),
    Void,
    Null,
    NotFound,
}

fn iterate_deep_type(
    parser: &mut crate::parser::Parser,
    page_id: u64,
    rtype: ellie_core::definite::types::Types,
) -> DeepTypeResult {
    match rtype.clone() {
        ellie_core::definite::types::Types::Integer(integer) => DeepTypeResult::Integer(integer),
        ellie_core::definite::types::Types::Float(float) => DeepTypeResult::Float(float),
        ellie_core::definite::types::Types::String(string) => DeepTypeResult::String(string),
        ellie_core::definite::types::Types::Char(char) => DeepTypeResult::Char(char),
        ellie_core::definite::types::Types::Collective(collective) => {
            DeepTypeResult::Collective(collective)
        }
        ellie_core::definite::types::Types::Reference(_) => todo!(),
        ellie_core::definite::types::Types::BraceReference(_) => todo!(),
        ellie_core::definite::types::Types::Operator(_) => todo!(),
        ellie_core::definite::types::Types::Cloak(cloak) => {
            if cloak.collective.len() == 1 {
                iterate_deep_type(
                    parser,
                    page_id,
                    cloak.collective.last().unwrap().clone().value,
                )
            } else {
                DeepTypeResult::Cloak(cloak)
            }
        }
        ellie_core::definite::types::Types::Array(array) => DeepTypeResult::Array(array),
        ellie_core::definite::types::Types::Vector(vector) => DeepTypeResult::Vector(vector),
        ellie_core::definite::types::Types::ClassCall(class_call) => {
            DeepTypeResult::ClassCall(class_call)
        }
        ellie_core::definite::types::Types::FunctionCall(_) => todo!(),
        ellie_core::definite::types::Types::NullResolver(_) => todo!(),
        ellie_core::definite::types::Types::Negative(_) => todo!(),
        ellie_core::definite::types::Types::VariableType(variable) => {
            let hash_deep_search =
                deep_search_hash(parser, page_id, variable.reference, vec![], true, 0);
            if hash_deep_search.found {
                match hash_deep_search.found_item {
                    ProcessedDeepSearchItems::Class(e) => {
                        //This is the class elements defining class. We're virtually building it
                        //See lib/class.ei
                        let class_class =
                            deep_search(parser, page_id, "class".to_owned(), None, vec![], 0);
                        if class_class.found {
                            if let ProcessedDeepSearchItems::Class(e) = class_class.found_item {
                                DeepTypeResult::ClassCall(
                                    ellie_core::definite::types::class_call::ClassCall {
                                        target: Box::new(ellie_core::definite::types::Types::VariableType(
                                            ellie_core::definite::types::variable::VariableType {
                                                value: "class".to_owned(),
                                                reference: e.hash,
                                                pos: ellie_core::defs::Cursor::default(),
                                            },
                                        )),
                                        params: vec![],
                                        keyword_pos: ellie_core::defs::Cursor::default(),
                                        target_pos: ellie_core::defs::Cursor::default(),
                                        generic_parameters: vec![],
                                        pos: ellie_core::defs::Cursor::default(),
                                    },
                                )
                            } else {
                                unreachable!("Ellie must ensure that class is a class, and no one can replace it");
                            }
                        } else {
                            let path = parser.find_page(page_id).unwrap().path.clone();
                            parser.informations.push(
                                &error::error_list::ERROR_S6.clone().build_with_path(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: "class".to_string(),
                                    }],
                                    file!().to_string(),
                                    path,
                                    e.pos,
                                ),
                            );
                            DeepTypeResult::NotFound
                        }
                    }
                    ProcessedDeepSearchItems::Variable(e) => {
                        iterate_deep_type(parser, page_id, e.value)
                    }
                    ProcessedDeepSearchItems::Function(_) => todo!(),
                    ProcessedDeepSearchItems::ImportReference(_) => todo!(),
                    ProcessedDeepSearchItems::None => todo!(),
                }
            } else {
                unreachable!(
                    "VariableName: {}, Hash: {}, {:#?}",
                    variable.value, variable.reference, rtype
                );
            }
        }
        ellie_core::definite::types::Types::AsKeyword(_) => todo!(),

        //ellie_core::definite::types::Types::ArrowFunction(_) => todo!(),
        ellie_core::definite::types::Types::Bool(_) => unreachable!(),
        ellie_core::definite::types::Types::Void => todo!(),
        ellie_core::definite::types::Types::Null => todo!(),
    }
}

pub fn resolve_deep_type(
    parser: &mut crate::parser::Parser,
    page_id: u64,
    rtype: ellie_core::definite::types::Types,
) -> DeepTypeResult {
    iterate_deep_type(parser, page_id, rtype)
}

#[derive(Debug, Clone)]
pub enum ProcessedDeepSearchItems {
    Class(ellie_core::definite::items::class::Class),
    Variable(ellie_core::definite::items::variable::Variable),
    Function(ellie_core::definite::items::function::Function),
    ImportReference(ellie_core::definite::items::import::Import),
    //SelfItem(ellie_core::definite::items::::SelfItem),
    //GenericItem(ellie_core::definite::items::generic::Generic),
    //FunctionParameter(ellie_core::definite::items::function::FunctionParameter),
    //ConstructorParameter(ellie_tokenizer::syntax::items::constructor_parameter::ConstructorParameter),
    //MixUp(Vec<(String, String)>),
    //BrokenPageGraph,
    None,
}

#[derive(Debug, Clone)]
pub struct ProcessedDeepSearchResult {
    pub found: bool,
    pub found_item: ProcessedDeepSearchItems,
    pub found_pos: Option<defs::Cursor>,
    pub found_page: ProcessedPage,
}

pub fn deep_search_hash(
    parser: &mut crate::parser::Parser,
    target_page: u64,
    target_hash: u64,
    searched: Vec<u64>,
    processed_only: bool,
    _level: u32,
) -> ProcessedDeepSearchResult {
    let mut level = _level;
    let mut found = false;
    let mut found_type = ProcessedDeepSearchItems::None;
    let mut found_pos = None;
    let mut found_page = ProcessedPage::default();
    let has_mixup = false;
    let mut inner_page = None;
    let mut searched: Vec<u64> = searched;
    //let mixup_hashes: Vec<(String, String)> = Vec::new();
    let mut self_dependencies = vec![];

    match parser.find_processed_page(target_page) {
        Some(page) => {
            self_dependencies.push(Dependency {
                hash: target_page,
                processed: true,
                ..Default::default()
            });
            self_dependencies.extend(
                page.dependencies
                    .iter()
                    .filter_map(|x| if x.processed && x.module.is_none() { Some(x.clone()) } else { None })
                    .collect::<Vec<Dependency>>(),
            );
            inner_page = page.inner;
        }
        None => (),
    };

    if !searched.contains(&target_page) {
        for dep in self_dependencies {
            searched.push(target_page);
            match parser.find_processed_page(dep.hash) {
                Some(page) => {
                    for item in &page.items {
                        match item.clone() {
                            Collecting::Variable(e) => {
                                if e.hash == target_hash && (e.public || level == 0) {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Variable(e);
                                }
                            }
                            Collecting::Function(e) => {
                                if e.hash == target_hash && (e.public || level == 0) {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Function(e);
                                }
                            }
                            Collecting::Import(e) => {
                                if e.reference != ""
                                    && e.hash == target_hash
                                    && (e.public
                                        || level == 0
                                        || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::ImportReference(e);
                                }
                            }
                            Collecting::Class(e) => {
                                if e.hash == target_hash
                                    && (e.public
                                        || level == 0
                                        || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Class(e);
                                }
                            }
                            _ => (),
                        }
                    }
                }
                None => {
                    panic!("Broken Page structure; Failed to find page {}", dep.hash);
                }
            }
            level += 1;
        }
    }

    if has_mixup {
        unreachable!();
        /*
        ProcessedDeepSearchResult {
            found: true,
            found_pos,
            found_item: DeepSearchItems::MixUp(mixup_hashes),
            found_page,
        }
        */
    } else if found {
        ProcessedDeepSearchResult {
            found: true,
            found_pos,
            found_item: found_type,
            found_page,
        }
    } else {
        ProcessedDeepSearchResult {
            found: false,
            found_pos,
            found_item: ProcessedDeepSearchItems::None,
            found_page,
        }
    }
}

pub fn deep_search(
    parser: &mut crate::parser::Parser,
    target_page: u64,
    name: String,
    ignore_hash: Option<u64>,
    searched: Vec<u64>,
    _level: u32,
) -> ProcessedDeepSearchResult {
    let mut level = _level;
    let mut found = false;
    let mut found_type = ProcessedDeepSearchItems::None;
    let mut found_pos = None;
    let mut found_page = ProcessedPage::default();
    let has_mixup = false;
    let mut inner_page = None;
    let mut searched: Vec<u64> = searched;
    let mixup_hashes: Vec<(String, String)> = Vec::new();
    let mut self_dependencies = vec![Dependency {
        hash: target_page,
        ..Default::default()
    }];

    match parser.find_processed_page(target_page) {
        Some(page) => {
            self_dependencies.push(Dependency {
                hash: target_page,
                processed: true,
                ..Default::default()
            });
            self_dependencies.extend(
                page.dependencies
                    .iter()
                    .filter_map(|x| if x.processed && x.module.is_none() { Some(x.clone()) } else { None })
                    .collect::<Vec<Dependency>>(),
            );
            inner_page = page.inner;
        }
        None => (),
    };

    if !searched.contains(&target_page) {
        for dep in self_dependencies {
            searched.push(target_page);
            match parser.find_processed_page(dep.hash) {
                Some(page) => {
                    let page = page.clone();
                    for item in page.items.iter() {
                        match item.clone() {
                            Collecting::Variable(e) => {
                                if e.name == name
                                    && (e.public || level == 0)
                                    && (ignore_hash.is_none()
                                        || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Variable(e);
                                }
                            }
                            Collecting::Function(e) => {
                                if e.name == name
                                    && (e.public || level == 0)
                                    && (ignore_hash.is_none()
                                        || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Function(e);
                                }
                            }
                            Collecting::Import(e) => {
                                if e.reference != ""
                                    && e.reference == name
                                    && (e.public
                                        || level == 0
                                        || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                    && (ignore_hash.is_none()
                                        || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::ImportReference(e);
                                }
                            }
                            Collecting::Class(e) => {
                                if e.name == name
                                    && (e.public
                                        || level == 0
                                        || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                    && (ignore_hash.is_none()
                                        || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Class(e);
                                }
                            }
                            _ => (),
                        }
                    }
                }
                None => {
                    panic!("Broken Page structure; Failed to find page {}", dep.hash);
                }
            }

            level += 1;
        }
    }

    if has_mixup {
        unreachable!();
    } else if found {
        ProcessedDeepSearchResult {
            found: true,
            found_pos,
            found_item: found_type,
            found_page,
        }
    } else {
        ProcessedDeepSearchResult {
            found: false,
            found_pos,
            found_item: ProcessedDeepSearchItems::None,
            found_page,
        }
    }
}
