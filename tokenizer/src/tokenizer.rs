use crate::processors::items;
use ellie_core::{
    defs, error,
    utils::{ExportPage, PageExport},
};
use serde::{Deserialize, Serialize};

/// TokenizerOptions
/// This struct contains all the options that can be set for the tokenizer
pub struct TokenizerOptions {
    pub functions: bool,
    pub variables: bool,
    pub classes: bool,
    pub imports: bool,
}

///Dependency
///Dependency is a link to another `ellie_tokenizer::Page` which helps us to resolve project hierarchy
#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub hash: usize,
    pub processed: bool,
    pub module: Option<usize>,
    pub deep_link: Option<usize>,
    pub public: bool,
}

/// `PageType` is gives us hint about the type of the page
/// Check [`Page`]
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum PageType {
    FunctionBody,
    ConstructorBody,
    RawBody,
    LoopBody,
    ClassBody,
    ValueConditionBody,
}

impl Default for PageType {
    fn default() -> Self {
        PageType::RawBody
    }
}

/// Page is a miror of a imported file which waits to be processed
/// # Fields
/// * `hash` - A unique hash of the page
/// * `inner` - A flag that indicates if the page is a inner page of function or class or maybe if block
/// * `processed` - A flag that indicates if the page has been processed
/// * `module` - A identifier that tells pager if its a pre-built module
/// * `unreachable` - A flag that indicates if the page is on unreachable code
/// * `unreachable_range` - A range of the unreachable code
/// * `page_type` - The type of the page (function, class, if block, etc) see [`PageType`]
/// * `path` - The path of the page
/// * `items` - A list of processed language items that are on the page
/// * `dependents` - A list of pages that depend on this page
/// * `dependencies` - A list of dependencies of the page use
///
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub hash: usize,
    pub inner: Option<usize>,
    pub processed: bool,
    pub module: bool,
    pub unreachable: bool,
    pub unreachable_range: defs::Cursor,
    pub page_type: PageType,
    pub path: String,
    pub items: Vec<items::Processors>,
    pub dependents: Vec<usize>,
    pub dependencies: Vec<Dependency>,
}

impl ExportPage for Page {
    fn get_hash(&self) -> usize {
        self.hash
    }
}

impl Page {
    /// Check page contains dependency
    /// ## Arguments
    /// * `hash` - A hash of the dependency
    /// ## Returns
    /// * `true` - If the page contains the dependency
    /// * `false` - If the page does not contain the dependency
    pub fn contains_dependency(&self, hash: usize) -> bool {
        self.dependencies
            .iter()
            .position(|x| x.hash == hash)
            .is_some()
    }
}

/// `ImportType` is a type that represents either Code(String) or pre-built module Module([`Module`]).
#[derive(Debug)]
pub enum ImportType {
    Code(String),
    Module(Module),
}

impl Default for ImportType {
    fn default() -> Self {
        ImportType::Code(String::new())
    }
}
/// ResolvedImport
/// A struct that contains all the information about an imported file
/// # Fields
/// * `found` - A boolean that indicates if the import was found
/// * `resolve_error` - A custom error given by import_resolver see [`Pager`]
/// * `hash` - A unique hash of the imported file
/// * `path` - The path of the imported file
/// * `matched` - The type of the import see [`ImportType`]
///
#[derive(Default, Debug)]
pub struct ResolvedImport {
    pub found: bool,
    pub resolve_error: String,
    pub hash: usize,
    pub path: String,
    pub matched: ImportType,
}

/// `Tokenizer` struct is used for building [`crate::iterator::Iterator`] interface
/// * Warning: This implementation not meant to be used by high end side. This is the low level of tokenizer, check [`Pager`] instead
/// ## Fields
/// * `code` - A string that contains the code to be tokenized [`String`]
/// * `path` - A string that contains the path of the code [`String`]
/// * `iterator` - [`crate::iterator::Iterator`] interface
pub struct Tokenizer {
    pub code: String,
    pub path: String,
    pub iterator: crate::iterator::Iterator,
}

impl Tokenizer {
    /// ### Create a new tokenizer
    /// [`Tokenizer`] is a base implementation of [`Iterator`], which iterates through the code
    /// ## Arguments
    /// * `code` - The code to tokenize
    /// * `path` - The path of the file
    /// ## Returns
    /// A new tokenizer [`Tokenizer`] instance
    pub fn new(code: String, path: String) -> Self {
        Tokenizer {
            code,
            path,
            iterator: crate::iterator::Iterator::default(),
        }
    }

