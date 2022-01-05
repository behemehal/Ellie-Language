use crate::processors::Processor;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::{string::String, vec};
use ellie_core::definite::{items::Collecting, Converter};
use ellie_core::{defs, error, information, warning};
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
    pub dependencies: Vec<ellie_tokenizer::tokenizer::Dependency>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub hash: u64,
    pub name: String,
    pub description: String,
    pub initial_page: u64,
    pub ellie_version: ellie_core::defs::Version,
    pub pages: Vec<ProcessedPage>,
    pub version: ellie_core::defs::Version,
    pub modules: Vec<ellie_tokenizer::tokenizer::Module>,
}

pub struct Parser {
    pub version: ellie_core::defs::Version,
    pub pages: Vec<Page>,
    pub processed_pages: Vec<ProcessedPage>,
    pub modules: Vec<Module>,
    pub initial_page: u64,
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
    pub fn new(
        pages: Vec<Page>,
        initial_hash: Option<u64>,
        version: ellie_core::defs::Version,
    ) -> Parser {
        Parser {
            version,
            pages,
            processed_pages: vec![],
            modules: vec![],
            initial_page: initial_hash.unwrap_or(0),
            informations: information::Informations::new(),
        }
    }

    pub fn calculate_hash(&self) -> u64 {
        self.pages[0].hash
    }

