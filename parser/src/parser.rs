use crate::processors::Processor;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::{string::String, vec};
use ellie_core::{defs, error, information, warning};
use ellie_tokenizer::processors::items::Processors;
use ellie_tokenizer::tokenizer::{Dependency, Page};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedPage {
    pub hash: u64,
    pub inner: Option<u64>,
    pub unreachable: bool,
    pub unreachable_range: defs::Cursor,
    pub path: String,
    pub items: Vec<ellie_core::definite::items::Collecting>,
    pub dependents: Vec<u64>,
    pub dependencies: Vec<u64>,
}

impl ProcessedPage {
    pub fn has_ret(&self) -> (bool, defs::Cursor) {
        let mut found = (false, defs::Cursor::default());
        for i in &self.items {
            match i {
                ellie_core::definite::items::Collecting::Ret(ret) => {
                    found = (true, ret.value_position);
                    break;
                }
                _ => {}
            }
        }
        found
    }

    pub fn find_dead_code(&self) -> (bool, defs::Cursor) {
        let mut dead = false;
        let mut range_start = defs::CursorPosition::default();
        let mut range_end = defs::CursorPosition::default();

        for item in self.items.iter() {
            match item {
                ellie_core::definite::items::Collecting::Ret(e) => {
                    dead = true;
                    range_start = e.pos.range_start;
                }
                e => {
                    if dead {
                        range_end = e.get_pos().range_start;
                    }
                }
            }
        }

        (
            dead,
            defs::Cursor {
                range_start,
                range_end,
            },
        )
    }
}

pub struct Parser {
    pub pages: Vec<Page>,
    pub processed_pages: Vec<ProcessedPage>,
    pub active_page: usize,
    pub informations: information::Informations,
}

#[derive(Debug, Clone)]
pub struct DeepSearchResult {
    pub found: bool,
    pub found_item: DeepSearchItems,
    pub found_pos: Option<defs::Cursor>,
    pub found_page: Page,
}

#[derive(Debug, Clone)]
pub enum DeepSearchItems {
    Class(ellie_tokenizer::syntax::items::class::Class),
    Variable(ellie_tokenizer::syntax::items::variable::Variable),
    Function(ellie_tokenizer::syntax::items::function::Function),
    ImportReference(ellie_tokenizer::syntax::items::import::Import),
    SelfItem(ellie_tokenizer::syntax::items::self_item::SelfItem),
    GenericItem(ellie_tokenizer::syntax::items::generic_item::GenericItem),
    FunctionParameter(ellie_tokenizer::syntax::items::function_parameter::FunctionParameter),
    ConstructorParameter(
        ellie_tokenizer::syntax::items::constructor_parameter::ConstructorParameter,
    ),
    BrokenPageGraph,
    MixUp(Vec<(String, String)>),
    None,
}

impl DeepSearchItems {
    pub fn get_pos(&self) -> defs::Cursor {
        match self {
            DeepSearchItems::Class(e) => e.pos,
            DeepSearchItems::Variable(e) => e.pos,
            DeepSearchItems::Function(e) => e.pos,
            DeepSearchItems::ImportReference(e) => e.pos,
            _ => defs::Cursor::default(),
        }
    }
}

impl Parser {
    pub fn new(pages: Vec<Page>) -> Parser {
        Parser {
            pages: pages,
            processed_pages: vec![],
            active_page: 0,
            informations: information::Informations::new(),
        }
    }

    pub fn resolve_type_name(&self, rtype: ellie_core::definite::types::Types) -> String {
        match rtype {
            ellie_core::definite::types::Types::Integer(_) => "int".to_string(),
            ellie_core::definite::types::Types::Float(_) => "flaot".to_string(),
            ellie_core::definite::types::Types::Bool(_) => "bool".to_string(),
            ellie_core::definite::types::Types::String(_) => "string".to_string(),
            ellie_core::definite::types::Types::Char(_) => "char".to_string(),
            ellie_core::definite::types::Types::Collective(_) => "collective".to_string(),
            ellie_core::definite::types::Types::Reference(_) => todo!(),
            ellie_core::definite::types::Types::BraceReference(_) => todo!(),
            ellie_core::definite::types::Types::Operator(_) => todo!(),
            ellie_core::definite::types::Types::Cloak(_) => "cloak".to_string(),
            ellie_core::definite::types::Types::Array(_) => "array".to_string(),
            ellie_core::definite::types::Types::Vector(_) => "vector".to_string(),
            ellie_core::definite::types::Types::ClassCall(_) => todo!(),
            ellie_core::definite::types::Types::FunctionCall(_) => todo!(),
            ellie_core::definite::types::Types::Void => "void".to_string(),
            ellie_core::definite::types::Types::NullResolver(e) => self.resolve_type_name(*e.value),
            ellie_core::definite::types::Types::Negative(e) => self.resolve_type_name(*e.value),
            ellie_core::definite::types::Types::VariableType(_) => todo!(),
            ellie_core::definite::types::Types::Null => "null".to_string(),
            ellie_core::definite::types::Types::AsKeyword(e) => self.resolve_type_name(*e.target),
        }
    }

