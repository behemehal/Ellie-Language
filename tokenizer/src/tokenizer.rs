use crate::processors::items;
use ellie_core::{defs, error};
use serde::{Deserialize, Serialize};

pub struct TokenizerOptions {
    pub functions: bool,
    pub variables: bool,
    pub classes: bool,
    pub imports: bool,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub hash: u64,
    pub processed: bool,
    pub public: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum PageType {
    FunctionBody,
    ConstructorBody,
    RawBody,
    ClassBody,
    ValueConditionBody,
}

impl Default for PageType {
    fn default() -> Self {
        PageType::RawBody
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]

pub struct Page {
    pub hash: u64,
    pub inner: Option<u64>,
    pub processed: bool,
    pub unreachable: bool,
    pub unreachable_range: defs::Cursor,
    pub page_type: PageType,
    pub path: String,
    pub items: Vec<items::Processors>,
    pub dependents: Vec<u64>,
    pub dependencies: Vec<Dependency>,
}

impl Page {
    pub fn contains_dependency(&self, hash: u64) -> bool {
        self.dependencies
            .iter()
            .position(|x| x.hash == hash)
            .is_some()
    }
}

#[derive(Default, Debug)]

pub struct ResolvedImport {
    pub found: bool,
    pub resolve_error: String,
    pub hash: u64,
    pub path: String,
    pub code: String,
}

pub struct Tokenizer {
    pub code: String,
    pub path: String,
    pub iterator: crate::iterator::Iterator,
}

impl Tokenizer {
    pub fn new(code: String, path: String) -> Self {
        Tokenizer {
            code,
            path,
            iterator: crate::iterator::Iterator::default(),
        }
    }

    pub fn tokenize_page(&mut self) -> Result<&mut Vec<items::Processors>, Vec<error::Error>> {
        let mut last_char = '\0';
        for letter_char in self.code.chars() {
            self.iterator.iterate(last_char, letter_char);
            last_char = letter_char;
        }
        self.iterator.finalize();
        if !self.iterator.errors.is_empty() {
            for i in &mut self.iterator.errors {
                if i.code == 0x25 {
                    i.pos.range_start.1 = 0;
                }
                i.path = self.path.clone();
                if let Some(ref mut ref_block) = i.reference_block {
                    if ref_block.1 == "<fill>" {
                        ref_block.1 = self.path.clone()
                    }
                }
            }
            Err(self.iterator.errors.clone())
        } else {
            Ok(&mut self.iterator.collected)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pager<E> {
    pub main: String,
    pub pages: Vec<Page>,
    pub current_page: u64,
    pub import_resolver: E,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawPage {
    pub hash: u64,
    pub path: String,
    pub dependents: Vec<u64>,
    pub dependencies: Vec<Dependency>,
}

impl<E> Pager<E>
where
    E: FnMut(String, String) -> ResolvedImport + Clone + Sized, //Path, filename
{
    pub fn find_page(&mut self, hash: u64) -> Option<&mut Page> {
        self.pages.iter_mut().find(|page| page.hash == hash)
    }

    pub fn new(main: String, path: String, import_resolver: E, initial_hash: Option<u64>) -> Self {
        Pager {
            main: main,
            pages: vec![Page {
                hash: initial_hash.unwrap_or(0),
                inner: None,
                path,
                processed: false,
                items: vec![],
                dependents: vec![],
                dependencies: vec![],
                page_type: PageType::RawBody,
                unreachable: false,
                unreachable_range: defs::Cursor::default(),
            }],
            current_page: initial_hash.unwrap_or(0),
            import_resolver: import_resolver,
        }
    }

    pub fn resolve_page(
        &mut self,
        cr_page: u64,
        code: String,
    ) -> Result<Vec<Dependency>, Vec<error::Error>> {
        let page = self.find_page(cr_page).unwrap().clone();
        let mut tokenizer = Tokenizer::new(code, page.path.clone());
        match tokenizer.tokenize_page() {
            Ok(tokenized) => {
                let mut errors = Vec::new();
                let mut data: Vec<Dependency> = Vec::new();
                let imports = tokenized
                    .iter_mut()
                    .filter_map(|f| match f {
                        items::Processors::Import(i) => Some(i),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                let mut dependencies = vec![];

                for import in imports {
                    let resolved = (self.import_resolver)(page.path.clone(), import.path.clone());
                    if resolved.found {
                        import.hash = resolved.hash.to_string();

                        let current_page = self.find_page(cr_page).unwrap();

                        current_page.dependencies.push(Dependency {
                            hash: resolved.hash,
                            processed: false,
                            public: import.public,
                        });
                        let dependents = current_page.dependents.clone();

                        let mut fullfiled_depenents = vec![];

                        if import.public {
                            for dependent in dependents {
                                fullfiled_depenents.push(dependent);
                                let found_dependent = self.find_page(dependent.clone()).unwrap();

                                if !found_dependent.contains_dependency(resolved.hash) {
                                    found_dependent.dependencies.push(Dependency {
                                        hash: resolved.hash,
                                        processed: false,
                                        public: import.public,
                                    });
                                }
                            }
                        }
                        fullfiled_depenents.push(cr_page);

                        match self.find_page(resolved.hash) {
                            Some(inner_child) => {
                                inner_child.dependents.extend(fullfiled_depenents);
                                let public_dependencies = inner_child
                                    .dependencies
                                    .clone()
                                    .into_iter()
                                    .clone()
                                    .filter(|d| d.public)
                                    .collect::<Vec<_>>();
                                data.extend(public_dependencies);
                            }
                            None => {
                                self.pages.push(Page {
                                    inner: None,
                                    hash: resolved.hash,
                                    path: resolved.path,
                                    processed: false,
                                    items: Vec::new(),
                                    dependents: fullfiled_depenents,
                                    dependencies: dependencies.clone(),
                                    page_type: PageType::RawBody,
                                    unreachable: false,
                                    unreachable_range: defs::Cursor::default(),
                                });
                                match self.resolve_page(resolved.hash, resolved.code) {
                                    Ok(inner_child) => {
                                        let public_dependencies = inner_child
                                            .into_iter()
                                            .clone()
                                            .filter(|d| d.public)
                                            .collect::<Vec<_>>();
                                        data.extend(public_dependencies);
                                    }
                                    Err(e) => errors.extend(e),
                                }
                            }
                        }
                    } else {
                        if resolved.resolve_error == "" {
                            errors.push(error::error_list::ERROR_S28.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: import.path.clone(),
                                }],
                                "tok_0x185".to_owned(),
                                import.pos,
                            ));
                        } else {
                            errors.push(error::error_list::ERROR_S32.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: resolved.resolve_error,
                                }],
                                "tok_0x194".to_owned(),
                                import.pos,
                            ));
                        }
                    }
                }
                self.find_page(cr_page).unwrap().items = tokenized.clone();
                if errors.len() > 0 {
                    Err(errors)
                } else {
                    Ok(data)
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn run(&mut self) -> Result<Vec<RawPage>, Vec<ellie_core::error::Error>> {
        match self.resolve_page(self.current_page, self.main.clone()) {
            Ok(e) => {
                self.find_page(self.current_page)
                    .unwrap()
                    .dependencies
                    .extend(e);
                Ok(self
                    .pages
                    .clone()
                    .into_iter()
                    .map(|x| RawPage {
                        hash: x.hash,
                        path: x.path,
                        dependents: x.dependents,
                        dependencies: x.dependencies,
                    })
                    .collect::<Vec<_>>())
            }
            Err(e) => Err(e),
        }
    }
}