    pub fn import_module(&mut self, module: Module) {
        self.modules.push(module.clone());
        let unprocessed_pages = module
            .pages
            .iter()
            .map(|p| ellie_tokenizer::tokenizer::Page {
                hash: p.hash,
                inner: p.inner,
                path: p.path.clone(),
                module: true,
                dependents: p.dependents.clone(),
                dependencies: p.dependencies.clone(),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let imported_dependencies: Vec<Dependency> = module
            .pages
            .iter()
            .map(|x| ellie_tokenizer::tokenizer::Dependency {
                hash: x.hash,
                processed: true,
                module: Some(module.initial_page),
                deep_link: if x.hash == 343 { None } else { Some(343) },
                public: false,
            })
            .collect();

        self.find_page(self.initial_page)
            .unwrap()
            .dependencies
            .extend(imported_dependencies);
        self.pages.extend(unprocessed_pages);
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

    pub fn page_has_file_key_with(&mut self, page_id: u64, key: &str, value: &str) -> bool {
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
        &mut self,
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
        &mut self,
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
            None => (),
        }

        if !searched.contains(&target_page) {
            for dep in self_dependendencies {
                searched.push(target_page);

                if let Some(module_initial_page) = dep.module {
                    let unprocessed_page = self
                        .find_page(dep.hash)
                        .unwrap_or_else(|| panic!("BrokenPageGraph: {}", dep.hash))
                        .clone();

                    match self.find_processed_page_in_module(module_initial_page, dep.hash) {
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
                                            found_page = unprocessed_page.clone();
                                            found_type = DeepSearchItems::Variable(ellie_tokenizer::syntax::items::variable::VariableCollector::default().from_definite(e).data);
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
                                            found_page = unprocessed_page.clone();
                                            found_type = DeepSearchItems::Function(ellie_tokenizer::syntax::items::function::FunctionCollector::default().from_definite(e).data);
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
                                            found_page = unprocessed_page.clone();
                                            found_type = DeepSearchItems::ImportReference(ellie_tokenizer::syntax::items::import::Import::default().from_definite(e));
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
                                            found_page = unprocessed_page.clone();
                                            found_type = DeepSearchItems::Class(ellie_tokenizer::syntax::items::class::Class::default().from_definite(e));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        None => {
                            panic!("Broken Page structure; Failed to find page {}", dep.hash);
                            //found = true;
                            //found_type = DeepSearchItems::BrokenPageGraph;
                            //break 'pages;
                        }
                    }
                } else if dep.processed {
                    let unprocessed_page = self
                        .find_page(dep.hash)
                        .unwrap_or_else(|| panic!("BrokenPageGraph: {}", dep.hash))
                        .clone();
                    match self.find_processed_page(dep.hash) {
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
                                            found_page = unprocessed_page.clone();
                                            found_type = DeepSearchItems::Variable(ellie_tokenizer::syntax::items::variable::VariableCollector::default().from_definite(e).data);
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
                                            found_page = unprocessed_page.clone();
                                            found_type = DeepSearchItems::Function(ellie_tokenizer::syntax::items::function::FunctionCollector::default().from_definite(e).data);
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
                                            found_page = unprocessed_page.clone();
                                            found_type = DeepSearchItems::ImportReference(ellie_tokenizer::syntax::items::import::Import::default().from_definite(e));
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
                                            found_page = unprocessed_page.clone();
                                            found_type = DeepSearchItems::Class(ellie_tokenizer::syntax::items::class::Class::default().from_definite(e));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        None => {
                            panic!("Broken Page structure; Failed to find page {}", dep.hash);
                            //found = true;
                            //found_type = DeepSearchItems::BrokenPageGraph;
                            //break 'pages;
                        }
                    }
                } else {
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
                            panic!("Broken Page structure; Failed to find page {}", dep.hash);
                            //found = true;
                            //found_type = DeepSearchItems::BrokenPageGraph;
                            //break 'pages;
                        }
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

    pub fn find_processed_page_in_module(
        &mut self,
        module_hash: u64,
        hash: u64,
    ) -> Option<&mut ProcessedPage> {
        match self.modules.iter_mut().find(|x| x.hash == module_hash) {
            Some(e) => e.pages.iter_mut().find(|x| x.hash == hash),
            None => None,
        }
    }

    pub fn find_page(&mut self, hash: u64) -> Option<&mut Page> {
        self.pages.iter_mut().find(|x| x.hash == hash)
    }

    pub fn process_page(&mut self, hash: u64) {
        let mut unprocessed_page = self
            .find_page(hash)
            .unwrap_or_else(|| panic!("Page not found"))
            .clone();

        match self.find_processed_page(hash) {
            None => {
                self.processed_pages.push(ProcessedPage {
                    hash: hash,
                    inner: unprocessed_page.inner,
                    path: unprocessed_page.path.clone(),
                    items: vec![],
                    dependents: unprocessed_page.dependents,
                    dependencies: unprocessed_page.dependencies,
                });
                None
            }
            Some(e) => Some(e.clone()),
        };

        for item in unprocessed_page.items.clone() {
            if unprocessed_page.unreachable {
                if !item.is_virtual() {
                    unprocessed_page.unreachable_range.range_end = item.get_pos().range_end;
                }
            } else {
                match unprocessed_page.page_type {
                    ellie_tokenizer::tokenizer::PageType::FunctionBody => match item {
                        Processors::Variable(e) => e.process(self, unprocessed_page.hash),
                        Processors::GetterCall(_) => todo!(),
                        Processors::SetterCall(_) => todo!(),
                        Processors::Function(e) => e.process(self, unprocessed_page.hash),
                        Processors::FileKey(e) => e.process(self, unprocessed_page.hash),
                        Processors::ForLoop(_) => todo!(),
                        Processors::Condition(_) => todo!(),
                        Processors::Class(e) => e.process(self, unprocessed_page.hash),
                        Processors::Ret(e) => e.process(self, unprocessed_page.hash),
                        Processors::SelfItem(_) => (),
                        Processors::GenericItem(_) => (),
                        Processors::FunctionParameter(_) => (),
                        Processors::ConstructorParameter(_) => (),
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465".to_string(),
                                    unprocessed_page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::ConstructorBody => match item {
                        Processors::Variable(e) => e.process(self, unprocessed_page.hash),
                        Processors::GetterCall(_) => todo!(),
                        Processors::SetterCall(_) => todo!(),
                        Processors::Function(e) => e.process(self, unprocessed_page.hash),
                        Processors::FileKey(e) => e.process(self, unprocessed_page.hash),
                        Processors::ForLoop(_) => todo!(),
                        Processors::Condition(_) => todo!(),
                        Processors::Class(e) => e.process(self, unprocessed_page.hash),
                        Processors::Ret(e) => e.process(self, unprocessed_page.hash),
                        Processors::SelfItem(_) => (),
                        Processors::GenericItem(_) => (),
                        Processors::FunctionParameter(_) => (),
                        Processors::ConstructorParameter(_) => (),
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465".to_string(),
                                    unprocessed_page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::RawBody => match item {
                        Processors::Variable(e) => e.process(self, unprocessed_page.hash),
                        Processors::GetterCall(_) => todo!(),
                        Processors::SetterCall(_) => todo!(),
                        Processors::Function(e) => e.process(self, unprocessed_page.hash),
                        Processors::FileKey(e) => e.process(self, unprocessed_page.hash),
                        Processors::Import(e) => {
                            let hash = e.hash.clone();
                            e.process(self, unprocessed_page.hash);
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
                        Processors::Class(e) => e.process(self, unprocessed_page.hash),

                        Processors::SelfItem(_) => (),
                        Processors::GenericItem(_) => (),
                        Processors::FunctionParameter(_) => (),
                        Processors::ConstructorParameter(_) => (),
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465".to_string(),
                                    unprocessed_page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::ClassBody => match item {
                        Processors::Variable(e) => e.process(self, unprocessed_page.hash),
                        Processors::Function(e) => e.process(self, unprocessed_page.hash),
                        Processors::FileKey(e) => e.process(self, unprocessed_page.hash),
                        Processors::Constructor(e) => e.process(self, unprocessed_page.hash),
                        Processors::SelfItem(_) => (),
                        Processors::GenericItem(_) => (),
                        Processors::FunctionParameter(_) => (),
                        Processors::ConstructorParameter(_) => (),
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465".to_string(),
                                    unprocessed_page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::ValueConditionBody => match item {
                        Processors::Variable(e) => e.process(self, unprocessed_page.hash),
                        Processors::GetterCall(_) => todo!(),
                        Processors::SetterCall(_) => todo!(),
                        Processors::Function(e) => e.process(self, unprocessed_page.hash),
                        Processors::FileKey(e) => e.process(self, unprocessed_page.hash),
                        Processors::ForLoop(_) => todo!(),
                        Processors::Condition(_) => todo!(),
                        Processors::Class(e) => e.process(self, unprocessed_page.hash),
                        Processors::Ret(e) => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465_Not_YetSupported".to_string(),
                                    unprocessed_page.path.clone(),
                                    e.pos,
                                ),
                            );
                        }
                        Processors::SelfItem(_) => (),
                        Processors::GenericItem(_) => (),
                        Processors::FunctionParameter(_) => (),
                        Processors::ConstructorParameter(_) => (),
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    "p_0x465".to_string(),
                                    unprocessed_page.path.clone(),
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
            match self.find_page(hash) {
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

    pub fn parse(
        &mut self,
        module_name: String,
        module_description: String,
        ellie_version: defs::Version,
    ) -> Module {
        self.process_page(self.initial_page);
        Module {
            name: module_name,
            description: module_description,
            initial_page: self.initial_page,
            hash: self.calculate_hash(),
            pages: self.processed_pages.clone(),
            version: self.version.clone(),
            ellie_version,
            modules: self
                .modules
                .iter()
                .map(|x| ellie_tokenizer::tokenizer::Module {
                    hash: x.hash,
                    initial_page: x.initial_page,
                    version: x.version.clone(),
                    name: x.name.clone(),
                })
                .collect(),
        }
    }
}