    pub fn resolve_definer_name(
        &self,
        definer: ellie_core::definite::definers::DefinerCollecting,
    ) -> String {
        match definer {
            ellie_core::definite::definers::DefinerCollecting::Array(_) => "Array".to_string(),
            ellie_core::definite::definers::DefinerCollecting::Vector(_) => "Vector".to_string(),
            ellie_core::definite::definers::DefinerCollecting::Generic(e) => e.rtype,
            ellie_core::definite::definers::DefinerCollecting::ParentGeneric(e) => e.rtype,
            ellie_core::definite::definers::DefinerCollecting::Function(_) => {
                "Function".to_string()
            }

            ellie_core::definite::definers::DefinerCollecting::Cloak(_) => "Cloak".to_string(),
            ellie_core::definite::definers::DefinerCollecting::Collective(_) => {
                "Collective".to_string()
            }
            ellie_core::definite::definers::DefinerCollecting::Nullable(_) => {
                "NullAble".to_string()
            }
            ellie_core::definite::definers::DefinerCollecting::Dynamic => "Dynamic".to_string(),
        }
    }

    pub fn page_has_file_key_with(&self, page_id: u64, key: &str, value: &str) -> bool {
        let mut found = false;
        match self.find_page(page_id) {
            Some(e) => {
                for file in e.items.iter() {
                    match file {
                        Processors::FileKey(e) => {
                            if e.key_name == key
                                && matches!(&e.value, ellie_tokenizer::processors::types::Processors::String(e) if e.data.value == value)
                            {
                                found = true;
                                break;
                            }
                        }
                        _ => (),
                    }
                }
            }
            None => (),
        }
        found
    }

    pub fn is_duplicate(
        &self,
        page_id: u64,
        name: String,
        hash: String,
        pos: defs::Cursor,
    ) -> (bool, Option<(Page, defs::Cursor)>) {
        let deep_search = self.deep_search(
            page_id,
            name,
            if hash == "" { None } else { Some(hash) },
            vec![],
            0,
        );

        if deep_search.found {
            match deep_search.found_item {
                DeepSearchItems::BrokenPageGraph => (false, None),
                DeepSearchItems::MixUp(_) => (true, None),
                DeepSearchItems::None => (false, None),
                DeepSearchItems::SelfItem(_) => (true, None),
                DeepSearchItems::GenericItem(e) => (true, Some((deep_search.found_page, e.pos))),
                e => (
                    pos.is_bigger(e.get_pos()),
                    Some((deep_search.found_page, e.get_pos())),
                ),
            }
        } else {
            (false, None)
        }
    }

