use crate::utils::ProcessedPage;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite::items::Collecting;
use ellie_tokenizer::{
    processors::items::Processors,
    tokenizer::{Dependency, Page},
};

#[derive(Debug, Clone)]
pub struct FoundPage {
    pub raw_page: Option<Page>,
    pub processed_page: Option<ProcessedPage>,
}

impl FoundPage {
    pub fn new() -> Self {
        FoundPage {
            raw_page: None,
            processed_page: None,
        }
    }

    pub fn as_raw(&self) -> &Page {
        &self.raw_page.as_ref().unwrap()
    }

    pub fn as_processed(&self) -> &ProcessedPage {
        &self.processed_page.as_ref().unwrap()
    }

    pub fn fill(&mut self, page: &Page) {
        self.raw_page = Some(page.clone());
    }

    pub fn fill_processed(&mut self, page: ProcessedPage) {
        self.processed_page = Some(page);
    }
}

#[derive(Debug, Clone)]
pub struct FoundItem {
    raw_item: Option<Processors>,
    processed_item: Option<Collecting>,
}

impl FoundItem {
    pub fn new() -> Self {
        FoundItem {
            raw_item: None,
            processed_item: None,
        }
    }

    pub fn raw(&self) -> Option<&Processors> {
        self.raw_item.as_ref()
    }

    pub fn processed(&self) -> Option<&Collecting> {
        self.processed_item.as_ref()
    }

    pub fn as_raw(&self) -> &Processors {
        self.raw_item.as_ref().unwrap()
    }

    pub fn as_processed(&self) -> &Collecting {
        self.processed_item.as_ref().unwrap()
    }

    pub fn found(&self) -> bool {
        self.raw_item.is_some() || self.processed_item.is_some()
    }

    pub fn fill_raw(&mut self, raw: Processors) {
        self.raw_item = Some(raw);
    }

    pub fn fill_processed(&mut self, processed: Collecting) {
        self.processed_item = Some(processed);
    }
}

#[derive(Debug, Clone)]
pub struct ItemSearchResult {
    pub found_item: FoundItem,
    pub found_page: FoundPage,
}

impl ItemSearchResult {
    pub fn found(&self) -> bool {
        self.found_item.found()
    }
}

