use crate::processors::items;
use ellie_core::{com, defs, error};
use serde::{Deserialize, Serialize};

pub struct TokenizerOptions {
    pub functions: bool,
    pub variables: bool,
    pub classes: bool,
    pub imports: bool,
}

#[derive(Default, Debug, Clone)]

pub struct Page {
    pub hash: u64,
    pub path: String,
    pub items: Vec<items::Processors>,
    pub dependents: Vec<u64>,
    pub dependencies: Vec<u64>,
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
    pub iterator: crate::iterator::Iterator,
}

impl Tokenizer {
    pub fn new(code: String) -> Self {
        Tokenizer {
            code: code,
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
            Err(self.iterator.errors.clone())
        } else {
            Ok(self.iterator.collected.clone())
        }
    }
}

pub struct Pager<E> {
    pub main: String,
    pub pages: Vec<Page>,
    pub current_page: u64,
    pub import_resolver: E,
}

impl<E> Pager<E>
where
    E: FnMut(String, String) -> ResolvedImport + Clone + Sized, //Path, filename
{
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

    pub fn resolve_page(&mut self, cr_page: u64, code: String) -> Result<u64, Vec<error::Error>> {
        let mut tokenizer = Tokenizer::new(code);
        let tokenized_main = tokenizer.tokenize_page();
        let page = self
            .pages
            .iter()
            .find(|p| p.hash == cr_page)
            .unwrap_or_else(|| panic!("Failed to resolve page: {}", cr_page))
            .clone();

        match tokenized_main {
            Ok(tokenized) => {
                let mut errors = Vec::new();
                let imports = tokenized
                    .clone()
                    .into_iter()
                    .filter(|x| x.as_import().clone().is_some())
                    .collect::<Vec<_>>();
                for import_processor in imports {
                    let import = import_processor.as_import().unwrap();
                    let resolved = (self.import_resolver)(page.path.clone(), import.path.clone());
                    if resolved.found {
                        if let Some(current_page_index) =
                            self.pages.iter().position(|p| p.hash == resolved.hash)
                        {
                            self.pages[current_page_index].dependents.push(cr_page);
                        } else {
                            self.pages.push(Page {
                                hash: resolved.hash,
                                path: resolved.path,
                                items: vec![],
                                dependents: vec![cr_page],
                                dependencies: vec![],
                            });
                            match self.resolve_page(resolved.hash, resolved.code) {
                                Err(e) => errors.extend(e),
                                _ => (),
                            }
                        }

                        if let Some(cpx) = self.pages.iter().position(|p| p.hash == cr_page) {
                            self.pages[cpx].dependencies.push(resolved.hash as u64);
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
                if let Some(cpx) = self.pages.iter().position(|p| p.hash == cr_page) {
                    self.pages[cpx].items = tokenized;
                }
                if errors.is_empty() {
                    Ok(cr_page)
                } else {
                    Err(errors)
                }
            }
            Err(errors) => Err(errors),
        }
    }

    pub fn run(&mut self) -> Result<u64, Vec<ellie_core::error::Error>> {
        self.resolve_page(0, self.main.clone())
    }
}
