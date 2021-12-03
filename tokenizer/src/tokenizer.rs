use crate::processors::items;
use ellie_core::{com, defs, error};
use serde::{Deserialize, Serialize};

pub struct TokenizerOptions {
    pub functions: bool,
    pub variables: bool,
    pub classes: bool,
    pub imports: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub hash: u64,
    pub public: bool,
}

#[derive(Default, Debug, Clone)]

pub struct Page {
    pub hash: u64,
    pub path: String,
    pub items: Vec<items::Processors>,
    pub dependents: Vec<u64>,
    pub dependencies: Vec<Dependency>,
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

    pub fn tokenize_page(&mut self) -> Result<Vec<items::Processors>, Vec<error::Error>> {
        let mut last_char = '\0';
        for letter_char in self.code.chars() {
            self.iterator.iterate(last_char, letter_char);
            last_char = letter_char;
        }
        self.iterator.finalize();
        if !self.iterator.errors.is_empty() {
            for i in &mut self.iterator.errors {
                i.path = self.path.clone();
            }
            Err(self.iterator.errors.clone())
        } else {
            Ok(self.iterator.collected.clone())
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

    pub fn new(main: String, path: String, import_resolver: E) -> Self {
        Pager {
            main: main,
            pages: vec![Page {
                hash: 0,
                path,
                items: vec![],
                dependents: vec![],
                dependencies: vec![],
            }],
            current_page: 0,
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
                    .clone()
                    .into_iter()
                    .filter_map(|f| match f {
                        items::Processors::Import(i) => Some(i),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                for import in imports {
                    let resolved = (self.import_resolver)(page.path.clone(), import.path.clone());
                    if resolved.found {
                        self.find_page(cr_page)
                            .unwrap()
                            .dependencies
                            .push(Dependency {
                                hash: resolved.hash,
                                public: import.public,
                            });

                        match self.find_page(resolved.hash) {
                            Some(inner_child) => {
                                inner_child.dependents.push(cr_page);
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
                                    hash: resolved.hash,
                                    path: resolved.path,
                                    items: Vec::new(),
                                    dependents: vec![cr_page],
                                    dependencies: Vec::new(),
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
                            errors.push(error::errorList::error_s28.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: import.path.clone(),
                                }],
                                "cond_0x57".to_owned(),
                                import.pos,
                            ));
                        } else {
                            errors.push(error::errorList::error_s32.clone().build(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: resolved.resolve_error,
                                }],
                                "tok_0x118".to_owned(),
                                import.pos,
                            ));
                        }
                    }
                }
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
        match self.resolve_page(0, self.main.clone()) {
            Ok(e) => {
                self.find_page(0).unwrap().dependencies.extend(e);
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
