use crate::deep_search_extensions::{self, resolve_deep_type, resolve_type};
use crate::processors::Processor;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::{string::String, vec};
use ellie_core::definite::definers::DefinerCollecting;
use ellie_core::definite::items::file_key::FileKey;
use ellie_core::definite::types::class_instance::{self, Attribute, AttributeType, ClassInstance};
use ellie_core::definite::{items::Collecting, Converter};
use ellie_core::defs::Cursor;
use ellie_core::utils::{ExportPage, PageExport};
#[cfg(feature = "standard_rules")]
use ellie_core::warning;
use ellie_core::{defs, error, information};
use ellie_tokenizer::processors::items::Processors;
use ellie_tokenizer::syntax::items::condition::ConditionType;
use ellie_tokenizer::tokenizer::{Dependency, Page, PageType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessedPage {
    pub hash: usize,
    pub inner: Option<usize>,
    pub unassigned_file_keys: Vec<FileKey>,
    pub global_file_keys: Vec<FileKey>,
    pub path: String,
    pub page_type: PageType,
    pub items: Vec<ellie_core::definite::items::Collecting>,
    pub dependents: Vec<usize>,
    pub dependencies: Vec<ellie_tokenizer::tokenizer::Dependency>,
}

impl ExportPage for ProcessedPage {
    fn get_hash(&self) -> usize {
        self.hash
    }
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

    pub fn find_item_by_hash(
        &self,
        hash: usize,
    ) -> Option<ellie_core::definite::items::Collecting> {
        for i in &self.items {
            if matches!(i.get_hash(), Some(e) if e == hash) {
                return Some(i.clone());
            }
        }
        None
    }

    pub fn generate_instance(&self) -> ClassInstance {
        let class_page = match &self.page_type {
            PageType::ClassBody(class_page) => class_page,
            _ => {
                panic!("Cannot generate instance from non-class body");
            }
        };

        let attributes = self
            .items
            .iter()
            .filter_map(|i| match i {
                Collecting::Variable(variable) => Some(Attribute {
                    _rtype: AttributeType::Property,
                    name: variable.name.clone(),
                    page: self.hash,
                    hash: variable.hash,
                    class_hash: class_page.hash,
                }),
                Collecting::Function(function) => Some(Attribute {
                    _rtype: AttributeType::Property,
                    name: function.name.clone(),
                    page: self.hash,
                    hash: function.hash,
                    class_hash: class_page.hash,
                }),
                Collecting::Getter(getter) => Some(Attribute {
                    _rtype: AttributeType::Property,
                    name: getter.name.clone(),
                    page: self.hash,
                    hash: getter.hash,
                    class_hash: class_page.hash,
                }),
                Collecting::Setter(setter) => Some(Attribute {
                    _rtype: AttributeType::Property,
                    name: setter.name.clone(),
                    page: self.hash,
                    hash: setter.hash,
                    class_hash: class_page.hash,
                }),
                Collecting::NativeFunction(function) => Some(Attribute {
                    _rtype: AttributeType::Property,
                    name: function.name.clone(),
                    page: self.hash,
                    hash: function.hash,
                    class_hash: class_page.hash,
                }),
                Collecting::SelfItem(_) => todo!(),
                _ => None,
            })
            .collect();

        ClassInstance {
            class_name: class_page.name.clone(),
            class_hash: class_page.hash,
            class_page: class_page.page_hash,
            attributes,
        }
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
    pub hash: usize,
    pub name: String,
    pub description: String,
    pub initial_page: usize,
    pub is_library: bool,
    pub ellie_version: ellie_core::defs::Version,
    pub pages: PageExport<ProcessedPage>,
    pub version: ellie_core::defs::Version,
    pub modules: Vec<ellie_tokenizer::tokenizer::Module>,
}

pub struct ParserSettings {
    pub dynamics: (bool, String),
    pub nullables: (bool, String),
    pub integers: (bool, String),
    pub floats: (bool, String),
    pub doubles: (bool, String),
    pub strings: (bool, String),
    pub chars: (bool, String),
    pub booleans: (bool, String),
    pub arrays: (bool, String),
    pub cloaks: (bool, String),
    pub enums: (bool, String),
    pub collectives: (bool, String),
    pub type_conversions: (bool, String),
    pub force_types: (bool, String),
}

impl Default for ParserSettings {
    fn default() -> Self {
        ParserSettings {
            dynamics: (true, String::new()),
            nullables: (true, String::new()),
            integers: (true, String::new()),
            floats: (true, String::new()),
            doubles: (true, String::new()),
            strings: (true, String::new()),
            chars: (true, String::new()),
            booleans: (true, String::new()),
            arrays: (true, String::new()),
            cloaks: (true, String::new()),
            enums: (true, String::new()),
            collectives: (true, String::new()),
            type_conversions: (true, String::new()),
            force_types: (false, String::new()),
        }
    }
}

impl ParserSettings {
    pub fn is_type_allowed(
        &self,
        type_: ellie_tokenizer::processors::types::Processors,
    ) -> (bool, String) {
        match type_ {
            ellie_tokenizer::processors::types::Processors::Decimal(decimal_type) => {
                if decimal_type.data.is_double {
                    self.floats.clone()
                } else {
                    self.doubles.clone()
                }
            }
            ellie_tokenizer::processors::types::Processors::Char(_) => self.chars.clone(),
            ellie_tokenizer::processors::types::Processors::String(_) => self.strings.clone(),
            ellie_tokenizer::processors::types::Processors::Array(_) => self.arrays.clone(),
            ellie_tokenizer::processors::types::Processors::BraceReference(_) => {
                self.nullables.clone()
            }
            ellie_tokenizer::processors::types::Processors::Cloak(_) => self.cloaks.clone(),
            ellie_tokenizer::processors::types::Processors::Collective(_) => {
                self.collectives.clone()
            }
            ellie_tokenizer::processors::types::Processors::AsKeyword(_) => {
                self.type_conversions.clone()
            }
            _ => (true, String::new()),
        }
    }

    pub fn is_item_allowed(&self, _type_: Processors) -> bool {
        true
    }
}

pub struct ModuleInfo {
    pub name: String,
    pub description: String,
    pub is_lib: bool,
    pub version: ellie_core::defs::Version,
    pub ellie_version: ellie_core::defs::Version,
}

pub struct Parser {
    pub pages: PageExport<Page>,
    pub processed_pages: PageExport<ProcessedPage>,
    pub modules: Vec<Module>,
    pub initial_page: usize,
    pub experimental_features: bool,
    pub informations: information::Informations,
    pub parser_settings: ParserSettings,
    pub module_info: ModuleInfo,
}

#[derive(Debug, Clone, Default)]
pub struct FoundPage {
    pub hash: usize,
    pub inner: Option<usize>,
    pub processed: bool,
    pub module: bool,
    pub path: String,
}

impl FoundPage {
    pub fn fill(page: &Page) -> FoundPage {
        FoundPage {
            hash: page.hash,
            inner: page.inner,
            processed: page.processed,
            module: page.module,
            path: page.path.clone(),
        }
    }

    pub fn fill_from_processed(page: &ProcessedPage) -> FoundPage {
        FoundPage {
            hash: page.hash,
            inner: page.inner,
            processed: true,
            module: false,
            path: page.path.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeepSearchResult {
    pub found: bool,
    pub found_item: DeepSearchItems,
    pub found_pos: Option<defs::Cursor>,
    pub found_page: FoundPage,
}

#[derive(Debug, Clone)]
pub enum DeepSearchItems {
    Class(ellie_tokenizer::syntax::items::class::Class),
    Variable(ellie_tokenizer::syntax::items::variable::Variable),
    Getter(ellie_tokenizer::syntax::items::getter::Getter),
    Setter(ellie_tokenizer::syntax::items::setter::Setter),
    Function(ellie_tokenizer::syntax::items::function::Function),
    Enum(ellie_tokenizer::syntax::items::enum_type::EnumType),
    ImportReference(ellie_tokenizer::syntax::items::import::Import),
    ClassInstance(class_instance::ClassInstance),
    GenericItem(ellie_tokenizer::syntax::items::generic_item::GenericItem),
    FunctionParameter(ellie_tokenizer::syntax::items::function_parameter::FunctionParameter),
    ConstructorParameter(
        ellie_tokenizer::syntax::items::constructor_parameter::ConstructorParameter,
    ),
    SelfItem(ellie_core::definite::items::self_item::SelfItem),
    BrokenPageGraph,
    MixUp(Vec<(String, String)>),
    None,
}

#[derive(Debug)]
pub struct CompareResult {
    pub same: bool,
    pub first: String,
    pub second: String,
    pub requires_cast: bool,
    pub cast_to: String,
}

impl CompareResult {
    pub fn result(same: bool, first: String, second: String) -> CompareResult {
        CompareResult {
            same,
            first,
            second,
            requires_cast: false,
            cast_to: String::new(),
        }
    }

    pub fn result_with_cast(
        same: bool,
        first: String,
        second: String,
        cast_to: String,
    ) -> CompareResult {
        CompareResult {
            same,
            first,
            second,
            requires_cast: true,
            cast_to,
        }
    }
}

impl DeepSearchItems {
    pub fn get_pos(&self) -> defs::Cursor {
        match self {
            DeepSearchItems::Class(e) => e.pos,
            DeepSearchItems::Variable(e) => e.pos,
            DeepSearchItems::Function(e) => e.pos,
            DeepSearchItems::ImportReference(e) => e.pos,
            DeepSearchItems::Getter(e) => e.pos,
            DeepSearchItems::Setter(e) => e.pos,
            DeepSearchItems::Enum(e) => e.pos,
            DeepSearchItems::GenericItem(e) => e.pos,
            DeepSearchItems::ConstructorParameter(e) => e.pos,
            _ => defs::Cursor::default(),
        }
    }
}

impl Parser {
    pub fn new(
        pages: PageExport<Page>,
        initial_hash: usize,
        version: ellie_core::defs::Version,
        module_name: String,
        module_description: String,
        is_lib: bool,
        experimental_features: bool,
        ellie_version: defs::Version,
    ) -> Parser {
        Parser {
            pages,
            processed_pages: PageExport::new(),
            modules: vec![],
            initial_page: initial_hash,
            experimental_features,
            informations: information::Informations::new(),
            parser_settings: ParserSettings::default(),
            module_info: ModuleInfo {
                name: module_name,
                description: module_description,
                is_lib,
                version,
                ellie_version,
            },
        }
    }

    pub fn calculate_hash(&self) -> usize {
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
                module: if x.hash == module.initial_page {
                    None
                } else {
                    Some(module.initial_page)
                },
                deep_link: if x.hash == 343 { None } else { Some(343) },
                public: false,
            })
            .collect();

        self.find_page(self.initial_page)
            .unwrap()
            .dependencies
            .extend(imported_dependencies);
        self.pages.extend_pages(unprocessed_pages);
        self.processed_pages.extend_pages(module.pages.pages);
    }

    pub fn compare_defining_with_type(
        &mut self,
        defining: ellie_core::definite::definers::DefinerCollecting,
        rtype: ellie_core::definite::types::Types,
        target_page: usize,
    ) -> Result<CompareResult, Vec<error::Error>> {
        let mut errors: Vec<error::Error> = Vec::new();
        let found_type = crate::deep_search_extensions::resolve_deep_type(
            self,
            target_page,
            rtype.clone(),
            &mut errors,
        );

        //Ignre everything since defining is dynamic
        if matches!(&defining, ellie_core::definite::definers::DefinerCollecting::Generic(e) if e.rtype == "dyn")
        {
            return Ok(CompareResult::result(
                true,
                "dyn".to_string(),
                "dyn".to_string(),
            ));
        }

        match found_type {
            deep_search_extensions::DeepTypeResult::Integer(_) => {
                if let ellie_core::definite::definers::DefinerCollecting::Generic(_) = defining {
                    if defining.to_string() == "int" {
                        if errors.is_empty() {
                            Ok(CompareResult::result(
                                true,
                                defining.to_string(),
                                "int".to_owned(),
                            ))
                        } else {
                            Err(errors)
                        }
                    } else if errors.is_empty() {
                        Ok(CompareResult::result(
                            false,
                            defining.to_string(),
                            "int".to_owned(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        "int".to_owned(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::Byte(_) => {
                if let ellie_core::definite::definers::DefinerCollecting::Generic(_) = defining {
                    if defining.to_string() == "byte" {
                        if errors.is_empty() {
                            Ok(CompareResult::result(
                                true,
                                defining.to_string(),
                                "byte".to_owned(),
                            ))
                        } else {
                            Err(errors)
                        }
                    } else if errors.is_empty() {
                        Ok(CompareResult::result(
                            false,
                            defining.to_string(),
                            "byte".to_owned(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        "byte".to_owned(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::Decimal(decimal_type) => {
                let generic_name = if decimal_type.is_double {
                    "double"
                } else {
                    "float"
                };
                if let ellie_core::definite::definers::DefinerCollecting::Generic(_) = defining {
                    if defining.to_string() == generic_name {
                        if errors.is_empty() {
                            Ok(CompareResult::result(
                                true,
                                defining.to_string(),
                                generic_name.to_owned(),
                            ))
                        } else {
                            Err(errors)
                        }
                    } else if errors.is_empty() {
                        Ok(CompareResult::result(
                            false,
                            defining.to_string(),
                            generic_name.to_owned(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        generic_name.to_owned(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::Bool(_) => {
                if let ellie_core::definite::definers::DefinerCollecting::Generic(_) = defining {
                    if defining.to_string() == "bool" {
                        if errors.is_empty() {
                            Ok(CompareResult::result(
                                true,
                                defining.to_string(),
                                "bool".to_owned(),
                            ))
                        } else {
                            Err(errors)
                        }
                    } else if errors.is_empty() {
                        Ok(CompareResult::result(
                            false,
                            defining.to_string(),
                            "bool".to_owned(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        "bool".to_owned(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::String(_) => {
                if let ellie_core::definite::definers::DefinerCollecting::Generic(_) = defining {
                    if defining.to_string() == "string" {
                        if errors.is_empty() {
                            Ok(CompareResult::result(
                                true,
                                defining.to_string(),
                                "string".to_owned(),
                            ))
                        } else {
                            Err(errors)
                        }
                    } else if errors.is_empty() {
                        Ok(CompareResult::result(
                            false,
                            defining.to_string(),
                            "string".to_owned(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        "string".to_owned(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::Char(_) => {
                if let ellie_core::definite::definers::DefinerCollecting::Generic(_) = defining {
                    if defining.to_string() == "char" {
                        if errors.is_empty() {
                            Ok(CompareResult::result(
                                true,
                                defining.to_string(),
                                "char".to_owned(),
                            ))
                        } else {
                            Err(errors)
                        }
                    } else if errors.is_empty() {
                        Ok(CompareResult::result(
                            false,
                            defining.to_string(),
                            "char".to_owned(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        "char".to_owned(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::Collective(_) => todo!(),
            deep_search_extensions::DeepTypeResult::Operator(e) => {
                let value_gen = match deep_search_extensions::resolve_type(
                    rtype,
                    target_page,
                    self,
                    &mut errors,
                    Some(e.pos),
                ) {
                    Some(e) => e,
                    None => {
                        return Err(errors);
                    }
                };

                if value_gen.same_as(defining.clone()) {
                    if errors.is_empty() {
                        Ok(CompareResult::result(
                            true,
                            defining.to_string(),
                            value_gen.to_string(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        value_gen.to_string(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::Cloak(_) => todo!(),
            deep_search_extensions::DeepTypeResult::Array(e) => {
                let value_gen = match deep_search_extensions::resolve_type(
                    rtype,
                    target_page,
                    self,
                    &mut errors,
                    Some(e.pos),
                ) {
                    Some(e) => e,
                    None => {
                        return Err(errors);
                    }
                };

                if value_gen.same_as(defining.clone()) {
                    if errors.is_empty() {
                        Ok(CompareResult::result(
                            true,
                            defining.to_string(),
                            value_gen.to_string(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        value_gen.to_string(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::ClassCall(class_call) => {
                let class_call_type = match resolve_type(
                    ellie_core::definite::types::Types::ClassCall(class_call.clone()),
                    target_page,
                    self,
                    &mut errors,
                    Some(class_call.pos),
                ) {
                    Some(e) => e,
                    None => return Err(errors),
                };
                if errors.is_empty() {
                    if class_call_type.same_as(defining.clone()) {
                        Ok(CompareResult::result(
                            true,
                            defining.to_string(),
                            class_call_type.to_string(),
                        ))
                    } else {
                        Ok(CompareResult::result(
                            false,
                            defining.to_string(),
                            class_call_type.to_string(),
                        ))
                    }
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::FunctionCall(e) => {
                let resolved_target =
                    resolve_deep_type(self, target_page, *e.target.clone(), &mut errors);

                match resolved_target {
                    deep_search_extensions::DeepTypeResult::Function(e) => {
                        Ok(CompareResult::result(
                            defining.same_as(e.return_type.clone()),
                            defining.to_string(),
                            e.return_type.to_string(),
                        ))
                    }
                    deep_search_extensions::DeepTypeResult::FunctionCall(e) => {
                        Ok(CompareResult::result(
                            defining.same_as(e.returning.clone()),
                            defining.to_string(),
                            e.returning.to_string(),
                        ))
                    }
                    _ => {
                        let rtype = match resolved_target {
                            deep_search_extensions::DeepTypeResult::Integer(e) => {
                                ellie_core::definite::types::Types::Integer(e)
                            }
                            deep_search_extensions::DeepTypeResult::Byte(e) => {
                                ellie_core::definite::types::Types::Byte(e)
                            }
                            deep_search_extensions::DeepTypeResult::Decimal(e) => {
                                ellie_core::definite::types::Types::Decimal(e)
                            }
                            deep_search_extensions::DeepTypeResult::Bool(e) => {
                                ellie_core::definite::types::Types::Bool(e)
                            }
                            deep_search_extensions::DeepTypeResult::String(e) => {
                                ellie_core::definite::types::Types::String(e)
                            }
                            deep_search_extensions::DeepTypeResult::Char(e) => {
                                ellie_core::definite::types::Types::Char(e)
                            }
                            deep_search_extensions::DeepTypeResult::Collective(e) => {
                                ellie_core::definite::types::Types::Collective(e)
                            }
                            deep_search_extensions::DeepTypeResult::Operator(e) => {
                                ellie_core::definite::types::Types::Operator(e)
                            }
                            deep_search_extensions::DeepTypeResult::Cloak(e) => {
                                ellie_core::definite::types::Types::Cloak(e)
                            }
                            deep_search_extensions::DeepTypeResult::Array(e) => {
                                ellie_core::definite::types::Types::Array(e)
                            }
                            deep_search_extensions::DeepTypeResult::ClassCall(e) => {
                                ellie_core::definite::types::Types::ClassCall(e)
                            }
                            deep_search_extensions::DeepTypeResult::BraceReference(e) => {
                                ellie_core::definite::types::Types::BraceReference(e)
                            }
                            deep_search_extensions::DeepTypeResult::Void => {
                                ellie_core::definite::types::Types::Void
                            }
                            deep_search_extensions::DeepTypeResult::Null => {
                                ellie_core::definite::types::Types::Null
                            }
                            deep_search_extensions::DeepTypeResult::Dynamic => {
                                ellie_core::definite::types::Types::Dynamic
                            }
                            _ => unreachable!(),
                        };
                        let resolved_type =
                            resolve_type(rtype, target_page, self, &mut errors, Some(e.target_pos));
                        if errors.is_empty() {
                            Ok(CompareResult::result(
                                false,
                                defining.to_string(),
                                resolved_type.unwrap().to_string(),
                            ))
                        } else {
                            Err(errors)
                        }
                    }
                }
            }
            deep_search_extensions::DeepTypeResult::Void => {
                if let ellie_core::definite::definers::DefinerCollecting::Generic(_) = defining {
                    if defining.clone().to_string() == "void" {
                        if errors.is_empty() {
                            Ok(CompareResult::result(
                                true,
                                defining.to_string(),
                                "void".to_owned(),
                            ))
                        } else {
                            Err(errors)
                        }
                    } else if errors.is_empty() {
                        Ok(CompareResult::result(
                            false,
                            defining.to_string(),
                            "void".to_owned(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        "void".to_owned(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::Null => {
                if let ellie_core::definite::definers::DefinerCollecting::Generic(_) = defining {
                    if defining.to_string() == "null" {
                        if errors.is_empty() {
                            Ok(CompareResult::result(
                                true,
                                defining.to_string(),
                                "null".to_owned(),
                            ))
                        } else {
                            Err(errors)
                        }
                    } else if errors.is_empty() {
                        Ok(CompareResult::result(
                            false,
                            defining.to_string(),
                            "null".to_owned(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        "null".to_owned(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::BraceReference(e) => {
                let value_gen = match deep_search_extensions::resolve_type(
                    rtype,
                    target_page,
                    self,
                    &mut errors,
                    Some(e.pos),
                ) {
                    Some(e) => e,
                    None => return Err(errors),
                };

                if value_gen.same_as(defining.clone()) {
                    if errors.is_empty() {
                        Ok(CompareResult::result(
                            true,
                            defining.to_string(),
                            value_gen.to_string(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        value_gen.to_string(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::Dynamic => {
                if let ellie_core::definite::definers::DefinerCollecting::Generic(_) = defining {
                    if defining.to_string() == "dyn" {
                        if errors.is_empty() {
                            Ok(CompareResult::result(
                                true,
                                defining.to_string(),
                                "dyn".to_owned(),
                            ))
                        } else {
                            Err(errors)
                        }
                    } else if errors.is_empty() {
                        Ok(CompareResult::result(
                            false,
                            defining.to_string(),
                            "dyn".to_owned(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if let ellie_core::definite::definers::DefinerCollecting::Dynamic = defining
                {
                    if errors.is_empty() {
                        Ok(CompareResult::result(
                            true,
                            defining.to_string(),
                            "dyn".to_owned(),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        "dyn".to_owned(),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::NotFound => {
                if errors.is_empty() {
                    Ok(CompareResult::result(false, String::new(), String::new()))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::Function(function) => {
                if defining.as_function().is_some()
                    && function
                        .return_type
                        .same_as(*defining.as_function().unwrap().returning.clone())
                    && defining.as_function().unwrap().params.len() == function.parameters.len()
                    && function
                        .parameters
                        .iter()
                        .enumerate()
                        .find(|(index, e)| match e.rtype.clone() {
                            Some(rtype) => {
                                let def = defining.as_function().unwrap();
                                !def.params[*index].same_as(rtype)
                            }
                            None => true,
                        })
                        .is_none()
                {
                    if errors.is_empty() {
                        Ok(CompareResult::result(
                            true,
                            defining.to_string(),
                            format!(
                                "Fn({}):{}",
                                function
                                    .parameters
                                    .iter()
                                    .map(|x| match &x.rtype {
                                        Some(x) => x.to_string(),
                                        None => "?".to_string(),
                                    })
                                    .collect::<Vec<_>>()
                                    .join(","),
                                function.return_type.to_string()
                            ),
                        ))
                    } else {
                        Err(errors)
                    }
                } else if errors.is_empty() {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        format!(
                            "Fn({}):{}",
                            function
                                .parameters
                                .iter()
                                .map(|x| match &x.rtype {
                                    Some(x) => x.to_string(),
                                    None => "?".to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(","),
                            function.return_type.to_string()
                        ),
                    ))
                } else {
                    Err(errors)
                }
            }
            deep_search_extensions::DeepTypeResult::EnumData(e) => {
                let (enum_hash, enum_name) = match *e.reference {
                    ellie_core::definite::types::Types::VariableType(e) => (e.reference, e.value),
                    _ => unreachable!("Parser should have prevented this"),
                };

                match &e.value {
                    ellie_core::definite::types::enum_data::Pointer::NoData => {
                        if errors.is_empty() {
                            match defining {
                                ellie_core::definite::definers::DefinerCollecting::Generic(
                                    generic,
                                ) => Ok(CompareResult::result(
                                    generic.hash == enum_hash,
                                    generic.rtype,
                                    enum_name,
                                )),
                                _ => Ok(CompareResult::result(
                                    false,
                                    defining.to_string(),
                                    enum_name,
                                )),
                            }
                        } else {
                            Err(errors)
                        }
                    }
                    ellie_core::definite::types::enum_data::Pointer::Data(q) => {
                        let mut errors = Vec::new();
                        match resolve_type(*q.clone(), target_page, self, &mut errors, Some(e.pos))
                        {
                            Some(value_defining) => {
                                if value_defining.same_as(defining.clone()) {
                                    Ok(CompareResult::result(
                                        true,
                                        defining.to_string(),
                                        value_defining.to_string(),
                                    ))
                                } else {
                                    match defining {
                                        ellie_core::definite::definers::DefinerCollecting::Generic(
                                            generic,
                                        ) => Ok(CompareResult::result(
                                            generic.hash == enum_hash,
                                            generic.rtype,
                                            enum_name,
                                        )),
                                        _ => Ok(CompareResult::result(
                                            false,
                                            defining.to_string(),
                                            enum_name,
                                        )),
                                    }
                                }
                            }
                            None => Err(errors),
                        }
                    }
                }
            }
            deep_search_extensions::DeepTypeResult::Enum(_) => todo!(),
            deep_search_extensions::DeepTypeResult::ClassInstance(_) => todo!(),
            deep_search_extensions::DeepTypeResult::SelfItem(_) => todo!(),
        }
    }

    pub fn resolve_definer_name(
        &self,
        definer: ellie_core::definite::definers::DefinerCollecting,
    ) -> String {
        match definer {
            ellie_core::definite::definers::DefinerCollecting::Array(_) => "Array".to_string(),
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
            ellie_core::definite::definers::DefinerCollecting::Dynamic => "dyn".to_string(),
            ellie_core::definite::definers::DefinerCollecting::EnumField(e) => e.name,
            ellie_core::definite::definers::DefinerCollecting::ClassInstance(_) => todo!(),
        }
    }

    pub fn global_key_matches(&mut self, processed_page: usize, key: &str, value: &str) -> bool {
        match self.find_processed_page(processed_page) {
            Some(e) => e
                .global_file_keys
                .iter()
                .find(|x| {
                    x.key_name == key
                        && match &x.value {
                            ellie_core::definite::types::Types::String(e) => e.value == value,
                            _ => false,
                        }
                })
                .is_some(),
            None => false,
        }
    }

    pub fn is_variable_duplicate(
        &mut self,
        page_id: usize,
        name: String,
        hash: usize,
        pos: defs::Cursor,
    ) -> (bool, Option<(FoundPage, defs::Cursor)>) {
        let deep_search = deep_search_extensions::deep_search(
            self,
            page_id,
            name.clone(),
            if hash == 0 { None } else { Some(hash) },
            vec![],
            0,
        );

        if deep_search.found {
            match deep_search.found_item {
                deep_search_extensions::ProcessedDeepSearchItems::Variable(e) => {
                    if deep_search.found_page.hash == page_id {
                        (
                            pos.range_start.is_bigger(&e.pos.range_start),
                            Some((deep_search.found_page, e.pos)),
                        )
                    } else {
                        (false, None)
                    }
                }
                deep_search_extensions::ProcessedDeepSearchItems::GenericItem(e) => {
                    (true, Some((deep_search.found_page, e.pos)))
                }
                deep_search_extensions::ProcessedDeepSearchItems::None
                | deep_search_extensions::ProcessedDeepSearchItems::FunctionParameter(_)
                | deep_search_extensions::ProcessedDeepSearchItems::ClassInstance(_) => {
                    (false, None)
                }
                e => {
                    let pos = match e {
                        deep_search_extensions::ProcessedDeepSearchItems::Class(e) => e.pos,
                        deep_search_extensions::ProcessedDeepSearchItems::Variable(e) => e.pos,
                        deep_search_extensions::ProcessedDeepSearchItems::Function(e) => e.pos,
                        deep_search_extensions::ProcessedDeepSearchItems::Enum(e) => e.pos,
                        deep_search_extensions::ProcessedDeepSearchItems::NativeFunction(e) => {
                            e.pos
                        }
                        deep_search_extensions::ProcessedDeepSearchItems::Getter(e) => e.pos,
                        deep_search_extensions::ProcessedDeepSearchItems::Setter(e) => e.pos,
                        deep_search_extensions::ProcessedDeepSearchItems::ImportReference(e) => {
                            e.pos
                        }
                        deep_search_extensions::ProcessedDeepSearchItems::SelfItem(e) => e.pos,
                        _ => unreachable!(),
                    };
                    if deep_search.found_page.hash == page_id {
                        (
                            pos.range_start.is_bigger(&pos.range_start),
                            Some((deep_search.found_page, pos)),
                        )
                    } else {
                        (true, Some((deep_search.found_page, pos)))
                    }
                }
            }
        } else {
            (false, None)
        }
    }

    pub fn is_duplicate(
        &mut self,
        page_id: usize,
        name: String,
        hash: usize,
        pos: defs::Cursor,
    ) -> (bool, Option<(FoundPage, defs::Cursor)>) {
        let deep_search = self.deep_search(
            page_id,
            name.clone(),
            if hash == 0 { None } else { Some(hash) },
            vec![],
            0,
            Some(pos),
        );

        if deep_search.found {
            match deep_search.found_item {
                DeepSearchItems::BrokenPageGraph => (false, None),
                DeepSearchItems::MixUp(_) => (true, None),
                DeepSearchItems::None => (false, None),
                DeepSearchItems::GenericItem(e) => (true, Some((deep_search.found_page, e.pos))),
                e => {
                    if deep_search.found_page.hash == page_id {
                        (
                            pos.range_start.is_bigger(&e.get_pos().range_start),
                            Some((deep_search.found_page, e.get_pos())),
                        )
                    } else {
                        (true, Some((deep_search.found_page, e.get_pos())))
                    }
                }
            }
        } else {
            (false, None)
        }
    }

    pub fn deep_search(
        &mut self,
        target_page: usize,
        name: String,
        ignore_hash: Option<usize>,
        searched: Vec<usize>,
        _level: usize,
        current_pos: Option<Cursor>,
    ) -> DeepSearchResult {
        let mut level = _level;
        let mut found = false;
        let mut found_type = DeepSearchItems::None;
        let mut found_pos = None;
        let mut found_page = FoundPage::default();
        let has_mixup = false;
        let mut inner_page = None;
        let mut searched: Vec<usize> = searched;
        let mixup_hashes: Vec<(String, String)> = Vec::new();
        let mut self_dependencies = vec![Dependency {
            hash: target_page,
            ..Default::default()
        }];

        match self.find_page(target_page) {
            Some(page) => {
                self_dependencies.extend(page.dependencies.clone());
                inner_page = page.inner;
            }
            None => (),
        }
        if !searched.contains(&target_page) {
            for dep in self_dependencies {
                searched.push(target_page);

                if let Some(module_initial_page) = dep.module {
                    let unprocessed_page = self
                        .find_page(dep.hash)
                        .unwrap_or_else(|| panic!("BrokenPageGraph: {}", dep.hash))
                        .clone();

                    match self.find_processed_page_in_module(module_initial_page, dep.hash) {
                        Some(page) => {
                            for item in page.items.iter() {
                                match item.clone() {
                                    Collecting::FunctionParameter(e) => {
                                        if e.name == name {
                                            found_pos = Some(defs::Cursor {
                                                range_start: e.name_pos.range_start,
                                                range_end: e.rtype_pos.range_end,
                                            });
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::FunctionParameter(ellie_tokenizer::syntax::items::function_parameter::FunctionParameter::default().from_definite(e));
                                        }
                                    }
                                    Collecting::Variable(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                            && (current_pos.is_none()
                                                || level != 0
                                                || (level == 0
                                                    && current_pos
                                                        .unwrap()
                                                        .range_start
                                                        .is_bigger(&e.pos.range_start)))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Variable(ellie_tokenizer::syntax::items::variable::VariableCollector::default().from_definite(e).data);
                                        }
                                    }
                                    Collecting::Enum(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Enum(ellie_tokenizer::syntax::items::enum_type::EnumType::default().from_definite(e));
                                        }
                                    }
                                    Collecting::Getter(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Getter(ellie_tokenizer::syntax::items::getter::Getter::default().from_definite(e));
                                        }
                                    }
                                    Collecting::Setter(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Setter(ellie_tokenizer::syntax::items::setter::Setter::default().from_definite(e));
                                        }
                                    }
                                    Collecting::Function(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Function(ellie_tokenizer::syntax::items::function::FunctionCollector::default().from_definite(e).data);
                                        }
                                    }
                                    Collecting::Import(e) => {
                                        if !e.reference.is_empty()
                                            && e.reference == name
                                            && (e.public
                                                || level == 0
                                                || dep.deep_link.is_some()
                                                || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::ImportReference(ellie_tokenizer::syntax::items::import::Import::default().from_definite(e));
                                        }
                                    }
                                    Collecting::Class(e) => {
                                        if e.name == name
                                            && (e.public
                                                || level == 0
                                                || dep.deep_link.is_some()
                                                || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Class(ellie_tokenizer::syntax::items::class::Class::default().from_definite(e));
                                        }
                                    }
                                    Collecting::NativeFunction(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Function(ellie_tokenizer::syntax::items::function::Function {
                                                name: name.clone(),
                                                name_pos: e.name_pos,
                                                public: e.public,
                                                defining: true,
                                                parameters: e.parameters.into_iter()
                                                .map(|x| ellie_tokenizer::syntax::items::function::FunctionParameter {
                                                    name: x.name,
                                                    rtype: ellie_tokenizer::syntax::items::definers::DefinerCollector {
                                                        definer_type: ellie_tokenizer::syntax::items::definers::DefinerTypes::default().from_definite(x.rtype),
                                                        complete: true,
                                                    },
                                                    name_pos: x.name_pos,
                                                    rtype_pos: x.rtype_pos,
                                                    multi_capture: x.multi_capture,
                                                    is_mut: x.is_mut,
                                                })
                                                .collect::<Vec<_>>(),
                                                parameters_pos: e.parameters_pos,
                                                return_type: ellie_tokenizer::syntax::items::definers::DefinerCollector {
                                                    definer_type: ellie_tokenizer::syntax::items::definers::DefinerTypes::default().from_definite(e.return_type),
                                                    complete: true,
                                                },
                                                no_return: e.no_return,
                                                return_pos:  e.return_pos,
                                                body_pos: defs::Cursor::default(),
                                                body: Vec::new(),
                                                pos: e.pos,
                                                hash: e.hash,
                                            });
                                        }
                                    }
                                    Collecting::SelfItem(e) => {
                                        if name == "self" {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::SelfItem(e);
                                        }
                                    }
                                    _ => (),
                                }
                                if found {
                                    break;
                                }
                            }
                        }
                        None => {
                            panic!("Broken Page structure; Failed to find page {}", dep.hash);
                        }
                    }
                } else if dep.processed {
                    let unprocessed_page = self
                        .find_page(dep.hash)
                        .unwrap_or_else(|| panic!("BrokenPageGraph: {}", dep.hash))
                        .clone();
                    match self.find_processed_page(dep.hash) {
                        Some(page) => {
                            for item in page.items.iter() {
                                match item.clone() {
                                    Collecting::Variable(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                            && (current_pos.is_none()
                                                || level != 0
                                                || (level == 0
                                                    && current_pos
                                                        .unwrap()
                                                        .range_start
                                                        .is_bigger(&e.pos.range_start)))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Variable(ellie_tokenizer::syntax::items::variable::VariableCollector::default().from_definite(e).data);
                                        }
                                    }
                                    Collecting::Enum(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Enum(ellie_tokenizer::syntax::items::enum_type::EnumType::default().from_definite(e));
                                        }
                                    }
                                    Collecting::Function(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Function(ellie_tokenizer::syntax::items::function::FunctionCollector::default().from_definite(e).data);
                                        }
                                    }
                                    Collecting::Getter(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Getter(ellie_tokenizer::syntax::items::getter::Getter::default().from_definite(e));
                                        }
                                    }
                                    Collecting::Setter(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Setter(ellie_tokenizer::syntax::items::setter::Setter::default().from_definite(e));
                                        }
                                    }
                                    Collecting::Import(e) => {
                                        if !e.reference.is_empty()
                                            && e.reference == name
                                            && (e.public
                                                || level == 0
                                                || dep.deep_link.is_some()
                                                || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::ImportReference(ellie_tokenizer::syntax::items::import::Import::default().from_definite(e));
                                        }
                                    }
                                    Collecting::Class(e) => {
                                        if e.name == name
                                            && (e.public
                                                || level == 0
                                                || dep.deep_link.is_some()
                                                || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Class(ellie_tokenizer::syntax::items::class::Class::default().from_definite(e));
                                        }
                                    }
                                    Collecting::NativeFunction(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::Function(ellie_tokenizer::syntax::items::function::Function {
                                                name: name.clone(),
                                                name_pos: e.name_pos,
                                                public: e.public,
                                                defining: true,
                                                parameters: e.parameters.into_iter()
                                                .map(|x| ellie_tokenizer::syntax::items::function::FunctionParameter {
                                                    name: x.name,
                                                    rtype: ellie_tokenizer::syntax::items::definers::DefinerCollector {
                                                        definer_type: ellie_tokenizer::syntax::items::definers::DefinerTypes::default().from_definite(x.rtype),
                                                        complete: true,
                                                    },
                                                    name_pos: x.name_pos,
                                                    rtype_pos: x.rtype_pos,
                                                    multi_capture: x.multi_capture,
                                                    is_mut: x.is_mut,
                                                })
                                                .collect::<Vec<_>>(),
                                                parameters_pos: e.parameters_pos,
                                                return_type: ellie_tokenizer::syntax::items::definers::DefinerCollector {
                                                    definer_type: ellie_tokenizer::syntax::items::definers::DefinerTypes::default().from_definite(e.return_type),
                                                    complete: true,
                                                },
                                                no_return: e.no_return,
                                                return_pos:  e.return_pos,
                                                body_pos: defs::Cursor::default(),
                                                body: Vec::new(),
                                                pos: e.pos,
                                                hash: e.hash,
                                            });
                                        }
                                    }
                                    Collecting::SelfItem(e) => {
                                        if name == "self" {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::SelfItem(e);
                                        }
                                    }
                                    _ => (),
                                }
                                if found {
                                    break;
                                }
                            }
                        }
                        None => {
                            panic!("Broken Page structure; Failed to find page {}", dep.hash);
                        }
                    }
                } else {
                    match self.find_page(dep.hash) {
                        Some(page) => {
                            for item in page.items.iter() {
                                match item {
                                    Processors::Variable(e) => {
                                        if e.data.name == name
                                            && (e.data.public
                                                || level == 0
                                                || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.data.hash != t))
                                            && (current_pos.is_none()
                                                || level != 0
                                                || (level == 0
                                                    && current_pos
                                                        .unwrap()
                                                        .range_start
                                                        .is_bigger(&e.data.pos.range_start)))
                                        {
                                            found_pos = Some(e.data.pos);
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type = DeepSearchItems::Variable(e.data.clone());
                                        }
                                    }
                                    Processors::Enum(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type = DeepSearchItems::Enum(e.clone());
                                        }
                                    }
                                    Processors::Function(e) => {
                                        if e.data.name == name
                                            && (e.data.public
                                                || level == 0
                                                || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.data.hash != t))
                                        {
                                            found_pos = Some(e.data.pos);
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type = DeepSearchItems::Function(e.data.clone());
                                        }
                                    }
                                    Processors::Getter(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type = DeepSearchItems::Getter(e.clone());
                                        }
                                    }
                                    Processors::Setter(e) => {
                                        if e.name == name
                                            && (e.public || level == 0 || dep.deep_link.is_some())
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type = DeepSearchItems::Setter(e.clone());
                                        }
                                    }
                                    Processors::Import(e) => {
                                        if !e.reference.is_empty()
                                            && e.reference == name
                                            && (e.public
                                                || level == 0
                                                || dep.deep_link.is_some()
                                                || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type =
                                                DeepSearchItems::ImportReference(e.clone());
                                        }
                                    }
                                    Processors::Class(e) => {
                                        if e.name == name
                                            && (e.public
                                                || level == 0
                                                || dep.deep_link.is_some()
                                                || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                            && (ignore_hash.is_none()
                                                || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type = DeepSearchItems::Class(e.clone());
                                        }
                                    }
                                    Processors::GenericItem(e) => {
                                        if e.generic_name == name
                                            && (level == 0
                                                || dep.deep_link.is_some()
                                                || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type = DeepSearchItems::GenericItem(e.clone());
                                        }
                                    }
                                    Processors::ClassInstance(e) => {
                                        if "self" == name && level == 0 {
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type = DeepSearchItems::ClassInstance(e.clone());
                                        }
                                    }
                                    Processors::FunctionParameter(e) => {
                                        if e.name == name && (level == 0 || dep.deep_link.is_some())
                                        {
                                            found_pos = Some(e.name_pos);
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type =
                                                DeepSearchItems::FunctionParameter(e.clone());
                                        }
                                    }
                                    Processors::ConstructorParameter(e) => {
                                        if e.name == name && (level == 0 || dep.deep_link.is_some())
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type =
                                                DeepSearchItems::ConstructorParameter(e.clone());
                                        }
                                    }
                                    Processors::SelfItem(e) => {
                                        if "self" == name && (level == 0 || dep.deep_link.is_some())
                                        {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(page);
                                            found_type = DeepSearchItems::SelfItem(*e);
                                        }
                                    }
                                    _ => (),
                                }
                                if found {
                                    break;
                                }
                            }
                        }
                        None => {
                            panic!("Broken Page structure; Failed to find page {}", dep.hash);
                        }
                    }
                }
                if found {
                    break;
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

    pub fn find_processed_page(&mut self, hash: usize) -> Option<&mut ProcessedPage> {
        self.processed_pages.find_page(hash)
    }

    pub fn find_processed_page_in_module(
        &mut self,
        module_hash: usize,
        hash: usize,
    ) -> Option<&mut ProcessedPage> {
        match self.modules.iter_mut().find(|x| x.hash == module_hash) {
            Some(e) => e.pages.iter_mut().find(|x| x.hash == hash),
            None => None,
        }
    }

    pub fn find_page(&mut self, hash: usize) -> Option<&mut Page> {
        self.pages.find_page(hash)
    }

    pub fn process_page(&mut self, hash: usize) {
        let (unprocessed_page, unprocessed_page_idx) = match self.pages.find_page_and_idx(hash) {
            Some(e) => (e.0.clone(), e.1),
            None => panic!("Page not found"),
        };

        let (_, processed_page_idx) = match self.processed_pages.find_page_and_idx(hash) {
            None => {
                self.processed_pages.push_page(ProcessedPage {
                    hash,
                    inner: unprocessed_page.inner,
                    path: unprocessed_page.path.clone(),
                    items: vec![],
                    page_type: unprocessed_page.page_type.clone(),
                    dependents: unprocessed_page.dependents.clone(),
                    dependencies: unprocessed_page.dependencies.clone(),
                    unassigned_file_keys: vec![],
                    global_file_keys: vec![],
                });
                (hash, self.processed_pages.pages.len() - 1)
            }
            Some(e) => (e.0.hash, e.1),
        };

        for item in &unprocessed_page.items {
            if unprocessed_page.unreachable {
                if !item.is_virtual() {
                    let unprocessed_page = self.pages.nth_mut(unprocessed_page_idx).unwrap();
                    if unprocessed_page.unreachable_range.range_start.is_zero() {
                        unprocessed_page.unreachable_range.range_start =
                            defs::CursorPosition(item.get_pos().range_start.0, 0);
                    }
                    unprocessed_page.unreachable_range.range_end = item.get_pos().range_end;
                }
            } else {
                //Check leftover file keys are compatible with the element
                match self.processed_pages.nth(processed_page_idx) {
                    Some(e) => {
                        if !e.unassigned_file_keys.is_empty() {
                            match item {
                                Processors::Variable(_) => (),
                                Processors::Function(_) => (),
                                Processors::FileKey(_) => (),
                                Processors::Class(_) => (),
                                Processors::Getter(_) => (),
                                Processors::Setter(_) => (),
                                _ => self.informations.push(
                                    &ellie_core::error::error_list::ERROR_S56
                                        .clone()
                                        .build_with_path(
                                            vec![],
                                            alloc::format!(
                                                "{}:{}:{}",
                                                file!().to_owned(),
                                                line!(),
                                                column!()
                                            ),
                                            unprocessed_page.path.clone(),
                                            item.get_pos(),
                                        ),
                                ),
                            }
                        }
                    }
                    None => (),
                }

                let terminated = match unprocessed_page.page_type {
                    ellie_tokenizer::tokenizer::PageType::FunctionBody(_) => match item {
                        Processors::Variable(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::GetterCall(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::SetterCall(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Function(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Enum(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::FileKey(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::ForLoop(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Loop(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Condition(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Getter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Setter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Class(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Ret(e) => {
                            self.pages
                                .nth_mut(unprocessed_page_idx)
                                .unwrap()
                                .unreachable = true;
                            e.process(
                                self,
                                unprocessed_page_idx,
                                processed_page_idx,
                                unprocessed_page.hash,
                            )
                        }
                        Processors::ClassInstance(e) => {
                            self.processed_pages
                                .nth_mut(processed_page_idx)
                                .unwrap()
                                .items
                                .push(Collecting::ClassInstance(e.clone()));
                            true
                        }
                        Processors::GenericItem(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::FunctionParameter(e) => {
                            self.processed_pages.nth_mut(processed_page_idx).unwrap().items.push(
                            Collecting::FunctionParameter(
                                ellie_core::definite::items::function_parameter::FunctionParameter { name: e.name.clone(), rtype: e.rtype.clone(), name_pos: e.name_pos, rtype_pos: e.rtype_pos, hash: e.hash }
                            ));
                            true
                        }
                        Processors::SelfItem(e) => {
                            self.processed_pages
                                .nth_mut(processed_page_idx)
                                .unwrap()
                                .items
                                .push(Collecting::SelfItem(*e));
                            true
                        }
                        Processors::ConstructorParameter(e) => {
                            self.processed_pages.nth_mut(processed_page_idx).unwrap().items.push(
                            Collecting::ConstructorParameter(
                                ellie_core::definite::items::constructor_parameter::ConstructorParameter { name: e.name.clone(), pos: e.pos }
                            ));
                            true
                        }
                        Processors::Comment(_) => true,
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    unprocessed_page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                            false
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::ConstructorBody => match item {
                        Processors::Variable(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::GetterCall(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::SetterCall(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Function(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Enum(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::FileKey(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::ForLoop(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Loop(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Condition(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Class(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Getter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Setter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Ret(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::GenericItem(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::FunctionParameter(_) => {
                            unreachable!("Unexpected element in body")
                        }
                        Processors::ConstructorParameter(e) => {
                            self.processed_pages.nth_mut(processed_page_idx).unwrap().items.push(
                            Collecting::ConstructorParameter(
                                ellie_core::definite::items::constructor_parameter::ConstructorParameter { name: e.name.clone(), pos: e.pos }
                            ));
                            true
                        }
                        Processors::Comment(_) => true,
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    unprocessed_page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                            false
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::RawBody => match item {
                        Processors::Variable(e) => {
                            if e.data.constant && e.data.value.is_static() {
                                e.process(
                                    self,
                                    unprocessed_page_idx,
                                    processed_page_idx,
                                    unprocessed_page.hash,
                                )
                            } else {
                                self.informations.push(
                                    &error::error_list::ERROR_S60.clone().build_with_path(
                                        vec![],
                                        alloc::format!(
                                            "{}:{}:{}",
                                            file!().to_owned(),
                                            line!(),
                                            column!()
                                        ),
                                        unprocessed_page.path.clone(),
                                        e.data.pos,
                                    ),
                                );
                                false
                            }
                        }
                        Processors::GetterCall(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::SetterCall(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Function(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Enum(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::FileKey(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Import(e) => {
                            let hash = e.hash;
                            e.process(
                                self,
                                unprocessed_page_idx,
                                processed_page_idx,
                                unprocessed_page.hash,
                            );
                            if self.find_processed_page(hash).is_none() {
                                self.process_page(hash);
                            }
                            true
                        }
                        Processors::Condition(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Class(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Getter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Setter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::GenericItem(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::FunctionParameter(e) => {
                            self.processed_pages.nth_mut(processed_page_idx).unwrap().items.push(
                                Collecting::FunctionParameter(
                                    ellie_core::definite::items::function_parameter::FunctionParameter { name: e.name.clone(), rtype: e.rtype.clone(), name_pos: e.name_pos, rtype_pos: e.rtype_pos, hash: e.hash }
                                ));
                            true
                        }
                        Processors::ConstructorParameter(e) => {
                            self.processed_pages.nth_mut(processed_page_idx).unwrap().items.push(
                                Collecting::ConstructorParameter(
                                    ellie_core::definite::items::constructor_parameter::ConstructorParameter { name: e.name.clone(), pos: e.pos }
                                ));
                            true
                        }
                        Processors::Comment(_) => true,
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    unprocessed_page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                            false
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::ClassBody(_) => match item {
                        Processors::Variable(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Function(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::FileKey(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Constructor(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::GenericItem(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Getter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Setter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::ClassInstance(_) => true,
                        Processors::FunctionParameter(_) => true,
                        Processors::ConstructorParameter(_) => true,
                        Processors::Comment(_) => true,
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    unprocessed_page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                            false
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::ConditionBody(_) => match item {
                        Processors::Variable(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::GetterCall(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::SetterCall(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Function(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Enum(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::FileKey(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::ForLoop(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Loop(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Condition(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Class(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Getter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Setter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Ret(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::ClassInstance(_) => {
                            todo!()
                            /*
                            self.processed_pages
                                .nth_mut(processed_page_idx)
                                .unwrap()
                                .items
                                .push(Collecting::SelfItem(
                                    ellie_core::definite::items::self_item::SelfItem {
                                        class_page: e.class_page,
                                        class_hash: e.class_hash,
                                        pos: e.pos,
                                    },
                                ));
                            true
                            */
                        }
                        Processors::GenericItem(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::FunctionParameter(_) => true,
                        Processors::ConstructorParameter(e) => {
                            self.processed_pages.nth_mut(processed_page_idx).unwrap().items.push(
                            Collecting::ConstructorParameter(
                                ellie_core::definite::items::constructor_parameter::ConstructorParameter { name: e.name.clone(), pos: e.pos }
                            ));
                            true
                        }
                        Processors::Comment(_) => true,
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    unprocessed_page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                            false
                        }
                    },
                    ellie_tokenizer::tokenizer::PageType::LoopBody => match item {
                        Processors::Variable(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::GetterCall(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::SetterCall(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Function(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Enum(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::FileKey(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::ForLoop(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Loop(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Condition(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Getter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Setter(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Class(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::Ret(e) => {
                            self.pages
                                .nth_mut(unprocessed_page_idx)
                                .unwrap()
                                .unreachable = true;
                            e.process(
                                self,
                                unprocessed_page_idx,
                                processed_page_idx,
                                unprocessed_page.hash,
                            )
                        }
                        Processors::ClassInstance(_) => {
                            todo!()
                            /* self.processed_pages
                                .nth_mut(processed_page_idx)
                                .unwrap()
                                .items
                                .push(Collecting::SelfItem(
                                    ellie_core::definite::items::self_item::SelfItem {
                                        class_page: e.class_page,
                                        class_hash: e.class_hash,
                                        pos: e.pos,
                                    },
                                ));
                            true */
                        }
                        Processors::Brk(e) => {
                            self.pages
                                .nth_mut(unprocessed_page_idx)
                                .unwrap()
                                .unreachable = true;
                            e.process(
                                self,
                                unprocessed_page_idx,
                                processed_page_idx,
                                unprocessed_page.hash,
                            )
                        }
                        Processors::Go(e) => {
                            self.pages
                                .nth_mut(unprocessed_page_idx)
                                .unwrap()
                                .unreachable = true;
                            e.process(
                                self,
                                unprocessed_page_idx,
                                processed_page_idx,
                                unprocessed_page.hash,
                            )
                        }
                        Processors::GenericItem(e) => e.process(
                            self,
                            unprocessed_page_idx,
                            processed_page_idx,
                            unprocessed_page.hash,
                        ),
                        Processors::FunctionParameter(e) => {
                            self.processed_pages.nth_mut(processed_page_idx).unwrap().items.push(
                                Collecting::FunctionParameter(
                                    ellie_core::definite::items::function_parameter::FunctionParameter { name: e.name.clone(), rtype: e.rtype.clone(), name_pos: e.name_pos, rtype_pos: e.rtype_pos, hash: e.hash }
                                ));
                            true
                        }
                        Processors::ConstructorParameter(e) => {
                            self.processed_pages.nth_mut(processed_page_idx).unwrap().items.push(
                            Collecting::ConstructorParameter(
                                ellie_core::definite::items::constructor_parameter::ConstructorParameter { name: e.name.clone(), pos: e.pos }
                            ));
                            true
                        }
                        Processors::Comment(_) => true,
                        unexpected_element => {
                            self.informations.push(
                                &error::error_list::ERROR_S22.clone().build_with_path(
                                    vec![],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    unprocessed_page.path.clone(),
                                    unexpected_element.get_pos(),
                                ),
                            );
                            false
                        }
                    },
                };
                if !terminated {
                    break;
                }
            }
        }

        #[cfg(feature = "standard_rules")]
        {
            if unprocessed_page.unreachable
                && !unprocessed_page.unreachable_range.range_end.is_zero()
            {
                self.informations
                    .push(&warning::warning_list::WARNING_S4.clone().build(
                        vec![],
                        unprocessed_page.path,
                        unprocessed_page.unreachable_range,
                    ));
            }
        }
    }

    pub fn parse(&mut self) -> Module {
        self.process_page(self.initial_page);
        let mut idx = 0;
        loop {
            let page = match self.pages.nth(idx) {
                Some(i) => i,
                None => break,
            };
            let page_hash = page.hash;
            let page_type = page.page_type.clone();
            let page_path = page.path.clone();
            idx += 1;
            if page.hash == self.initial_page {
                continue;
            }
            self.process_page(page_hash);
            match page_type {
                PageType::FunctionBody(function_page) => {
                    let found_ret = self
                        .find_processed_page(page_hash)
                        .unwrap()
                        .items
                        .clone()
                        .into_iter()
                        .find_map(|item| match item {
                            ellie_core::definite::items::Collecting::Ret(e) => Some(e),
                            _ => None,
                        });
                    if let Some(ret) = found_ret {
                        if self.informations.has_no_errors() {
                            match self.compare_defining_with_type(
                                function_page.return_type,
                                ret.value,
                                page_hash,
                            ) {
                                Ok(result) => {
                                    if result.requires_cast {
                                        self.informations.push(
                                                &error::error_list::ERROR_S41.clone().build_with_path(
                                                    vec![error::ErrorBuildField {
                                                        key: "token".to_owned(),
                                                        value: "Type helpers are not completely implemented yet. Next error is result of this. Follow progress here (https://github.com/behemehal/EllieWorks/issues/8)".to_owned(),
                                                    }],
                                                    alloc::format!(
                                                        "{}:{}:{}",
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!()
                                                    ),
                                                    page_path.clone(),
                                                    ret.pos,
                                                ),
                                            );
                                        let mut err =
                                            error::error_list::ERROR_S3.clone().build_with_path(
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "token1".to_owned(),
                                                        value: result.first.clone(),
                                                    },
                                                    error::ErrorBuildField {
                                                        key: "token2".to_owned(),
                                                        value: result.second.clone(),
                                                    },
                                                ],
                                                alloc::format!(
                                                    "{}:{}:{}",
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!()
                                                ),
                                                page_path.clone(),
                                                ret.pos,
                                            );
                                        err.reference_block =
                                            Some((function_page.return_pos, page_path.clone()));
                                        err.reference_message = "Defined here".to_owned();
                                        err.semi_assist = true;
                                        self.informations.push(&err);
                                    }

                                    if !result.same {
                                        let mut err =
                                            error::error_list::ERROR_S3.clone().build_with_path(
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "token1".to_owned(),
                                                        value: result.first,
                                                    },
                                                    error::ErrorBuildField {
                                                        key: "token2".to_owned(),
                                                        value: result.second,
                                                    },
                                                ],
                                                alloc::format!(
                                                    "{}:{}:{}",
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!()
                                                ),
                                                page_path.clone(),
                                                ret.pos,
                                            );
                                        err.reference_block =
                                            Some((function_page.return_pos, page_path));
                                        err.reference_message = "Defined here".to_owned();
                                        err.semi_assist = true;
                                        self.informations.push(&err);
                                    }
                                }
                                Err(e) => {
                                    self.informations.extend(&e);
                                }
                            }
                        }
                    }
                }
                PageType::ConditionBody(condition_page) => {
                    let mut common_return: Option<(
                        Cursor,
                        Option<(DefinerCollecting, Cursor)>,
                        ellie_tokenizer::syntax::items::condition::ConditionType,
                    )> = None;
                    let found_ret = self
                        .find_processed_page(page_hash)
                        .unwrap()
                        .items
                        .clone()
                        .into_iter()
                        .find_map(|item| match item {
                            ellie_core::definite::items::Collecting::Ret(e) => Some(e),
                            _ => None,
                        });

                    if let Some(ret) = found_ret {
                        let mut errors = Vec::new();
                        match resolve_type(ret.value, page_hash, self, &mut errors, Some(ret.pos)) {
                            Some(ret_type) => match common_return {
                                Some(e) => {
                                    match e.1 {
                                        Some(previous_type) => {
                                            if previous_type.0 != ret_type {
                                                let mut error = error::error_list::ERROR_S13
                                                    .clone()
                                                    .build_with_path(
                                                        vec![
                                                            error::ErrorBuildField {
                                                                key: "token".to_string(),
                                                                value: match e.2 {
                                                                    ConditionType::If => "if",
                                                                    ConditionType::ElseIf => {
                                                                        "else if"
                                                                    }
                                                                    ConditionType::Else => "else",
                                                                }
                                                                .to_string(),
                                                            },
                                                            error::ErrorBuildField {
                                                                key: "token1".to_string(),
                                                                value: match condition_page
                                                                    .chain_type
                                                                {
                                                                    ConditionType::If => "if",
                                                                    ConditionType::ElseIf => {
                                                                        "else if"
                                                                    }
                                                                    ConditionType::Else => "else",
                                                                }
                                                                .to_string(),
                                                            },
                                                        ],
                                                        alloc::format!(
                                                            "{}:{}:{}",
                                                            file!().to_owned(),
                                                            line!(),
                                                            column!()
                                                        ),
                                                        page_path.clone(),
                                                        e.0,
                                                    );

                                                error.reference_block =
                                                    Some((previous_type.1, page_path.clone()));
                                                error.reference_message =
                                                    "Type mismatch".to_string();

                                                self.informations.push(&error);
                                            }
                                        }
                                        None => {
                                            let mut error = error::error_list::ERROR_S13
                                                .clone()
                                                .build_with_path(
                                                    vec![
                                                        error::ErrorBuildField {
                                                            key: "token".to_string(),
                                                            value: match e.2 {
                                                                ConditionType::If => "if",
                                                                ConditionType::ElseIf => "else if",
                                                                ConditionType::Else => "else",
                                                            }
                                                            .to_string(),
                                                        },
                                                        error::ErrorBuildField {
                                                            key: "token1".to_string(),
                                                            value:
                                                                match condition_page.chain_type {
                                                                    ConditionType::If => "if",
                                                                    ConditionType::ElseIf => {
                                                                        "else if"
                                                                    }
                                                                    ConditionType::Else => "else",
                                                                }
                                                                .to_string(),
                                                        },
                                                    ],
                                                    alloc::format!(
                                                        "{}:{}:{}",
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!()
                                                    ),
                                                    page_path.clone(),
                                                    e.0,
                                                );
                                            error.reference_block =
                                                Some((ret.pos, page_path.clone()));
                                            error.reference_message = "Type mismatch".to_string();
                                            self.informations.push(&error);
                                        }
                                    }

                                    common_return = Some((
                                        condition_page.keyword_pos,
                                        Some((ret_type, ret.pos)),
                                        condition_page.chain_type,
                                    ));
                                }
                                None => {
                                    common_return = Some((
                                        condition_page.keyword_pos,
                                        Some((ret_type, ret.pos)),
                                        condition_page.chain_type,
                                    ));
                                }
                            },
                            None => {
                                //Parser should prevent this
                                unreachable!()
                            }
                        };
                        self.informations.extend(&errors);
                    } else {
                        match common_return.clone() {
                            Some(e) => {
                                match e.1 {
                                    Some(f) => {
                                        let mut error =
                                            error::error_list::ERROR_S13.clone().build_with_path(
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "token".to_string(),
                                                        value: match e.2 {
                                                            ConditionType::If => "if",
                                                            ConditionType::ElseIf => "else if",
                                                            ConditionType::Else => "else",
                                                        }
                                                        .to_string(),
                                                    },
                                                    error::ErrorBuildField {
                                                        key: "token1".to_string(),
                                                        value: match condition_page.chain_type {
                                                            ConditionType::If => "if",
                                                            ConditionType::ElseIf => "else if",
                                                            ConditionType::Else => "else",
                                                        }
                                                        .to_string(),
                                                    },
                                                ],
                                                alloc::format!(
                                                    "{}:{}:{}",
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!()
                                                ),
                                                page_path.clone(),
                                                e.0,
                                            );
                                        error.reference_block = Some((f.1, page_path.clone()));
                                        error.reference_message = "Type mismatch".to_string();
                                        self.informations.push(&error);
                                    }
                                    None => (),
                                }

                                common_return = Some((
                                    condition_page.keyword_pos,
                                    None,
                                    condition_page.chain_type,
                                ));
                            }
                            None => {
                                common_return = Some((
                                    condition_page.keyword_pos,
                                    None,
                                    condition_page.chain_type,
                                ));
                            }
                        }
                    }

                    let condition_body =
                        self.find_processed_page(condition_page.page_hash).unwrap();
                    condition_body.items.iter_mut().for_each(|x| match x {
                        Collecting::Condition(e) => {
                            if e.hash == condition_page.condition_hash {
                                e.returns = match &common_return {
                                    Some(e) => e.1.as_ref().map(|e| e.0.clone()),
                                    None => None,
                                };
                            }
                        }
                        _ => (),
                    });
                }
                _ => (),
            }
        }

        if !self.module_info.is_lib {
            let main_function =
                self.deep_search(self.initial_page, "main".to_string(), None, vec![], 0, None);
            if main_function.found {
                match main_function.found_item {
                    DeepSearchItems::Function(_) => (),
                    _ => {
                        let path = self.find_page(self.initial_page).unwrap().path.clone();
                        self.informations.push(
                            &error::error_list::ERROR_S33.clone().build_with_path(
                                vec![],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                path,
                                defs::Cursor::default(),
                            ),
                        );
                    }
                }
            } else {
                let path = self.find_page(self.initial_page).unwrap().path.clone();
                self.informations
                    .push(&error::error_list::ERROR_S33.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path,
                        defs::Cursor::default(),
                    ));
            }
        }

        Module {
            name: self.module_info.name.clone(),
            description: self.module_info.description.clone(),
            initial_page: self.initial_page,
            hash: self.calculate_hash(),
            is_library: self.module_info.is_lib,
            pages: self.processed_pages.clone(),
            version: self.module_info.version.clone(),
            ellie_version: self.module_info.ellie_version.clone(),
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