    /// `tokenize_page` is a function initalizes [`Iterator`] and iters through the code
    pub fn tokenize_page(&mut self) -> Result<&mut Vec<items::Processors>, Vec<error::Error>> {
        let mut last_char = '\0';
        for letter_char in self.code.chars() {
            let hang = self.iterator.iterate(last_char, letter_char);
            last_char = letter_char;
            if hang {
                break;
            }
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

/// `Module` is a struct that contains all the information about a pre-built modules
/// # Fields
/// * `hash` - A unique hash of the module
/// * `initial_page` - A hash of the initial page of the module
/// * `version` - The version of the module presented by [`defs::Version`]
/// * `name` - Name of the module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub hash: usize,
    pub initial_page: usize,
    pub version: ellie_core::defs::Version,
    pub name: String,
}

/// `Pager` is a struct for implementing **Low Level** [`Tokenizer`] interface, and handles modulation and importation.
/// * Do not use this struct directly, use [`Pager::new`] instead
#[derive(Debug, Clone)]
pub struct Pager<E> {
    pub main: String,
    pub main_path: String,
    pub pages: PageExport<Page>,
    pub modules: Vec<Module>,
    pub current_page: usize,
    pub import_resolver: E,
}

/// RawPages
/// This is a duplicate of [`Page`] but without code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawPage {
    pub hash: usize,
    pub path: String,
    pub dependents: Vec<usize>,
    pub dependencies: Vec<Dependency>,
}

impl<E> Pager<E>
where
    E: FnMut(bool, String, String) -> ResolvedImport, //Path, filename
{
    /// Find page by hash
    /// ## Arguments
    /// * `hash` - page hash
    /// ## Returns
    /// Option<&mut [`Page`]>
    /// ## Example
    /// ```
    /// let pre_constructed_pager = ellie_tokenizer::Pager::new(...);
    /// match pre_constructed_pager.find_page_by_hash(0x12345678) {
    ///   Some(page) => {
    ///       // do something with page
    ///   },
    ///   None => {
    ///      // Page not found
    ///   },
    /// };
    /// ```
    pub fn find_page(&mut self, hash: usize) -> Option<&mut Page> {
        self.pages.find_page(hash)
    }

    /// Find module by hash
    /// ## Arguments
    /// * `hash` - module hash
    /// ## Returns
    /// Option<&mut [`Module`]>
    /// ## Example
    /// ```
    /// let pre_constructed_pager = ellie_tokenizer::Pager::new(...);
    /// match pre_constructed_pager.find_module_by_hash(0x12345678) {
    ///  Some(module) => {
    ///     // do something with module
    ///  },
    ///  None => {
    ///   // Module not found
    ///  },
    /// };
    /// ```
    pub fn find_module(&mut self, hash: usize) -> Option<&mut Module> {
        self.modules.iter_mut().find(|module| module.hash == hash)
    }

    /// ## Arguments
    /// * `main` - Main file
    /// * `main_path` - Main file path for example; If main is "C:\Users\User\Desktop\Project\main.ei" then main_path is "C:\Users\User\Desktop\Project"
    /// * `import_resolver` - Pager's query mechanism for resolving imports see [`Pager`] for more info
    /// * `initial_hash` - Initial page hash; Recommended to be None
    /// ## Example
    /// ```
    /// let mut pager = ellie_tokenizer::Pager::new("C:\\Users\\User\\Desktop\\Project\\main.ei", "C:\\Users\\User\\Desktop\\Project", |path, filename| {
    ///   // Example resolver located here 'NotReadyYet'
    ///   // do something with path and filename usaly to resolve imports you should use `ellie_core::module_path::ModulePath`
    ///   // return a ResolvedImport
    /// }, None);
    ///
    pub fn new(
        main: String,
        main_file_name: String,
        path: String,
        import_resolver: E,
        initial_hash: usize,
    ) -> Self {
        Pager {
            main: main,
            main_path: path.clone(),
            pages: PageExport {
                pages: vec![Page {
                    hash: initial_hash,
                    inner: None,
                    path: path + &main_file_name,
                    processed: false,
                    module: false,
                    items: vec![],
                    dependents: vec![],
                    dependencies: vec![],
                    page_type: PageType::RawBody,
                    unreachable: false,
                    unreachable_range: defs::Cursor::default(),
                }],
                page_hashs: (vec![0], vec![initial_hash]),
            },
            current_page: initial_hash,
            import_resolver,
            modules: vec![],
        }
    }

    /// Tokenize a page
    /// **Note: this is a inner function and should not be called directly**
    /// ## Arguments
    /// * `page` - Existing page hash
    /// * `code` - Code to tokenize
    /// ## Returns
    /// If successful returns a vector of used inner pages [`Dependency`] (Dependencies) or vector of syntax errors ([`error::Error`]) from tokenized page
    /// ## Example
    /// ```
    /// let mut pager = ellie_tokenizer::Pager::new("C:\\Users\\User\\Desktop\\Project\\main.ei", "C:\\Users\\User\\Desktop\\Project", |path, filename| {
    ///  // TODO: Implement import resolver
    ///  ellie_tokenizer::ResolvedImport::default()
    /// }, None);
    /// let page_hash = 0x12345678;
    /// let code = "v test : string = \"test\";";
    /// match pager.tokenize_page(page_hash, code) {
    ///     Ok(dependencies) => {
    ///         // do something with dependencies
    ///     },
    ///     Err(errors) => {
    ///         // do something with errors
    ///     }
    /// };
    /// ```
    pub fn resolve_page(
        &mut self,
        cr_page: usize,
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

                for import in imports {
                    let resolved = (self.import_resolver)(
                        import.link_module,
                        page.path.clone(),
                        import.path.trim().to_string(),
                    );
                    if resolved.found {
                        import.hash = resolved.hash;
                        let current_page = self.find_page(cr_page).unwrap();

                        current_page.dependencies.push(Dependency {
                            hash: resolved.hash,
                            processed: false,
                            module: match &resolved.matched {
                                ImportType::Code(_) => None,
                                ImportType::Module(x) => Some(x.initial_page.clone()),
                            },
                            deep_link: None,
                            public: import.public,
                        });
                        let dependents = current_page.dependents.clone();

                        let mut fullfiled_depenents = vec![];

                        if import.public {
                            for dependent in dependents {
                                fullfiled_depenents.push(dependent);
                                let found_dependent = self.find_page(dependent.clone()).unwrap();

                                if !found_dependent.contains_dependency(resolved.hash)
                                    && resolved.hash != found_dependent.hash
                                {
                                    found_dependent.dependencies.push(Dependency {
                                        hash: resolved.hash,
                                        processed: false,
                                        module: match &resolved.matched {
                                            ImportType::Code(_) => None,
                                            ImportType::Module(x) => Some(x.initial_page),
                                        },
                                        deep_link: Some(cr_page),
                                        public: import.public,
                                    });
                                }
                            }
                        }
                        fullfiled_depenents.push(cr_page);

                        match resolved.matched {
                            ImportType::Code(code_str) => match self.find_page(resolved.hash) {
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
                                    self.pages.push_page(Page {
                                        inner: None,
                                        hash: resolved.hash,
                                        path: resolved.path,
                                        processed: false,
                                        module: false,
                                        items: Vec::new(),
                                        dependents: fullfiled_depenents,
                                        dependencies: vec![],
                                        page_type: PageType::RawBody,
                                        unreachable: false,
                                        unreachable_range: defs::Cursor::default(),
                                    });
                                    match self.resolve_page(resolved.hash, code_str) {
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
                            },
                            ImportType::Module(module) => match self.find_module(resolved.hash) {
                                Some(_) => {}
                                None => {
                                    self.modules.push(module);
                                }
                            },
                        }
                    } else {
                        if resolved.resolve_error == "" {
                            errors.push(error::error_list::ERROR_S28.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: import.path.clone(),
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                page.path.clone(),
                                import.pos,
                            ));
                        } else {
                            errors.push(error::error_list::ERROR_S32.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: resolved.resolve_error,
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                page.path.clone(),
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

    /// Run tokenizing process. Start resolving_pages from first page defined in `Pager::new`, and recursively resolve all dependencies.
    /// ## Example
    /// ```
    /// use ellie_tokenizer::Pager;
    /// let mut pager = Pager::new(...);
    /// match pager.run() {
    ///     Ok(raw_pages) => {
    ///        // do something with `Vec<ellie_tokenizer::RawPage>`
    ///     },
    ///     Err(errors) => {
    ///         // do something with errors
    ///     }    
    /// }
    /// ```
    pub fn run(&mut self) -> Result<(), Vec<ellie_core::error::Error>> {
        match self.resolve_page(self.current_page, self.main.clone()) {
            Ok(e) => {
                self.find_page(self.current_page)
                    .unwrap()
                    .dependencies
                    .extend(e);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
