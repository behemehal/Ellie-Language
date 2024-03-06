use alloc::{string::String, vec::Vec};
use ellie_core::defs;

pub struct DeepSearchOptions<'a> {
    pub(in crate::extra) parser: Option<&'a mut crate::parser::Parser>,
    pub(in crate::extra) page_id: Option<usize>,
    pub(in crate::extra) searched: Vec<usize>,
    pub(in crate::extra) level: usize,
    pub(in crate::extra) position: Option<defs::Cursor>,
    pub(in crate::extra) name_to_search: Option<String>,
    pub(in crate::extra) hash_to_search: Option<usize>,
    pub(in crate::extra) ignore_hash: Option<usize>,
    pub(in crate::extra) search_on_modules: bool,
    pub(in crate::extra) search_on_processed: bool,
    pub(in crate::extra) search_on_raw: bool,
}

impl<'a> DeepSearchOptions<'a> {
    pub fn new() -> Self {
        Self {
            parser: None,
            page_id: None,
            searched: Vec::new(),
            level: 0,
            position: None,
            name_to_search: None,
            hash_to_search: None,
            ignore_hash: None,
            search_on_modules: false,
            search_on_processed: false,
            search_on_raw: false,
        }
    }

    pub fn parser(mut self, parser: &'a mut crate::parser::Parser) -> Self {
        self.parser = Some(parser);
        self
    }

    pub fn page_id(mut self, page_id: usize) -> Self {
        self.page_id = Some(page_id);
        self
    }

    pub fn searched(mut self, searched: Vec<usize>) -> Self {
        self.searched = searched;
        self
    }

    pub fn level(mut self, level: usize) -> Self {
        self.level = level;
        self
    }

    pub fn position(mut self, position: defs::Cursor) -> Self {
        self.position = Some(position);
        self
    }

    pub fn search_on_all(mut self) -> Self {
        self.search_on_modules = true;
        self.search_on_processed = true;
        self.search_on_raw = true;
        self
    }

    pub fn search_on_modules(mut self) -> Self {
        self.search_on_modules = true;
        self
    }

    pub fn search_on_processed(mut self) -> Self {
        self.search_on_processed = true;
        self
    }

    pub fn search_on_raw(mut self) -> Self {
        self.search_on_raw = true;
        self
    }

    pub fn increase_level(&mut self) {
        self.level += 1;
    }

    pub fn name(mut self, name: String) -> Self {
        self.name_to_search = Some(name);
        self
    }

    pub fn hash(mut self, hash: usize) -> Self {
        self.hash_to_search = Some(hash);
        self
    }

    pub fn ignore_hash(mut self, hash: usize) -> Self {
        self.ignore_hash = Some(hash);
        self
    }

    pub(in crate::extra) fn hash_or_name_matched(&self, hash: usize, name: &str) -> bool {
        matches!(self.hash_to_search, Some(hash_to_search) if hash_to_search == hash)
            || matches!(&self.name_to_search, Some(name_to_search) if name_to_search == name)
    }

    pub fn build(self) -> Self {
        if self.parser.is_none() {
            panic!("Parser is not set");
        }

        if self.page_id.is_none() {
            panic!("Page ID is not set");
        }

        if self.name_to_search.is_none() && self.hash_to_search.is_none() {
            panic!("Name or hash to search is not set");
        }

        if !self.search_on_modules && !self.search_on_processed && !self.search_on_raw {
            panic!("No search criteria is set");
        }

        self
    }
}
