use crate::processors::Processor;
use alloc::vec::Vec;
use alloc::{string::String, vec};
use ellie_core::{defs, information};
use ellie_tokenizer::processors::items::Processors;
use ellie_tokenizer::tokenizer::{Dependency, Page};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedPage {
    pub hash: u64,
    pub inner: Option<u64>,
    pub path: String,
    pub items: Vec<ellie_core::definite::items::Collecting>,
    pub dependents: Vec<u64>,
    pub dependencies: Vec<u64>,
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

trait DeepSearch {
    fn search(&self, name: String) -> Option<DeepSearchResult>;
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
        let deep_search = self.deep_search(page_id, name, Some(hash), vec![], 0);

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
            None => (),
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
                                        found = true;
                                        found_page = page.clone();
                                        found_type = DeepSearchItems::ImportReference(e);
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
                found_item: DeepSearchItems::MixUp(mixup_hashes),
                found_page,
            }
        } else if found {
            DeepSearchResult {
                found: true,
                found_item: found_type,
                found_page,
            }
        } else {
            DeepSearchResult {
                found: false,
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
                });
                for item in page.items.clone() {
                    match item {
                        Processors::Variable(e) => e.process(self, page.hash),
                        Processors::GetterCall(_) => todo!(),
                        Processors::SetterCall(_) => todo!(),
                        Processors::Function(_) => todo!(),
                        Processors::FileKey(e) => e.process(self, page.hash),
                        Processors::Import(e) => e.process(self, page.hash),
                        Processors::ForLoop(_) => todo!(),
                        Processors::Condition(_) => todo!(),
                        Processors::Constructor(e) => e.process(self, page.hash),
                        Processors::Class(e) => e.process(self, page.hash),
                        Processors::Ret(_) => todo!(),
                        Processors::SelfItem(_) => (),
                        Processors::GenericItem(_) => (),
                        Processors::FunctionVariable(_) => (),
                        Processors::ConstructorVariable(_) => (),
                    }
                }
            }
            _ => (),
        }
    }

    pub fn parse(&mut self) {
        self.process_page(0);
    }
}
