use core::panic;

use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use ellie_core::{
    definite::{
        definers,
        items::Collecting,
        types::{brace_reference, Types, null_resolver},
    },
    defs, error,
};
use ellie_tokenizer::tokenizer::Dependency;
use enum_as_inner::EnumAsInner;

use crate::parser::{DeepSearchItems, DeepSearchResult, Parser, ProcessedPage};

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
    BraceReference(ellie_core::definite::types::brace_reference::BraceReferenceType),
    Void,
    Null,
    NotFound,
}

fn iterate_deep_type(parser: &mut Parser, page_id: u64, rtype: Types) -> DeepTypeResult {
    match rtype.clone() {
        Types::Integer(integer) => DeepTypeResult::Integer(integer),
        Types::Float(float) => DeepTypeResult::Float(float),
        Types::String(string) => DeepTypeResult::String(string),
        Types::Char(char) => DeepTypeResult::Char(char),
        Types::Collective(collective) => DeepTypeResult::Collective(collective),
        Types::Reference(_) => todo!(),
        Types::BraceReference(e) => {
            let resolved_reference = resolve_deep_type(parser, page_id, *e.reference.clone());
            let resolved_index = resolve_deep_type(parser, page_id, *e.value);
            DeepTypeResult::BraceReference(
                ellie_core::definite::types::brace_reference::BraceReferenceType {
                    reference: Box::new(match resolved_reference {
                        DeepTypeResult::Integer(e) => Types::Integer(e),
                        DeepTypeResult::Float(e) => Types::Float(e),
                        DeepTypeResult::Bool(e) => Types::Bool(e),
                        DeepTypeResult::String(e) => Types::String(e),
                        DeepTypeResult::Char(e) => Types::Char(e),
                        DeepTypeResult::Collective(e) => Types::Collective(e),
                        DeepTypeResult::Operator(e) => Types::Operator(e),
                        DeepTypeResult::Cloak(e) => Types::Cloak(e),
                        DeepTypeResult::Array(e) => Types::Array(e),
                        DeepTypeResult::Vector(e) => Types::Vector(e),
                        DeepTypeResult::ClassCall(e) => Types::ClassCall(e),
                        DeepTypeResult::FunctionCall(e) => Types::FunctionCall(e),
                        DeepTypeResult::BraceReference(e) => Types::BraceReference(e),
                        DeepTypeResult::Void => unreachable!(),
                        DeepTypeResult::Null => unreachable!(),
                        DeepTypeResult::NotFound => unreachable!("cannot find reference: {:?}", *e.reference),
                    }),
                    reference_pos: e.reference_pos,
                    brace_pos: e.brace_pos,
                    value: Box::new(match resolved_index {
                        DeepTypeResult::Integer(e) => Types::Integer(e),
                        DeepTypeResult::Float(e) => Types::Float(e),
                        DeepTypeResult::Bool(e) => Types::Bool(e),
                        DeepTypeResult::String(e) => Types::String(e),
                        DeepTypeResult::Char(e) => Types::Char(e),
                        DeepTypeResult::Collective(e) => Types::Collective(e),
                        DeepTypeResult::Operator(e) => Types::Operator(e),
                        DeepTypeResult::Cloak(e) => Types::Cloak(e),
                        DeepTypeResult::Array(e) => Types::Array(e),
                        DeepTypeResult::Vector(e) => Types::Vector(e),
                        DeepTypeResult::ClassCall(e) => Types::ClassCall(e),
                        DeepTypeResult::FunctionCall(e) => Types::FunctionCall(e),
                        DeepTypeResult::BraceReference(e) => Types::BraceReference(e),
                        DeepTypeResult::Void => unreachable!(),
                        DeepTypeResult::Null => unreachable!(),
                        DeepTypeResult::NotFound => unreachable!(),
                    }),
                    pos: e.pos,
                },
            )
        }
        Types::Operator(_) => todo!(),
        Types::Cloak(cloak) => {
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
        Types::Array(array) => {
            let mut collective = vec![];
            for i in array.collective {
                let resolved_collective = resolve_deep_type(parser, page_id, i.value);
                match resolved_collective {
                    DeepTypeResult::Integer(integer_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Integer(integer_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Float(float_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Float(float_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Bool(bool_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Bool(bool_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::String(string_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::String(string_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Char(char_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Char(char_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Collective(collective_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Collective(collective_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Operator(operator_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Operator(operator_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Cloak(cloak_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Cloak(cloak_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Array(array_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Array(array_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Vector(vector_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Vector(vector_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::ClassCall(class_call) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::ClassCall(class_call),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::FunctionCall(function_call) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::FunctionCall(function_call),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Void => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Void,
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Null => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Null,
                            location: i.location,
                        });
                    }
                    DeepTypeResult::NotFound => {
                        return DeepTypeResult::NotFound;
                    }
                    DeepTypeResult::BraceReference(brace_reference) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::BraceReference(brace_reference),
                            location: i.location,
                        });
                    }
                }
            }
            DeepTypeResult::Array(ellie_core::definite::types::array::ArrayType {
                collective,
                pos: array.pos,
            })
        }
        Types::Vector(vector) => DeepTypeResult::Vector(vector),
        Types::ClassCall(class_call) => DeepTypeResult::ClassCall(class_call),
        Types::FunctionCall(_) => todo!(),
        Types::NullResolver(null_resolver) => {
            let from_type = resolve_type(*null_resolver.target.clone(), page_id, parser);
            panic!("{:?}", from_type);
        },
        Types::Negative(_) => todo!(),
        Types::VariableType(variable) => {
            let hash_deep_search = deep_search_hash(parser, page_id, variable.reference, vec![], 0);
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
                                        target: Box::new(Types::VariableType(
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
                let path = parser.find_page(page_id).unwrap().path.clone();
                parser
                    .informations
                    .push(&error::error_list::ERROR_S6.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "class".to_string(),
                        }],
                        file!().to_string(),
                        path,
                        variable.pos,
                    ));
                DeepTypeResult::NotFound
            }
        }
        Types::AsKeyword(as_keyword) => {
            let from_type = resolve_type(*as_keyword.target.clone(), page_id, parser);
            match as_keyword.rtype.clone() {
                definers::DefinerCollecting::Generic(e) => {
                    let targeted_type = find_type(e.rtype, page_id, parser);

                    match targeted_type.clone() {
                        Some(target_type_gen) => match from_type.clone() {
                            definers::DefinerCollecting::ParentGeneric(pg) => {

                                //TODO Implement more types
                                if pg.rtype == "nullAble" {
                                    DeepTypeResult::Integer(
                                        ellie_core::definite::types::integer::IntegerType {
                                            value: ellie_core::definite::types::integer::IntegerSize::U8(0),
                                            rtype: ellie_core::definite::types::integer::IntegerTypes::U8,
                                            pos: defs::Cursor::default(),
                                        },
                                    )
                                } else {
                                    let path = parser.find_page(page_id).unwrap().path.clone();
                                    parser.informations.push(
                                        &error::error_list::ERROR_S50.clone().build_with_path(
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "target".to_string(),
                                                    value: from_type.to_string(),
                                                },
                                                error::ErrorBuildField {
                                                    key: "token".to_string(),
                                                    value: target_type_gen.rtype,
                                                },
                                            ],
                                            file!().to_string(),
                                            path,
                                            as_keyword.pos,
                                        ),
                                    );
                                    DeepTypeResult::NotFound
                                }
                            }
                            _ => {
                                let path = parser.find_page(page_id).unwrap().path.clone();
                                parser.informations.push(
                                    &error::error_list::ERROR_S50.clone().build_with_path(
                                        vec![
                                            error::ErrorBuildField {
                                                key: "target".to_string(),
                                                value: from_type.to_string(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: target_type_gen.rtype,
                                            },
                                        ],
                                        file!().to_string(),
                                        path,
                                        as_keyword.pos,
                                    ),
                                );
                                DeepTypeResult::NotFound
                            }
                        },
                        None => {
                            let path = parser.find_page(page_id).unwrap().path.clone();
                            parser.informations.push(
                                &error::error_list::ERROR_S6.clone().build_with_path(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: as_keyword.rtype.to_string(),
                                    }],
                                    file!().to_string(),
                                    path,
                                    as_keyword.type_pos,
                                ),
                            );
                            DeepTypeResult::NotFound
                        }
                    }
                }
                _ => {
                    todo!()
                }
            }
        }

        //Types::ArrowFunction(_) => todo!(),
        Types::Bool(_) => unreachable!(),
        Types::Void => todo!(),
        Types::Null => todo!(),
    }
}

pub fn resolve_deep_type(parser: &mut Parser, page_id: u64, rtype: Types) -> DeepTypeResult {
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
    parser: &mut Parser,
    target_page: u64,
    target_hash: u64,
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
                    .filter_map(|x| {
                        if x.processed && x.module.is_none() {
                            Some(x.clone())
                        } else {
                            None
                        }
                    })
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
    parser: &mut Parser,
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
                    .filter_map(|x| {
                        if x.processed && x.module.is_none() {
                            Some(x.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Dependency>>(),
            );
            inner_page = page.inner;
        }
        None => (),
    };

    if !searched.contains(&target_page) {
        let mut i = 0;
        loop {
            let dep = self_dependencies[i].clone();
            searched.push(target_page);
            match parser.find_processed_page(dep.hash) {
                Some(page) => {
                    let page = page.clone();
                    let internal_deps = page
                        .dependencies
                        .iter()
                        .filter_map(|x| if x.public { Some(x.clone()) } else { None })
                        .collect::<Vec<Dependency>>();
                    self_dependencies.extend(internal_deps);

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
            if i == self_dependencies.len() - 1 {
                break;
            }
            i += 1;
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

pub fn find_type(
    rtype: String,
    target_page: u64,
    parser: &mut Parser,
) -> Option<definers::GenericType> {
    let result = deep_search(parser, target_page, rtype.clone(), None, vec![], 0);
    if result.found {
        match result.found_item {
            ProcessedDeepSearchItems::Class(e) => Some(definers::GenericType {
                rtype,
                pos: e.pos,
                hash: e.hash,
            }),
            ProcessedDeepSearchItems::Variable(_) => {
                panic!("Unexpected internal crash, parser should have prevented this")
            }
            ProcessedDeepSearchItems::Function(_) => {
                panic!("Unexpected internal crash, parser should have prevented this")
            }
            ProcessedDeepSearchItems::ImportReference(_) => {
                panic!("Unexpected internal crash, parser should have prevented this")
            }
            ProcessedDeepSearchItems::None => None,
        }
    } else {
        None
    }
}

pub fn resolve_type(
    target_type: Types,
    target_page: u64,
    parser: &mut Parser,
) -> definers::DefinerCollecting {
    let deep_type =
        crate::deep_search_extensions::resolve_deep_type(parser, target_page, target_type.clone());

    match deep_type {
        DeepTypeResult::Integer(_) => {
            let int_type = find_type("int".to_string(), target_page, parser);
            match int_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find int type");
                }
            }
        }
        DeepTypeResult::Float(_) => {
            let float_type = find_type("float".to_string(), target_page, parser);
            match float_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find string type");
                }
            }
        }
        DeepTypeResult::Bool(_) => {
            let bool_type = find_type("bool".to_string(), target_page, parser);
            match bool_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find string type");
                }
            }
        }
        DeepTypeResult::String(_) => {
            let string_type = find_type("string".to_string(), target_page, parser);
            match string_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find string type");
                }
            }
        }
        DeepTypeResult::Char(_) => {
            let char_type = find_type("char".to_string(), target_page, parser);
            match char_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find char type");
                }
            }
        }
        DeepTypeResult::Collective(_) => todo!(),
        DeepTypeResult::Operator(_) => todo!(),
        DeepTypeResult::Cloak(_) => todo!(),
        DeepTypeResult::Array(array_type) => {
            #[derive(PartialEq, EnumAsInner, Clone)]
            enum GenericExists {
                Generic(definers::DefinerCollecting),
                Null,
            }
            let mut child_generic = GenericExists::Null;
            for entry in array_type.collective {
                let resolved = resolve_type(entry.value, target_page, parser);
                if child_generic != GenericExists::Null
                    && resolved != *child_generic.as_generic().unwrap()
                {
                    let dyn_type = find_type("dyn".to_string(), target_page, parser);
                    match dyn_type {
                        Some(dynamic_type) => {
                            child_generic = GenericExists::Generic(
                                definers::DefinerCollecting::Generic(dynamic_type),
                            );
                        }
                        None => {
                            panic!("Unhandled behaviour, failed to find string type");
                        }
                    }

                    break;
                }
                child_generic = GenericExists::Generic(resolved);
            }

            let array_type = find_type("array".to_string(), target_page, parser);
            match array_type {
                Some(array_generic) => {
                    let val = child_generic
                        .as_generic()
                        .unwrap_or(&definers::DefinerCollecting::Dynamic)
                        .clone();
                    definers::DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                        rtype: "array".to_string(),
                        generics: vec![definers::GenericParameter {
                            value: val,
                            pos: ellie_core::defs::Cursor::default(),
                        }],
                        hash: array_generic.hash,
                        parent_pos: ellie_core::defs::Cursor::default(),
                    })
                }
                None => panic!("Unhandled behaviour"),
            }
        }
        DeepTypeResult::Vector(_) => todo!(),
        DeepTypeResult::ClassCall(_) => todo!(),
        DeepTypeResult::FunctionCall(_) => todo!(),
        DeepTypeResult::Void => todo!(),
        DeepTypeResult::Null => todo!(),
        DeepTypeResult::NotFound => unreachable!(),
        DeepTypeResult::BraceReference(e) => {
            let nullable_type = find_type("nullAble".to_string(), target_page, parser);
            match nullable_type {
                Some(nullable_generic) => {
                    let nullable_child_generic = match *e.reference.clone() {
                        Types::Array(_) => {
                            let array_type = resolve_type(*e.reference, target_page, parser);
                            array_type.as_parent_generic().unwrap().generics[0]
                                .value
                                .clone()
                        }
                        _ => {
                            unimplemented!("Custom index queries are not yet supported",)
                        }
                    };
                    definers::DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                        rtype: "nullAble".to_string(),
                        generics: vec![definers::GenericParameter {
                            value: nullable_child_generic,
                            pos: ellie_core::defs::Cursor::default(),
                        }],
                        hash: nullable_generic.hash,
                        parent_pos: ellie_core::defs::Cursor::default(),
                    })
                }
                None => panic!("Unhandled behaviour"),
            }
        }
    }
}

/*
match target_type {
        Types::Integer(integer_type) => {
            let int_class = deep_search(parser, target_page, "class".to_owned(), None, vec![], 0);
            if int_class.found {
                if let ProcessedDeepSearchItems::Class(e) = int_class.found_item {
                    definers::DefinerCollecting::Generic(definers::GenericType {
                        rtype: "int".to_owned(),
                        pos: integer_type.pos,
                        hash: e.hash,
                    })
                } else {
                    unreachable!(
                        "Ellie must ensure that class is a class, and no one can replace it"
                    );
                }
            } else {
                unreachable!("Ellie must ensure that int exists");
            }
        }
        Types::Float(_) => todo!(),
        Types::Bool(_) => todo!(),
        Types::String(_) => todo!(),
        Types::Char(_) => todo!(),
        Types::Collective(_) => todo!(),
        Types::Reference(_) => todo!(),
        Types::BraceReference(_) => todo!(),
        Types::Operator(_) => todo!(),
        Types::Cloak(_) => todo!(),
        Types::Array(array_type) => {
            let deep_type = crate::deep_search_extensions::resolve_deep_type(parser, target_page, target_type.clone());


            panic!("{:#?}", deep_type);
        },
        Types::Vector(_) => todo!(),
        Types::ClassCall(_) => todo!(),
        Types::FunctionCall(_) => todo!(),
        Types::Void => todo!(),
        Types::NullResolver(_) => todo!(),
        Types::Negative(_) => todo!(),
        Types::VariableType(_) => todo!(),
        Types::AsKeyword(_) => todo!(),
        Types::Null => todo!(),
    }
*/