    pub fn deep_search(
        &self,
        target_page: u64,
        name: String,
        ignore_hash: Option<String>,
        searched: Vec<u64>,
        _level: u32,
    ) -> DeepSearchResult {
        let mut level = _level;
        let mut found = false;
        let mut found_type = DeepSearchItems::None;
        let mut found_pos = None;
        let mut found_page = Page::default();
        let has_mixup = false;
        let mut inner_page = None;
        let mut searched: Vec<u64> = searched;
        let mixup_hashes: Vec<(String, String)> = Vec::new();
        let mut self_dependendencies = vec![Dependency {
            hash: target_page,
            ..Default::default()
        }];

        match self.find_page(target_page) {
            Some(page) => {
                self_dependendencies.extend(page.dependencies.clone());
                inner_page = page.inner;
            }
            None => {
                alloc::format!("REMOVE THIS");
            }
        }

        if !searched.contains(&target_page) {
            'pages: for (_, dep) in self_dependendencies.clone().iter().enumerate() {
                searched.push(target_page);
                match self.find_page(dep.hash) {
                    Some(page) => {
                        for item in page.items.iter() {
                            match item.clone() {
                                Processors::Variable(e) => {
                                    if e.data.name == name
                                        && (e.data.public || level == 0)
                                        && (ignore_hash.is_none()
                                            || matches!(ignore_hash, Some(ref t) if &e.data.hash != t))
                                    {
                                        found_pos = Some(e.data.pos);
                                        found = true;
                                        found_page = page.clone();
                                        found_type = DeepSearchItems::Variable(e.data);
                                    }
                                }
                                Processors::Function(e) => {
                                    if e.data.name == name
                                        && (e.data.public || level == 0)
                                        && (ignore_hash.is_none()
                                            || matches!(ignore_hash, Some(ref t) if &e.data.hash != t))
                                    {
                                        found_pos = Some(e.data.pos);
                                        found = true;
                                        found_page = page.clone();
                                        found_type = DeepSearchItems::Function(e.data);
                                    }
                                }
                                Processors::Import(e) => {
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
                                        found_type = DeepSearchItems::ImportReference(e);
                                    } else {
                                    }
                                }
                                Processors::Class(e) => {
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
                                        found_type = DeepSearchItems::Class(e);
                                    }
                                }
                                Processors::GenericItem(e) => {
                                    if e.generic_name == name
                                        && (level == 0
                                            || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                    {
                                        found_pos = Some(e.pos);
                                        found = true;
                                        found_page = page.clone();
                                        found_type = DeepSearchItems::GenericItem(e);
                                    }
                                }
                                Processors::SelfItem(e) => {
                                    if "self" == name
                                        && (level == 0
                                            || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                    {
                                        found = true;
                                        found_page = page.clone();
                                        found_type = DeepSearchItems::SelfItem(e);
                                    }
                                }
                                Processors::FunctionParameter(e) => {
                                    if e.name == name && level == 0 {
                                        found_pos = Some(e.pos);
                                        found = true;
                                        found_page = page.clone();
                                        found_type = DeepSearchItems::FunctionParameter(e);
                                    }
                                }
                                Processors::ConstructorParameter(e) => {
                                    if e.name == name && level == 0 {
                                        found_pos = Some(e.pos);
                                        found = true;
                                        found_page = page.clone();
                                        found_type = DeepSearchItems::ConstructorParameter(e);
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    None => {
                        found = true;
                        found_type = DeepSearchItems::BrokenPageGraph;
                        break 'pages;
                    }
                }
                level += 1;
            }
        }

        if has_mixup {
            DeepSearchResult {
                found: true,
                found_pos,
                found_item: DeepSearchItems::MixUp(mixup_hashes),
                found_page,
            }
        } else if found {
            DeepSearchResult {
                found: true,
                found_pos,
                found_item: found_type,
                found_page,
            }
        } else {
            DeepSearchResult {
                found: false,
                found_pos,
                found_item: DeepSearchItems::None,
                found_page,
            }
        }
    }

    pub fn find_processed_page(&mut self, hash: u64) -> Option<&mut ProcessedPage> {
        self.processed_pages.iter_mut().find(|x| x.hash == hash)
    }

    pub fn find_page(&self, hash: u64) -> Option<&Page> {
        self.pages.iter().find(|x| x.hash == hash)
    }

    pub fn process_page(&mut self, hash: u64) {
        let page = self
            .find_page(hash)
            .unwrap_or_else(|| panic!("Page not found"))
            .clone();

        match self.find_processed_page(hash) {
            None => {
                self.processed_pages.push(ProcessedPage {
                    hash: hash,
                    inner: page.inner,
                    path: page.path.clone(),
                    items: vec![],
                    dependents: vec![],
                    dependencies: vec![],
                    unreachable: false,
                    unreachable_range: defs::Cursor::default(),
                });
                None
            }
            Some(e) => Some(e.clone()),
        };

        for item in page.items.clone() {
            if matches!(self.find_processed_page(hash), Some(e) if e.unreachable) {
                if !item.is_virtual() {
                    let page = self.find_processed_page(hash).unwrap();
                    page.unreachable_range.range_end = item.get_pos().range_end;
                }
            } else {
                match page.page_type {
                    ellie_tokenizer::tokenizer::PageType::FunctionBody => match item {
                        Processors::Variable(e) => e.process(self, page.hash),
                        Processors::GetterCall(_) => todo!(),
                        Processors::SetterCall(_) => todo!(),
                        Processors::Function(e) => e.process(self, page.hash),
                        Processors::FileKey(e) => e.process(self, page.hash),
                        Processors::ForLoop(_) => todo!(),
                        Processors::Condition(_) => todo!(),
                        Processors::Class(e) => e.process(self, page.hash),
                        Processors::Ret(e) => e.process(self, page.hash),
                        Processors::SelfItem(_) => (),
                        Processors::GenericItem(_) => (),
                        Processors::FunctionParameter(_) => (),
                        Processors::ConstructorParameter(_) => (),
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465".to_string(),
                                    page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::ConstructorBody => match item {
                        Processors::Variable(e) => e.process(self, page.hash),
                        Processors::GetterCall(_) => todo!(),
                        Processors::SetterCall(_) => todo!(),
                        Processors::Function(e) => e.process(self, page.hash),
                        Processors::FileKey(e) => e.process(self, page.hash),
                        Processors::ForLoop(_) => todo!(),
                        Processors::Condition(_) => todo!(),
                        Processors::Class(e) => e.process(self, page.hash),
                        Processors::Ret(e) => e.process(self, page.hash),
                        Processors::SelfItem(_) => (),
                        Processors::GenericItem(_) => (),
                        Processors::FunctionParameter(_) => (),
                        Processors::ConstructorParameter(_) => (),
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465".to_string(),
                                    page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::RawBody => match item {
                        Processors::Variable(e) => e.process(self, page.hash),
                        Processors::GetterCall(_) => todo!(),
                        Processors::SetterCall(_) => todo!(),
                        Processors::Function(e) => e.process(self, page.hash),
                        Processors::FileKey(e) => e.process(self, page.hash),
                        Processors::Import(e) => {
                            let hash = e.hash.clone();
                            e.process(self, page.hash);
                            match hash.parse::<u64>() {
                                Ok(hash) => {
                                    if self.find_processed_page(hash).is_none() {
                                        self.process_page(hash);
                                    }
                                }
                                Err(_) => {
                                    panic!("Import's hash is not valid");
                                }
                            }
                        }
                        Processors::ForLoop(_) => todo!(),
                        Processors::Condition(_) => todo!(),
                        Processors::Class(e) => e.process(self, page.hash),

                        Processors::SelfItem(_) => (),
                        Processors::GenericItem(_) => (),
                        Processors::FunctionParameter(_) => (),
                        Processors::ConstructorParameter(_) => (),
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465".to_string(),
                                    page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::ClassBody => match item {
                        Processors::Variable(e) => e.process(self, page.hash),
                        Processors::Function(e) => e.process(self, page.hash),
                        Processors::FileKey(e) => e.process(self, page.hash),
                        Processors::Constructor(e) => e.process(self, page.hash),
                        Processors::SelfItem(_) => (),
                        Processors::GenericItem(_) => (),
                        Processors::FunctionParameter(_) => (),
                        Processors::ConstructorParameter(_) => (),
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465".to_string(),
                                    page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::ValueConditionBody => match item {
                        Processors::Variable(e) => e.process(self, page.hash),
                        Processors::GetterCall(_) => todo!(),
                        Processors::SetterCall(_) => todo!(),
                        Processors::Function(e) => e.process(self, page.hash),
                        Processors::FileKey(e) => e.process(self, page.hash),
                        Processors::ForLoop(_) => todo!(),
                        Processors::Condition(_) => todo!(),
                        Processors::Class(e) => e.process(self, page.hash),
                        Processors::Ret(e) => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465_Not_YetSupported".to_string(),
                                    page.path.clone(),
                                    e.pos,
                                ),
                            );
                        },
                        Processors::SelfItem(_) => (),
                        Processors::GenericItem(_) => (),
                        Processors::FunctionParameter(_) => (),
                        Processors::ConstructorParameter(_) => (),
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465".to_string(),
                                    page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                        }
                    },
                }
            }
        }

        #[cfg(feature = "standard_rules")]
        {
            match self.find_processed_page(hash) {
                Some(e) => {
                    let q = e.clone();
                    if q.unreachable && !q.unreachable_range.range_end.is_zero() {
                        self.informations
                            .push(&warning::warning_list::WARNING_S4.clone().build(
                                vec![],
                                q.path,
                                q.unreachable_range,
                            ));
                    }
                }
                _ => (),
            }
        }
    }

    pub fn parse(&mut self) {
        self.process_page(0);
    }
}
