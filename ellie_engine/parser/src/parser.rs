use crate::deep_search_extensions::{
    deep_search, resolve_deep_type, resolve_type, DeepTypeResult, ProcessedDeepSearchItems,
};
pub use crate::utils::*;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::{string::String, vec};
use ellie_core::definite::definers::DefinerCollecting;
use ellie_core::definite::types::class_instance;
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
                processed: false,
                module: true,
                dependents: p.dependents.clone(),
                //TODO: THIS DEPENDENCIES ARE COUNTS AS UNPROCESSED PAGES BUT THEY ARE PROCESSED PAGES
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
                deep_link: if module.name == "ellieCore" && x.inner.is_none() {
                    Some(1)
                } else {
                    None
                },
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
        let found_type = resolve_deep_type(self, target_page, rtype.clone(), &mut errors);

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
            DeepTypeResult::Integer(_) => {
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
            DeepTypeResult::Byte(_) => {
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
            DeepTypeResult::Decimal(decimal_type) => {
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
            DeepTypeResult::Bool(_) => {
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
            DeepTypeResult::String(_) => {
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
            DeepTypeResult::Char(_) => {
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
            DeepTypeResult::Collective(_) => todo!(),
            DeepTypeResult::Operator(e) => {
                let value_gen =
                    match resolve_type(rtype, target_page, self, &mut errors, Some(e.pos)) {
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
            DeepTypeResult::Cloak(_) => todo!(),
            DeepTypeResult::Array(e) => {
                let value_gen =
                    match resolve_type(rtype, target_page, self, &mut errors, Some(e.pos)) {
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
            DeepTypeResult::ClassCall(class_call) => {
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
            DeepTypeResult::FunctionCall(e) => {
                let resolved_target =
                    resolve_deep_type(self, target_page, *e.target.clone(), &mut errors);

                match resolved_target {
                    DeepTypeResult::Function(e) => Ok(CompareResult::result(
                        defining.same_as(e.return_type.clone()),
                        defining.to_string(),
                        e.return_type.to_string(),
                    )),
                    DeepTypeResult::FunctionCall(e) => Ok(CompareResult::result(
                        defining.same_as(e.returning.clone()),
                        defining.to_string(),
                        e.returning.to_string(),
                    )),
                    _ => {
                        let rtype = match resolved_target {
                            DeepTypeResult::Integer(e) => {
                                ellie_core::definite::types::Types::Integer(e)
                            }
                            DeepTypeResult::Byte(e) => ellie_core::definite::types::Types::Byte(e),
                            DeepTypeResult::Decimal(e) => {
                                ellie_core::definite::types::Types::Decimal(e)
                            }
                            DeepTypeResult::Bool(e) => ellie_core::definite::types::Types::Bool(e),
                            DeepTypeResult::String(e) => {
                                ellie_core::definite::types::Types::String(e)
                            }
                            DeepTypeResult::Char(e) => ellie_core::definite::types::Types::Char(e),
                            DeepTypeResult::Collective(e) => {
                                ellie_core::definite::types::Types::Collective(e)
                            }
                            DeepTypeResult::Operator(e) => {
                                ellie_core::definite::types::Types::Operator(e)
                            }
                            DeepTypeResult::Cloak(e) => {
                                ellie_core::definite::types::Types::Cloak(e)
                            }
                            DeepTypeResult::Array(e) => {
                                ellie_core::definite::types::Types::Array(e)
                            }
                            DeepTypeResult::ClassCall(e) => {
                                ellie_core::definite::types::Types::ClassCall(e)
                            }
                            DeepTypeResult::BraceReference(e) => {
                                ellie_core::definite::types::Types::BraceReference(e)
                            }
                            DeepTypeResult::Void => ellie_core::definite::types::Types::Void,
                            DeepTypeResult::Null => ellie_core::definite::types::Types::Null,
                            DeepTypeResult::Dynamic => ellie_core::definite::types::Types::Dynamic,
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
            DeepTypeResult::Void => {
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
            DeepTypeResult::Null => {
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
            DeepTypeResult::BraceReference(e) => {
                let value_gen =
                    match resolve_type(rtype, target_page, self, &mut errors, Some(e.pos)) {
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
            DeepTypeResult::Dynamic => {
                if let ellie_core::definite::definers::DefinerCollecting::Generic(_) = defining {
                    Ok(CompareResult::result(
                        true,
                        defining.to_string(),
                        "dyn".to_owned(),
                    ))
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
            DeepTypeResult::NotFound => {
                if errors.is_empty() {
                    Ok(CompareResult::result(false, String::new(), String::new()))
                } else {
                    Err(errors)
                }
            }
            DeepTypeResult::Function(function) => {
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
            DeepTypeResult::EnumData(e) => {
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
            DeepTypeResult::FunctionParameter(e) => {
                if e.rtype.as_ref().unwrap().same_as(defining.clone()) {
                    Ok(CompareResult::result(
                        true,
                        defining.to_string(),
                        e.rtype.as_ref().unwrap().to_string(),
                    ))
                } else {
                    Ok(CompareResult::result(
                        false,
                        defining.to_string(),
                        e.rtype.unwrap().to_string(),
                    ))
                }
            }
            DeepTypeResult::Enum(_) => todo!(),
            DeepTypeResult::ClassInstance(_) => todo!(),
            DeepTypeResult::SelfItem(_) => todo!(),
            DeepTypeResult::ConstructorParameter(_) => todo!(),
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
        let deep_search = deep_search(
            self,
            page_id,
            name.clone(),
            if hash == 0 { None } else { Some(hash) },
            vec![],
            0,
        );

        if deep_search.found {
            match deep_search.found_item {
                ProcessedDeepSearchItems::Variable(e) => {
                    if deep_search.found_page.hash == page_id {
                        (
                            pos.range_start.is_bigger(&e.pos.range_start),
                            Some((deep_search.found_page, e.pos)),
                        )
                    } else {
                        (false, None)
                    }
                }
                ProcessedDeepSearchItems::GenericItem(e) => {
                    (true, Some((deep_search.found_page, e.pos)))
                }
                ProcessedDeepSearchItems::None
                | ProcessedDeepSearchItems::FunctionParameter(_)
                | ProcessedDeepSearchItems::ClassInstance(_) => (false, None),
                e => {
                    let pos = match e {
                        ProcessedDeepSearchItems::Class(e) => e.pos,
                        ProcessedDeepSearchItems::Variable(e) => e.pos,
                        ProcessedDeepSearchItems::Function(e) => e.pos,
                        ProcessedDeepSearchItems::Enum(e) => e.pos,
                        ProcessedDeepSearchItems::NativeFunction(e) => e.pos,
                        ProcessedDeepSearchItems::Getter(e) => e.pos,
                        ProcessedDeepSearchItems::Setter(e) => e.pos,
                        ProcessedDeepSearchItems::ImportReference(e) => e.pos,
                        ProcessedDeepSearchItems::SelfItem(e) => e.pos,
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
                                    Collecting::ConstructorParameter(e) => {
                                        if e.name == name {
                                            found_pos = Some(e.pos);
                                            found = true;
                                            found_page = FoundPage::fill(&unprocessed_page);
                                            found_type = DeepSearchItems::ConstructorParameter(ellie_tokenizer::syntax::items::constructor_parameter::ConstructorParameter::default().from_definite(e));
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
                match self.processed_pages.nth_mut(processed_page_idx) {
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

                let terminated = parse_page_element(
                    self,
                    item,
                    unprocessed_page.page_type.clone(),
                    unprocessed_page_idx,
                    processed_page_idx,
                    unprocessed_page.hash,
                    &unprocessed_page.path,
                );

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