pub fn item_search(options: &mut super::utils::DeepSearchOptions) -> ItemSearchResult {
    // let mut inner_page = None;
    let mut searched: Vec<usize> = options.searched.clone();

    let mut self_dependencies = vec![Dependency {
        hash: options.page_id.unwrap(),
        ..Default::default()
    }];

    match options
        .parser
        .as_mut()
        .unwrap()
        .find_page(options.page_id.unwrap())
    {
        Some(page) => {
            self_dependencies.extend(page.dependencies.clone());
            //inner_page = page.inner;
        }
        None => (),
    }

    let mut found_item = FoundItem::new();
    let mut found_page = FoundPage::new();

    let parser = options.parser.take().unwrap();

    if !searched.contains(&options.page_id.unwrap()) {
        for dependency in self_dependencies {
            searched.push(dependency.hash);

            if let Some(module_initial_page) = dependency.module {
                if !options.search_on_modules {
                    continue;
                }
                match parser.find_processed_page_in_module(module_initial_page, dependency.hash) {
                    Some(page) => {
                        for item in &page.items {
                            match item.clone() {
                                Collecting::Variable(variable) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(variable.hash, &variable.name);

                                    // Deep link is for global imports like ellieCore
                                    let publicity_met = variable.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != variable.hash;

                                    let position_criteria_met = options.position.is_none()
                                        || options.level != 0
                                        || (options.level == 0
                                            && options
                                                .position
                                                .unwrap()
                                                .range_start
                                                .is_bigger(&variable.pos.range_start));

                                    if name_or_hash_met
                                        && publicity_met
                                        && ignore_hash
                                        && position_criteria_met
                                    {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::Function(function) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(function.hash, &function.name);

                                    let publicity_met = function.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != function.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::Class(class) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(class.hash, &class.name);

                                    // Deep link is for global imports like ellieCore
                                    let publicity_met = class.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != class.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::Import(import) => {
                                    if !import.reference.is_empty() {
                                        let name_or_hash_met = options
                                            .hash_or_name_matched(import.hash, &import.reference);

                                        let publicity_met = import.public
                                            || options.level == 0
                                            || dependency.deep_link.is_some();

                                        let ignore_hash = options.ignore_hash.is_none()
                                            || options.ignore_hash.unwrap() != import.hash;

                                        if name_or_hash_met && publicity_met && ignore_hash {
                                            found_item.fill_processed(item.clone());
                                            found_page.fill_processed(page.clone());
                                        }
                                    }
                                }
                                Collecting::Getter(getter) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(getter.hash, &getter.name);

                                    // Deep link is for global imports like ellieCore
                                    let publicity_met = getter.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != getter.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::Setter(seter) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(seter.hash, &seter.name);

                                    // Deep link is for global imports like ellieCore
                                    let publicity_met = seter.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != seter.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::Enum(r#enum) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(r#enum.hash, &r#enum.name);

                                    // Deep link is for global imports like ellieCore
                                    let publicity_met = r#enum.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != r#enum.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::NativeFunction(native_function) => {
                                    let name_or_hash_met = options.hash_or_name_matched(
                                        native_function.hash,
                                        &native_function.name,
                                    );

                                    // Deep link is for global imports like ellieCore
                                    let publicity_met = native_function.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != native_function.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::FunctionParameter(function_parameter) => {
                                    let name_or_hash_met = options.hash_or_name_matched(
                                        function_parameter.hash,
                                        &function_parameter.name,
                                    );

                                    if name_or_hash_met {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::ConstructorParameter(constructor_parameter) => {
                                    let name_or_hash_met = options.hash_or_name_matched(
                                        constructor_parameter.hash,
                                        &constructor_parameter.name,
                                    );

                                    if name_or_hash_met {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::SelfItem(_) => {
                                    if matches!(&options.name_to_search, Some(name) if name == "self")
                                    {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    None => unreachable!("Broken Page structure"),
                }
            }

            if dependency.processed && options.search_on_processed && !found_item.found() {
                match parser.find_processed_page(dependency.hash) {
                    Some(page) => {
                        for item in &page.items {
                            match item.clone() {
                                Collecting::Variable(variable) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(variable.hash, &variable.name);

                                    let publicity_met = variable.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != variable.hash;

                                    let position_criteria_met = options.position.is_none()
                                        || options.level != 0
                                        || (options.level == 0
                                            && options
                                                .position
                                                .unwrap()
                                                .range_start
                                                .is_bigger(&variable.pos.range_start));

                                    if name_or_hash_met
                                        && publicity_met
                                        && ignore_hash
                                        && position_criteria_met
                                    {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::Function(function) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(function.hash, &function.name);

                                    let publicity_met = function.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != function.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::Class(class) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(class.hash, &class.name);

                                    let publicity_met = class.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != class.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::Import(import) => {
                                    if !import.reference.is_empty() {
                                        let name_or_hash_met = options
                                            .hash_or_name_matched(import.hash, &import.reference);

                                        let publicity_met = import.public
                                            || options.level == 0
                                            || dependency.deep_link.is_some();

                                        let ignore_hash = options.ignore_hash.is_none()
                                            || options.ignore_hash.unwrap() != import.hash;

                                        if name_or_hash_met && publicity_met && ignore_hash {
                                            found_item.fill_processed(item.clone());
                                            found_page.fill_processed(page.clone());
                                        }
                                    }
                                }
                                Collecting::Getter(getter) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(getter.hash, &getter.name);

                                    let publicity_met = getter.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != getter.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::Setter(seter) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(seter.hash, &seter.name);

                                    let publicity_met = seter.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != seter.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::Enum(r#enum) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(r#enum.hash, &r#enum.name);

                                    let publicity_met = r#enum.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != r#enum.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::NativeFunction(native_function) => {
                                    let name_or_hash_met = options.hash_or_name_matched(
                                        native_function.hash,
                                        &native_function.name,
                                    );

                                    let publicity_met = native_function.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != native_function.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::FunctionParameter(function_parameter) => {
                                    let name_or_hash_met = options.hash_or_name_matched(
                                        function_parameter.hash,
                                        &function_parameter.name,
                                    );

                                    if name_or_hash_met {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::ConstructorParameter(constructor_parameter) => {
                                    let name_or_hash_met = options.hash_or_name_matched(
                                        constructor_parameter.hash,
                                        &constructor_parameter.name,
                                    );

                                    if name_or_hash_met {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                Collecting::SelfItem(_) => {
                                    if matches!(&options.name_to_search, Some(name) if name == "self")
                                    {
                                        found_item.fill_processed(item.clone());
                                        found_page.fill_processed(page.clone());
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    None => unreachable!(
                        "Broken Page structure; Failed to find page {}",
                        dependency.hash
                    ),
                }
            }

            if options.search_on_raw && !found_item.found() {
                match parser.find_page(dependency.hash) {
                    Some(page) => {
                        for item in page.items.iter() {
                            match item {
                                Processors::Variable(variable) => {
                                    let name_or_hash_met = options.hash_or_name_matched(
                                        variable.data.hash,
                                        &variable.data.name,
                                    );

                                    let publicity_met = variable.data.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != variable.data.hash;

                                    let position_criteria_met = options.position.is_none()
                                        || options.level != 0
                                        || (options.level == 0
                                            && options
                                                .position
                                                .unwrap()
                                                .range_start
                                                .is_bigger(&variable.data.pos.range_start));

                                    if name_or_hash_met
                                        && publicity_met
                                        && ignore_hash
                                        && position_criteria_met
                                    {
                                        found_item.fill_raw(item.clone());
                                        found_page.fill(page);
                                    }
                                }
                                Processors::Function(function) => {
                                    let name_or_hash_met = options.hash_or_name_matched(
                                        function.data.hash,
                                        &function.data.name,
                                    );

                                    let publicity_met = function.data.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != function.data.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_raw(item.clone());
                                        found_page.fill(page);
                                    }
                                }
                                Processors::Class(class) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(class.hash, &class.name);

                                    let publicity_met = class.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != class.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_raw(item.clone());
                                        found_page.fill(page);
                                    }
                                }
                                Processors::Import(import) => {
                                    if !import.reference.is_empty() {
                                        let name_or_hash_met = options
                                            .hash_or_name_matched(import.hash, &import.reference);

                                        let publicity_met = import.public
                                            || options.level == 0
                                            || dependency.deep_link.is_some();

                                        let ignore_hash = options.ignore_hash.is_none()
                                            || options.ignore_hash.unwrap() != import.hash;

                                        if name_or_hash_met && publicity_met && ignore_hash {
                                            found_item.fill_raw(item.clone());
                                            found_page.fill(page);
                                        }
                                    }
                                }
                                Processors::Getter(getter) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(getter.hash, &getter.name);

                                    let publicity_met = getter.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != getter.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_raw(item.clone());
                                        found_page.fill(page);
                                    }
                                }
                                Processors::Setter(seter) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(seter.hash, &seter.name);

                                    let publicity_met = seter.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != seter.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_raw(item.clone());
                                        found_page.fill(page);
                                    }
                                }
                                Processors::Enum(r#enum) => {
                                    let name_or_hash_met =
                                        options.hash_or_name_matched(r#enum.hash, &r#enum.name);

                                    let publicity_met = r#enum.public
                                        || options.level == 0
                                        || dependency.deep_link.is_some();

                                    let ignore_hash = options.ignore_hash.is_none()
                                        || options.ignore_hash.unwrap() != r#enum.hash;

                                    if name_or_hash_met && publicity_met && ignore_hash {
                                        found_item.fill_raw(item.clone());
                                        found_page.fill(page);
                                    }
                                }
                                Processors::FunctionParameter(function_parameter) => {
                                    let name_or_hash_met = options.hash_or_name_matched(
                                        function_parameter.hash,
                                        &function_parameter.name,
                                    );

                                    if name_or_hash_met {
                                        found_item.fill_raw(item.clone());
                                        found_page.fill(page);
                                    }
                                }
                                Processors::ConstructorParameter(constructor_parameter) => {
                                    let name_or_hash_met = options.hash_or_name_matched(
                                        constructor_parameter.hash,
                                        &constructor_parameter.name,
                                    );

                                    if name_or_hash_met {
                                        found_item.fill_raw(item.clone());
                                        found_page.fill(page);
                                    }
                                }
                                Processors::GenericItem(generic_item) => {
                                    let name_or_hash_met = options.hash_or_name_matched(
                                        generic_item.hash,
                                        &generic_item.generic_name,
                                    );

                                    if name_or_hash_met {
                                        found_item.fill_raw(item.clone());
                                        found_page.fill(page);
                                    }
                                }
                                Processors::SelfItem(_) => {
                                    if matches!(&options.name_to_search, Some(name) if name == "self")
                                    {
                                        found_item.fill_raw(item.clone());
                                        found_page.fill(page);
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    None => unreachable!(
                        "Broken Page structure; Failed to find page {}",
                        dependency.hash
                    ),
                }
            }

            if found_item.found() {
                break;
            }
            options.increase_level();
        }
    }

    ItemSearchResult {
        found_item,
        found_page,
    }
}
