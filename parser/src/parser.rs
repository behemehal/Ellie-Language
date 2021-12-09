use crate::processors::Processor;
use alloc::vec::Vec;
use alloc::{string::String, vec};
use ellie_core::error;
use ellie_tokenizer::processors::items::Processors;
use ellie_tokenizer::tokenizer::{Dependency, Page};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedPage {
    pub hash: u64,
    pub path: String,
    pub items: Vec<ellie_core::definite::items::Collecting>,
    pub dependents: Vec<u64>,
    pub dependencies: Vec<u64>,
}

pub struct Parser {
    pub pages: Vec<Page>,
    pub processed_pages: Vec<ProcessedPage>,
    pub active_page: usize,
    pub errors: Vec<error::Error>,
}

#[derive(Debug, Clone)]
pub enum DeepSearchResult {
    Class(ellie_tokenizer::syntax::items::class::Class),
    Variable(ellie_tokenizer::syntax::items::variable::Variable),
    Function(ellie_tokenizer::syntax::items::function::Function),
    ImportReference(ellie_tokenizer::syntax::items::import::Import),
    BrokenPageGraph,
    MixUp(Vec<(String, String)>),
    None,
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
            errors: vec![],
        }
    }

    pub fn deep_search(
        &self,
        target_page: u64,
        name: String,
        ignore_hash: Option<String>,
        searched: Vec<u64>,
        level: u32,
    ) -> Option<DeepSearchResult> {
        let mut found = false;
        let mut found_type = DeepSearchResult::None;
        let has_mixup = false;
        let mut searched: Vec<u64> = searched;
        let mixup_hashes: Vec<(String, String)> = Vec::new();
        let mut self_dependendencies = vec![Dependency {
            hash: target_page,
            ..Default::default()
        }];

        if !searched.contains(&target_page) {
            'pages: for (_, dep) in self_dependendencies.clone().iter().enumerate() {
                searched.push(target_page);
                match self.find_page(dep.hash) {
                    Some(e) => {
                        if e.hash == target_page {
                            self_dependendencies.extend(e.dependencies.clone());
                        }
                        for item in e.items.iter() {
                            match item.clone() {
                                Processors::Variable(e) => {
                                    if e.data.name == name
                                        && (e.data.public || level == 0)
                                        && (ignore_hash.is_none()
                                            || matches!(ignore_hash, Some(ref t) if &e.data.hash != t))
                                    {
                                        found = true;
                                        found_type = DeepSearchResult::Variable(e.data);
                                    }
                                }
                                Processors::Function(e) => {
                                    if e.data.name == name
                                        && (e.data.public || level == 0)
                                        && (ignore_hash.is_none()
                                            || matches!(ignore_hash, Some(ref t) if &e.data.hash != t))
                                    {
                                        found = true;
                                        found_type = DeepSearchResult::Function(e.data);
                                    }
                                }
                                Processors::Import(e) => {
                                    if e.reference != ""
                                        && e.reference == name
                                        && (e.public || level == 0)
                                        && (ignore_hash.is_none()
                                            || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                    {
                                        found = true;
                                        found_type = DeepSearchResult::ImportReference(e);
                                    } else {
                                        match self.deep_search(
                                            e.hash.parse::<u64>().unwrap_or_else(|_| {
                                                panic!("Import's hash is not valid")
                                            }),
                                            name.clone(),
                                            None,
                                            searched.clone(),
                                            level + 1,
                                        ) {
                                            Some(found_result) => {
                                                found = true;
                                                found_type = found_result;
                                            }
                                            None => found = false,
                                        }
                                    }
                                }
                                Processors::Class(e) => {
                                    if e.name == name
                                        && (e.public || level == 0)
                                        && (ignore_hash.is_none()
                                            || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                    {
                                        found = true;
                                        found_type = DeepSearchResult::Class(e);
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    None => {
                        found = true;
                        found_type = DeepSearchResult::BrokenPageGraph;
                        break 'pages;
                    }
                }
            }
        }

        if has_mixup {
            Some(DeepSearchResult::MixUp(mixup_hashes))
        } else if found {
            Some(found_type)
        } else {
            None
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
                    path: page.path.clone(),
                    items: vec![],
                    dependents: vec![],
                    dependencies: vec![],
                });
            }
            _ => (),
        }

        for item in page.items.clone() {
            match item {
                Processors::Variable(e) => e.process(self, page.hash),
                Processors::GetterCall(_) => todo!(),
                Processors::SetterCall(_) => todo!(),
                Processors::Function(_) => todo!(),
                Processors::FileKey(_) => todo!(),
                Processors::Import(e) => e.process(self, page.hash),
                Processors::ForLoop(_) => todo!(),
                Processors::Condition(_) => todo!(),
                Processors::Constructor(_) => todo!(),
                Processors::Class(e) => e.process(self, page.hash),
                Processors::Ret(_) => todo!(),
            }
        }
    }

    pub fn parse(&mut self) {
        self.process_page(0);
    }
}
