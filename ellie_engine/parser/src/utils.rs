use crate::parser;
use crate::processors::items::{ItemParserProcessor, ItemParserProcessorOptions};
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use ellie_core::definite::types::class_instance::{self, Attribute, AttributeType, ClassInstance};
use ellie_core::definite::{items::Collecting, Converter};
use ellie_core::utils::{ExportPage, PageExport};
use ellie_core::{defs, error};
use ellie_tokenizer::processors::items::Processors;
use ellie_tokenizer::processors::types::Processors as TypeProcessors;
use ellie_tokenizer::tokenizer::{Page, PageType};
use serde::{Deserialize, Serialize};
use ellie_core::definite::items::file_key::FileKey;


pub fn parse_page_element(
    parser: &mut parser::Parser,
    item: &Processors,
    page_type: PageType,
    unprocessed_page_idx: usize,
    processed_page_idx: usize,
    unprocessed_page_hash: usize,
    unprocessed_page_path: &String,
) -> bool {
    let mut processor_options = ItemParserProcessorOptions::build(
        parser,
        unprocessed_page_idx,
        processed_page_idx,
        unprocessed_page_hash,
    );

    match page_type {
        PageType::FunctionBody(_) => match item {
            Processors::Variable(e) => e.process(&mut processor_options),
            Processors::GetterCall(e) => e.process(&mut processor_options),
            Processors::SetterCall(e) => e.process(&mut processor_options),
            Processors::Function(e) => e.process(&mut processor_options),
            Processors::Enum(e) => e.process(&mut processor_options),
            Processors::FileKey(e) => e.process(&mut processor_options),
            Processors::ForLoop(e) => e.process(&mut processor_options),
            Processors::Loop(e) => e.process(&mut processor_options),
            Processors::Condition(e) => e.process(&mut processor_options),
            Processors::Getter(e) => e.process(&mut processor_options),
            Processors::Setter(e) => e.process(&mut processor_options),
            Processors::Class(e) => e.process(&mut processor_options),
            Processors::Ret(e) => {
                parser
                    .pages
                    .nth_mut(unprocessed_page_idx)
                    .unwrap()
                    .unreachable = true;

                let mut processor_options = ItemParserProcessorOptions::build(
                    parser,
                    unprocessed_page_idx,
                    processed_page_idx,
                    unprocessed_page_hash,
                );

                e.process(&mut processor_options)
            }
            Processors::ClassInstance(e) => {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::ClassInstance(e.clone()));
                true
            }
            Processors::GenericItem(e) => e.process(&mut processor_options),
            Processors::FunctionParameter(e) => {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::FunctionParameter(
                        ellie_core::definite::items::function_parameter::FunctionParameter {
                            name: e.name.clone(),
                            rtype: e.rtype.clone(),
                            name_pos: e.name_pos,
                            rtype_pos: e.rtype_pos,
                            hash: e.hash,
                        },
                    ));
                true
            }
            Processors::SelfItem(e) => {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::SelfItem(*e));
                true
            }
            Processors::ConstructorParameter(e) => {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::ConstructorParameter(e.clone().to_definite()));
                true
            }
            Processors::Comment(_) => true,
            unexpected_element => {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S22.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        unprocessed_page_path.clone(),
                        unexpected_element.get_pos(),
                    ));
                false
            }
        },
        ellie_tokenizer::tokenizer::PageType::ConstructorBody => match item {
            Processors::Variable(e) => e.process(&mut processor_options),
            Processors::GetterCall(e) => e.process(&mut processor_options),
            Processors::SetterCall(e) => e.process(&mut processor_options),
            Processors::Function(e) => e.process(&mut processor_options),
            Processors::Enum(e) => e.process(&mut processor_options),
            Processors::FileKey(e) => e.process(&mut processor_options),
            Processors::ForLoop(e) => e.process(&mut processor_options),
            Processors::Loop(e) => e.process(&mut processor_options),
            Processors::Condition(e) => e.process(&mut processor_options),
            Processors::Class(e) => e.process(&mut processor_options),
            Processors::Getter(e) => e.process(&mut processor_options),
            Processors::Setter(e) => e.process(&mut processor_options),
            Processors::Ret(e) => e.process(&mut processor_options),
            Processors::GenericItem(e) => e.process(&mut processor_options),
            Processors::FunctionParameter(_) => {
                unreachable!("Unexpected element in body")
            }
            Processors::ConstructorParameter(e) => {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::ConstructorParameter(e.clone().to_definite()));
                true
            }
            Processors::Comment(_) => true,
            unexpected_element => {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S22.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        unprocessed_page_path.clone(),
                        unexpected_element.get_pos(),
                    ));
                false
            }
        },
        ellie_tokenizer::tokenizer::PageType::RawBody => match item {
            Processors::Variable(e) => {
                if e.data.constant && e.data.value.is_static() {
                    e.process(&mut processor_options)
                } else {
                    parser.informations.push(
                        &error::error_list::ERROR_S60.clone().build_with_path(
                            vec![],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            unprocessed_page_path.clone(),
                            e.data.pos,
                        ),
                    );
                    false
                }
            }
            Processors::GetterCall(e) => e.process(&mut processor_options),
            Processors::SetterCall(e) => e.process(&mut processor_options),
            Processors::Function(e) => e.process(&mut processor_options),
            Processors::Enum(e) => e.process(&mut processor_options),
            Processors::FileKey(e) => e.process(&mut processor_options),
            Processors::Import(e) => {
                let hash = e.hash;
                e.process(&mut processor_options);
                if parser.find_processed_page(hash).is_none() {
                    parser.process_page(hash);
                }
                true
            }
            Processors::Condition(e) => e.process(&mut processor_options),
            Processors::Class(e) => e.process(&mut processor_options),
            Processors::Getter(e) => e.process(&mut processor_options),
            Processors::Setter(e) => e.process(&mut processor_options),
            Processors::GenericItem(e) => e.process(&mut processor_options),
            Processors::FunctionParameter(e) => {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::FunctionParameter(
                        ellie_core::definite::items::function_parameter::FunctionParameter {
                            name: e.name.clone(),
                            rtype: e.rtype.clone(),
                            name_pos: e.name_pos,
                            rtype_pos: e.rtype_pos,
                            hash: e.hash,
                        },
                    ));
                true
            }
            Processors::ConstructorParameter(e) => {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::ConstructorParameter(e.clone().to_definite()));
                true
            }
            Processors::Comment(_) => true,
            unexpected_element => {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S22.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        unprocessed_page_path.clone(),
                        unexpected_element.get_pos(),
                    ));
                false
            }
        },
        ellie_tokenizer::tokenizer::PageType::ClassBody(_) => match item {
            Processors::Variable(e) => e.process(&mut processor_options),
            Processors::Function(e) => e.process(&mut processor_options),
            Processors::FileKey(e) => e.process(&mut processor_options),
            Processors::Constructor(e) => e.process(&mut processor_options),
            Processors::GenericItem(e) => e.process(&mut processor_options),
            Processors::Getter(e) => e.process(&mut processor_options),
            Processors::Setter(e) => e.process(&mut processor_options),
            Processors::ClassInstance(_) => true,
            Processors::FunctionParameter(_) => true,
            Processors::ConstructorParameter(_) => true,
            Processors::Comment(_) => true,
            unexpected_element => {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S22.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        unprocessed_page_path.clone(),
                        unexpected_element.get_pos(),
                    ));
                false
            }
        },
        ellie_tokenizer::tokenizer::PageType::ConditionBody(_) => match item {
            Processors::Variable(e) => e.process(&mut processor_options),
            Processors::GetterCall(e) => e.process(&mut processor_options),
            Processors::SetterCall(e) => e.process(&mut processor_options),
            Processors::Function(e) => e.process(&mut processor_options),
            Processors::Enum(e) => e.process(&mut processor_options),
            Processors::FileKey(e) => e.process(&mut processor_options),
            Processors::ForLoop(e) => e.process(&mut processor_options),
            Processors::Loop(e) => e.process(&mut processor_options),
            Processors::Condition(e) => e.process(&mut processor_options),
            Processors::Class(e) => e.process(&mut processor_options),
            Processors::Getter(e) => e.process(&mut processor_options),
            Processors::Setter(e) => e.process(&mut processor_options),
            Processors::Ret(e) => e.process(&mut processor_options),
            Processors::Brk(e) => e.process(&mut processor_options),
            Processors::ClassInstance(_) => {
                todo!()
                /*
                parser.processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::parserItem(
                        ellie_core::definite::items::parser_item::parserItem {
                            class_page: e.class_page,
                            class_hash: e.class_hash,
                            pos: e.pos,
                        },
                    ));
                true
                */
            }
            Processors::GenericItem(e) => e.process(&mut processor_options),
            Processors::FunctionParameter(_) => true,
            Processors::ConstructorParameter(e) => {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::ConstructorParameter(e.clone().to_definite()));
                true
            }
            Processors::Comment(_) => true,
            unexpected_element => {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S22.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        unprocessed_page_path.clone(),
                        unexpected_element.get_pos(),
                    ));
                false
            }
        },
        ellie_tokenizer::tokenizer::PageType::LoopBody => match item {
            Processors::Variable(e) => e.process(&mut processor_options),
            Processors::GetterCall(e) => e.process(&mut processor_options),
            Processors::SetterCall(e) => e.process(&mut processor_options),
            Processors::Function(e) => e.process(&mut processor_options),
            Processors::Enum(e) => e.process(&mut processor_options),
            Processors::FileKey(e) => e.process(&mut processor_options),
            Processors::ForLoop(e) => e.process(&mut processor_options),
            Processors::Loop(e) => e.process(&mut processor_options),
            Processors::Condition(e) => e.process(&mut processor_options),
            Processors::Getter(e) => e.process(&mut processor_options),
            Processors::Setter(e) => e.process(&mut processor_options),
            Processors::Class(e) => e.process(&mut processor_options),
            Processors::Ret(e) => {
                parser
                    .pages
                    .nth_mut(unprocessed_page_idx)
                    .unwrap()
                    .unreachable = true;

                let mut processor_options = ItemParserProcessorOptions::build(
                    parser,
                    unprocessed_page_idx,
                    processed_page_idx,
                    unprocessed_page_hash,
                );

                e.process(&mut processor_options)
            }
            Processors::ClassInstance(_) => {
                todo!()
                /* parser.processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::parserItem(
                        ellie_core::definite::items::parser_item::parserItem {
                            class_page: e.class_page,
                            class_hash: e.class_hash,
                            pos: e.pos,
                        },
                    ));
                true */
            }
            Processors::Brk(e) => {
                parser
                    .pages
                    .nth_mut(unprocessed_page_idx)
                    .unwrap()
                    .unreachable = true;
                let mut processor_options = ItemParserProcessorOptions::build(
                    parser,
                    unprocessed_page_idx,
                    processed_page_idx,
                    unprocessed_page_hash,
                );

                e.process(&mut processor_options)
            }
            Processors::Go(e) => {
                parser
                    .pages
                    .nth_mut(unprocessed_page_idx)
                    .unwrap()
                    .unreachable = true;
                let mut processor_options = ItemParserProcessorOptions::build(
                    parser,
                    unprocessed_page_idx,
                    processed_page_idx,
                    unprocessed_page_hash,
                );
                e.process(&mut processor_options)
            }
            Processors::GenericItem(e) => e.process(&mut processor_options),
            Processors::FunctionParameter(e) => {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::FunctionParameter(e.clone().to_definite()));
                true
            }
            Processors::ConstructorParameter(e) => {
                parser
                    .processed_pages
                    .nth_mut(processed_page_idx)
                    .unwrap()
                    .items
                    .push(Collecting::ConstructorParameter(e.clone().to_definite()));
                true
            }
            Processors::Comment(_) => true,
            unexpected_element => {
                parser
                    .informations
                    .push(&error::error_list::ERROR_S22.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        unprocessed_page_path.clone(),
                        unexpected_element.get_pos(),
                    ));
                false
            }
        },
    }
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
    pub fn is_type_allowed(&self, type_: TypeProcessors) -> (bool, String) {
        match type_ {
            TypeProcessors::Decimal(decimal_type) => {
                if decimal_type.data.is_double {
                    self.floats.clone()
                } else {
                    self.doubles.clone()
                }
            }
            TypeProcessors::Char(_) => self.chars.clone(),
            TypeProcessors::String(_) => self.strings.clone(),
            TypeProcessors::Array(_) => self.arrays.clone(),
            TypeProcessors::BraceReference(_) => self.nullables.clone(),
            TypeProcessors::Cloak(_) => self.cloaks.clone(),
            TypeProcessors::Collective(_) => self.collectives.clone(),
            TypeProcessors::AsKeyword(_) => self.type_conversions.clone(),
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
