use crate::deep_search_extensions::{
    find_type, resolve_type,
    ProcessedDeepSearchItems,
};
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use ellie_core::definite::types::reference::IndexChainAttribute;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite::definers::DefinerCollecting;
use ellie_core::definite::types::class_instance::AttributeType;
use ellie_core::definite::{items::Collecting};
use ellie_core::{definite::types, error};
use ellie_tokenizer::syntax::types::reference_type;

use super::TypeParserProcessorOptions;
impl super::TypeParserProcessor for reference_type::ReferenceTypeCollector {
    fn process(
        &self,
        options: &mut super::TypeParserProcessorOptions,
    ) -> Result<types::Types, Vec<error::Error>> {
        let mut errors = Vec::new();

        let mut binding = TypeParserProcessorOptions::new(options.parser, options.page_id);
        let options = binding.dont_include_setter().dont_ignore_type().build();
        let processed_reference = self.data.reference.process(options).clone();

        match processed_reference {
            Ok(found_reference) => {
                #[derive(Debug, Clone)]
                struct Attribute {
                    rtype: AttributeType,
                    name: String,
                    hash: usize,
                    page_hash: usize,
                    class_attribute_idx: usize,
                    value: DefinerCollecting,
                }

                #[allow(dead_code)]
                fn generate_type_from_defining(
                    rtype: ellie_core::definite::definers::DefinerCollecting,
                    page_id: usize,
                    parser: &mut crate::parser::Parser,
                ) -> Option<types::Types> {
                    match rtype {
                        DefinerCollecting::Generic(generic) => {
                            if generic.rtype == "int" {
                                Some(types::Types::Integer(
                                    ellie_core::definite::types::integer::IntegerType {
                                        value: 0,
                                        pos: ellie_core::defs::Cursor::default(),
                                    },
                                ))
                            } else if generic.rtype == "float" || generic.rtype == "double" {
                                Some(types::Types::Decimal(
                                    ellie_core::definite::types::decimal::DecimalType {
                                        value: if generic.rtype == "float" {
                                            ellie_core::definite::types::decimal::DecimalTypeEnum::Float(0.0)
                                        } else {
                                            ellie_core::definite::types::decimal::DecimalTypeEnum::Double(0.0)
                                        },
                                        pos: ellie_core::defs::Cursor::default(),
                                        is_double: generic.rtype == "double",
                                    },
                                ))
                            } else if generic.rtype == "string" {
                                Some(types::Types::String(
                                    ellie_core::definite::types::string::StringType {
                                        value: "".to_owned(),
                                        pos: ellie_core::defs::Cursor::default(),
                                    },
                                ))
                            } else if generic.rtype == "bool" {
                                Some(types::Types::Bool(
                                    ellie_core::definite::types::bool::BoolType { value: true },
                                ))
                            } else if generic.rtype == "dyn" {
                                Some(types::Types::Dynamic)
                            } else if generic.rtype == "void" {
                                Some(types::Types::Void)
                            } else if generic.rtype == "char" {
                                Some(types::Types::Char(
                                    ellie_core::definite::types::ellie_char::CharType {
                                        value: '\0',
                                    },
                                ))
                            } else if generic.rtype == "null" {
                                Some(types::Types::Null)
                            } else {
                                let hash_deep_search =
                                    crate::deep_search_extensions::deep_search_hash(
                                        parser,
                                        page_id,
                                        generic.hash,
                                        vec![],
                                        0,
                                    );
                                if hash_deep_search.found {
                                    match hash_deep_search.found_item {
                                        ProcessedDeepSearchItems::Class(matched_class) => {
                                            if matched_class.generic_definings.is_empty() {
                                                Some(
                                                    types::Types::ClassCall(
                                                        ellie_core::definite::types::class_call::ClassCall {
                                                            target: Box::new(types::Types::VariableType(
                                                                ellie_core::definite::types::variable::VariableType {
                                                                    value: matched_class.name.clone(),
                                                                    reference: matched_class.hash,
                                                                    pos: ellie_core::defs::Cursor::default(),
                                                                },
                                                            )),
                                                            resolved_generics: vec![],
                                                            generic_parameters: vec![],
                                                            keyword_pos: ellie_core::defs::Cursor::default(),
                                                            pos: ellie_core::defs::Cursor::default(),
                                                            target_pos: ellie_core::defs::Cursor::default(),
                                                            params: vec![],
                                                        }
                                                    )
                                                )
                                            } else {
                                                unimplemented!()
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                } else {
                                    unreachable!("Not found: {:?}", generic)
                                }
                            }
                        }
                        DefinerCollecting::ParentGeneric(parent_generic) => {
                            if parent_generic.rtype == "array" {
                                generate_type_from_defining(
                                    parent_generic.generics[0].value.clone(),
                                    page_id,
                                    parser,
                                )
                                .map(|t| {
                                    types::Types::Array(
                                        ellie_core::definite::types::array::ArrayType {
                                            collective: vec![
                                                ellie_core::definite::types::array::ArrayEntry {
                                                    value: t,
                                                    location: ellie_core::defs::Cursor::default(),
                                                },
                                            ],
                                            pos: ellie_core::defs::Cursor::default(),
                                        },
                                    )
                                })
                            } else if parent_generic.rtype == "cloak" {
                                let mut cloak_entries = vec![];
                                let mut unresolved_element_available = false;
                                for generic in parent_generic.generics {
                                    match generate_type_from_defining(
                                        generic.value,
                                        page_id,
                                        parser,
                                    ) {
                                        Some(t) => cloak_entries.push(
                                            ellie_core::definite::types::cloak::CloakEntry {
                                                value: t,
                                                location: ellie_core::defs::Cursor::default(),
                                            },
                                        ),
                                        None => {
                                            unresolved_element_available = true;
                                            break;
                                        }
                                    }
                                }
                                if unresolved_element_available {
                                    None
                                } else {
                                    Some(types::Types::Cloak(
                                        ellie_core::definite::types::cloak::CloakType {
                                            collective: cloak_entries,
                                            pos: ellie_core::defs::Cursor::default(),
                                        },
                                    ))
                                }
                            } else if parent_generic.rtype == "collective" {
                                generate_type_from_defining(parent_generic.generics[0].value.clone(), page_id,
                                    parser,).map(|t| types::Types::Collective(
                                        ellie_core::definite::types::collective::CollectiveType {
                                            entries: vec![
                                                ellie_core::definite::types::collective::CollectiveEntry {
                                                    key: "".to_owned(),
                                                    value: t,
                                                    key_pos: ellie_core::defs::Cursor::default(),
                                                    value_pos: ellie_core::defs::Cursor::default(),
                                                },
                                            ],
                                            pos: ellie_core::defs::Cursor::default(),
                                        },
                                    ))
                            } else {
                                let hash_deep_search =
                                    crate::deep_search_extensions::deep_search_hash(
                                        parser,
                                        page_id,
                                        parent_generic.hash,
                                        vec![],
                                        0,
                                    );
                                if hash_deep_search.found {
                                    match hash_deep_search.found_item {
                                        ProcessedDeepSearchItems::Class(matched_class) => {
                                                Some(
                                                    types::Types::ClassCall(
                                                        ellie_core::definite::types::class_call::ClassCall {
                                                            target: Box::new(types::Types::VariableType(
                                                                ellie_core::definite::types::variable::VariableType {
                                                                    value: matched_class.name.clone(),
                                                                    reference: matched_class.hash,
                                                                    pos: ellie_core::defs::Cursor::default(),
                                                                },
                                                            )),
                                                            resolved_generics: parent_generic.generics.iter().map(|generic| {
                                                                generic.value.clone()
                                                            }).collect::<Vec<_>>(),
                                                            generic_parameters: parent_generic.generics.iter().map(|generic| {
                                                                ellie_core::definite::types::class_call::ClassCallGenericParameter {
                                                                    value: generic.value.clone(),
                                                                    pos: ellie_core::defs::Cursor::default(),
                                                                }
                                                            }).collect::<Vec<_>>(),
                                                            keyword_pos: ellie_core::defs::Cursor::default(),
                                                            pos: ellie_core::defs::Cursor::default(),
                                                            target_pos: ellie_core::defs::Cursor::default(),
                                                            params: vec![],
                                                        }
                                                    )
                                                )
                                        }
                                        _ => unreachable!(),
                                    }
                                } else {
                                    unreachable!()
                                }
                            }
                        }
                        DefinerCollecting::Function(function) => Some(types::Types::FunctionCall(
                            ellie_core::definite::types::function_call::FunctionCall {
                                target: Box::new(types::Types::Null),
                                returning: *function.returning,
                                target_pos: ellie_core::defs::Cursor::default(),
                                params: vec![],
                                generic_parameters: vec![],
                                pos: ellie_core::defs::Cursor::default(),
                            },
                        )),
                        _ => unreachable!(),
                    }
                }

                fn resolve_chain(
                    reference_type: DefinerCollecting,
                    reference_pos: ellie_core::defs::Cursor,
                    page_id: usize,
                    parser: &mut crate::parser::Parser,
                    is_setter: bool,
                ) -> Result<Vec<Attribute>, Vec<error::Error>> {
                    let mut errors: Vec<error::Error> = Vec::new();
                    match reference_type.clone() {
                        ellie_core::definite::definers::DefinerCollecting::Array(_) => todo!(),
                        ellie_core::definite::definers::DefinerCollecting::Generic(generic) => {
                            let hash_deep_search = crate::deep_search_extensions::deep_search_hash(
                                parser,
                                page_id,
                                generic.hash,
                                vec![],
                                0,
                            );

                            if hash_deep_search.found {
                                match hash_deep_search.found_item {
                                    ProcessedDeepSearchItems::Class(class_page) => {
                                        match parser
                                            .find_processed_page(class_page.inner_page_id)
                                            .cloned()
                                        {
                                            Some(class_inner_page) => {
                                                let attributes = class_inner_page.items.iter().filter_map(|item| {
                                                    let class_attribute_idx = class_inner_page.items.iter().filter_map(|x| match x {
                                                        Collecting::Variable(x) => Some(x),
                                                        _ => None
                                                    }).collect::<Vec<_>>();
                                                    match item.clone() {
                                                        Collecting::Variable(e) => {
                                                            let resolved_type = if e.has_type { e.rtype } else { match resolve_type(e.value, class_inner_page.hash, parser, &mut errors, Some(e.value_pos)) {
                                                                Some(e) => e,
                                                                None => return None,
                                                            } };
                                                            Some(Attribute {
                                                                rtype: AttributeType::Property,
                                                                name: e.name.clone(),
                                                                hash: e.hash,
                                                                page_hash: class_inner_page.hash,
                                                                class_attribute_idx: class_attribute_idx.iter().position(|x| x.hash == e.hash).unwrap(),
                                                                value: resolved_type,
                                                            })
                                                        },
                                                        Collecting::Function(e) => {
                                                            Some(Attribute {
                                                                rtype: AttributeType::Method,
                                                                name: e.name.clone(),
                                                                hash: e.hash,
                                                                page_hash: class_inner_page.hash,
                                                                class_attribute_idx: 0,
                                                                value: DefinerCollecting::Function(
                                                                    ellie_core::definite::definers::FunctionType {
                                                                        params: e.parameters.iter().map(|param| {
                                                                            param.rtype.clone()
                                                                        }).collect::<Vec<_>>(),
                                                                        returning: Box::new(e.return_type),
                                                                    }
                                                                ),
                                                            })
                                                        },
                                                        Collecting::NativeFunction(e) => {
                                                            Some(Attribute {
                                                                rtype: AttributeType::Method,
                                                                name: e.name.clone(),
                                                                hash: e.hash,
                                                                page_hash: class_inner_page.hash,
                                                                class_attribute_idx: 0,
                                                                value: DefinerCollecting::Function(
                                                                    ellie_core::definite::definers::FunctionType {
                                                                        params: e.parameters.iter().map(|param| {
                                                                            param.rtype.clone()
                                                                        }).collect::<Vec<_>>(),
                                                                        returning: Box::new(e.return_type),
                                                                    }
                                                                ),
                                                            })
                                                        }
                                                        Collecting::Getter(e) => {
                                                            Some(Attribute {
                                                                rtype: AttributeType::Getter,
                                                                name: e.name.clone(),
                                                                hash: e.hash,
                                                                page_hash: class_inner_page.hash,
                                                                class_attribute_idx: 0,
                                                                value: e.return_type,
                                                            })
                                                        }
                                                        Collecting::Setter(e) => {
                                                            if is_setter {
                                                                Some(Attribute {
                                                                    rtype: AttributeType::Setter,
                                                                    name: e.name.clone(),
                                                                    class_attribute_idx: 0,
                                                                    value: e.rtype,
                                                                    hash: e.hash,
                                                                    page_hash: class_inner_page.hash,
                                                                })
                                                            } else {
                                                                //TODO add setter check
                                                                errors.push(
                                                                    error::error_list::ERROR_S6.clone().build_with_path(
                                                                        vec![error::ErrorBuildField {
                                                                            key: "token".to_owned(),
                                                                            value: reference_type.to_string(),
                                                                        }],
                                                                        alloc::format!(
                                                                            "{}:{}:{}",
                                                                            file!().to_owned(),
                                                                            line!(),
                                                                            column!()
                                                                        ),
                                                                        parser.find_page(page_id).unwrap().path.clone(),
                                                                        reference_pos,
                                                                    ),
                                                                );
                                                                None
                                                            }
                                                        }
                                                        _ => None,
                                                    }
                                                }).collect::<Vec<_>>();
                                                Ok(attributes)
                                            }
                                            None => {
                                                unreachable!()
                                            }
                                        }
                                    }
                                    ProcessedDeepSearchItems::GenericItem(_) => Ok(vec![]),
                                    ProcessedDeepSearchItems::Enum(enum_data) => {
                                        Ok(enum_data.items.iter().map(| item| {
                                            Attribute {
                                                rtype: match item.value {
                                                    ellie_core::definite::items::enum_type::EnumValue::NoValue => AttributeType::EnumItemData,
                                                    ellie_core::definite::items::enum_type::EnumValue::Value(_) => AttributeType::EnumItemData,
                                                },
                                                name: item.identifier.clone(),
                                                //TODO: Fix this
                                                hash: 0,
                                                class_attribute_idx: 0,
                                                page_hash: enum_data.hash,
                                                value:  match item.value.clone() {
                                                    ellie_core::definite::items::enum_type::EnumValue::NoValue => {
                                                        match find_type("void".to_string(), page_id, parser) {
                                                            Some(e) => ellie_core::definite::definers::DefinerCollecting::Generic(e),
                                                            None => {
                                                                errors.push(
                                                                    error::error_list::ERROR_S38.clone().build_with_path(
                                                                        vec![error::ErrorBuildField {
                                                                            key: "void".to_owned(),
                                                                            value: reference_type.to_string(),
                                                                        }],
                                                                        alloc::format!(
                                                                            "{}:{}:{}",
                                                                            file!().to_owned(),
                                                                            line!(),
                                                                            column!()
                                                                        ),
                                                                        parser.find_page(page_id).unwrap().path.clone(),
                                                                        reference_pos,
                                                                    ),
                                                                );
                                                                DefinerCollecting::Dynamic
                                                            }
                                                        }
                                                    },
                                                    ellie_core::definite::items::enum_type::EnumValue::Value(e) => e
                                                },
                                            }
                                        }).collect())
                                    }
                                    _ => unreachable!(),
                                }
                            } else {
                                errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: reference_type.to_string(),
                                    }],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    parser.find_page(page_id).unwrap().path.clone(),
                                    reference_pos,
                                ));
                                Err(errors)
                            }
                        }
                        ellie_core::definite::definers::DefinerCollecting::ParentGeneric(rtype) => {
                            match find_type(rtype.rtype.clone(), page_id, parser) {
                                Some(found_rtype) => {
                                    let hash_deep_search =
                                        crate::deep_search_extensions::deep_search_hash(
                                            parser,
                                            page_id,
                                            found_rtype.hash,
                                            vec![],
                                            0,
                                        );

                                    if hash_deep_search.found {
                                        match hash_deep_search.found_item {
                                            ProcessedDeepSearchItems::Class(class_item) => {
                                                match resolve_chain(
                                                    DefinerCollecting::Generic(found_rtype.clone()),
                                                    ellie_core::defs::Cursor::default(),
                                                    page_id,
                                                    parser,
                                                    is_setter,
                                                ) {
                                                    Ok(e) => {
                                                        Ok(e.clone()
                                                            .iter_mut()
                                                            .map(|attr| {
                                                                for (i, generic_defining) in
                                                                    class_item
                                                                        .generic_definings
                                                                        .iter()
                                                                        .enumerate()
                                                                {
                                                                    attr.value.convert_generic(
                                                                        generic_defining.hash,
                                                                        rtype.generics[i]
                                                                            .value
                                                                            .clone(),
                                                                    );
                                                                    // = rtype.generics[i].value.clone();
                                                                }
                                                                attr.clone()
                                                            })
                                                            .collect::<Vec<_>>())
                                                    }
                                                    Err(e) => {
                                                        errors.extend(e);
                                                        Err(errors)
                                                    }
                                                }
                                            }
                                            _ => unreachable!("Unexpected parent_generic target."),
                                        }
                                    } else {
                                        errors.push(
                                            error::error_list::ERROR_S6.clone().build_with_path(
                                                vec![error::ErrorBuildField {
                                                    key: "token".to_owned(),
                                                    value: reference_type.to_string(),
                                                }],
                                                alloc::format!(
                                                    "{}:{}:{}",
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!()
                                                ),
                                                parser.find_page(page_id).unwrap().path.clone(),
                                                reference_pos,
                                            ),
                                        );
                                        Err(errors)
                                    }
                                }
                                None => {
                                    errors.push(
                                        error::error_list::ERROR_S38.clone().build_with_path(
                                            vec![error::ErrorBuildField {
                                                key: "token".to_owned(),
                                                value: "function".to_string(),
                                            }],
                                            alloc::format!(
                                                "{}:{}:{}",
                                                file!().to_owned(),
                                                line!(),
                                                column!()
                                            ),
                                            parser.find_page(page_id).unwrap().path.clone(),
                                            reference_pos,
                                        ),
                                    );
                                    Err(errors)
                                }
                            }
                        }
                        ellie_core::definite::definers::DefinerCollecting::Function(_) => {
                            match find_type("function".to_owned(), page_id, parser) {
                                Some(rtype) => {
                                    match resolve_chain(
                                        DefinerCollecting::Generic(rtype),
                                        ellie_core::defs::Cursor::default(),
                                        page_id,
                                        parser,
                                        is_setter,
                                    ) {
                                        Ok(e) => Ok(e),
                                        Err(e) => {
                                            errors.extend(e);
                                            Err(errors)
                                        }
                                    }
                                }
                                None => {
                                    errors.push(
                                        error::error_list::ERROR_S38.clone().build_with_path(
                                            vec![error::ErrorBuildField {
                                                key: "token".to_owned(),
                                                value: "function".to_string(),
                                            }],
                                            alloc::format!(
                                                "{}:{}:{}",
                                                file!().to_owned(),
                                                line!(),
                                                column!()
                                            ),
                                            parser.find_page(page_id).unwrap().path.clone(),
                                            reference_pos,
                                        ),
                                    );
                                    Err(errors)
                                }
                            }
                        }
                        ellie_core::definite::definers::DefinerCollecting::Cloak(_) => todo!(),
                        ellie_core::definite::definers::DefinerCollecting::Collective(_) => {
                            todo!()
                        }
                        ellie_core::definite::definers::DefinerCollecting::Nullable(_) => {
                            todo!()
                        }
                        ellie_core::definite::definers::DefinerCollecting::Dynamic => todo!(),
                        DefinerCollecting::EnumField(_) => todo!(),
                        DefinerCollecting::ClassInstance(_) => {
                            todo!("TO BE REMOVED");
                            /*
                            let mut attributes = Vec::new();
                            for attribute in &class_instance.attributes {
                                let page = parser.find_processed_page(attribute.page).unwrap();
                                let item = page.find_item_by_hash(attribute.hash).unwrap();

                                match item {
                                    Collecting::Variable(e) => {
                                        let value = if e.has_type {
                                            e.rtype
                                        } else {
                                            match resolve_type(
                                                e.value,
                                                page_id,
                                                parser,
                                                &mut errors,
                                                Some(e.value_pos),
                                            ) {
                                                Some(e) => e,
                                                None => return Err(errors),
                                            }
                                        };
                                        attributes.push(Attribute {
                                            rtype: attribute._rtype.clone(),
                                            name: attribute.name.clone(),
                                            hash: attribute.hash,
                                            class_attribute_idx: 0,
                                            page_hash: attribute.page,
                                            value,
                                        });
                                    }
                                    Collecting::Function(e) => {
                                        attributes.push(Attribute {
                                            rtype: attribute._rtype.clone(),
                                            name: attribute.name.clone(),
                                            hash: attribute.hash,
                                            page_hash: attribute.page,
                                            class_attribute_idx: 0,
                                            value: DefinerCollecting::Function(
                                                ellie_core::definite::definers::FunctionType {
                                                    params: e
                                                        .parameters
                                                        .iter()
                                                        .map(|param| param.rtype.clone())
                                                        .collect::<Vec<_>>(),
                                                    returning: Box::new(e.return_type),
                                                },
                                            ),
                                        });
                                    }
                                    Collecting::Getter(e) => {
                                        attributes.push(Attribute {
                                            rtype: attribute._rtype.clone(),
                                            name: attribute.name.clone(),
                                            hash: attribute.hash,
                                            page_hash: attribute.page,
                                            class_attribute_idx: 0,
                                            value: e.return_type,
                                        });
                                    }
                                    Collecting::Setter(e) => {
                                        let value = if is_setter {
                                            e.rtype
                                        } else {
                                            //TODO add setter check
                                            errors.push(
                                                error::error_list::ERROR_S6
                                                    .clone()
                                                    .build_with_path(
                                                        vec![error::ErrorBuildField {
                                                            key: "token".to_owned(),
                                                            value: reference_type.to_string(),
                                                        }],
                                                        alloc::format!(
                                                            "{}:{}:{}",
                                                            file!().to_owned(),
                                                            line!(),
                                                            column!()
                                                        ),
                                                        parser
                                                            .find_page(page_id)
                                                            .unwrap()
                                                            .path
                                                            .clone(),
                                                        reference_pos,
                                                    ),
                                            );
                                            return Err(errors);
                                        };
                                        attributes.push(Attribute {
                                            rtype: attribute._rtype.clone(),
                                            name: attribute.name.clone(),
                                            hash: attribute.hash,
                                            page_hash: attribute.page,
                                            class_attribute_idx: 0,
                                            value,
                                        });
                                    }
                                    Collecting::NativeFunction(e) => {
                                        attributes.push(Attribute {
                                            rtype: attribute._rtype.clone(),
                                            name: attribute.name.clone(),
                                            hash: attribute.hash,
                                            page_hash: attribute.page,
                                            class_attribute_idx: 0,
                                            value: DefinerCollecting::Function(
                                                ellie_core::definite::definers::FunctionType {
                                                    params: e
                                                        .parameters
                                                        .iter()
                                                        .map(|param| param.rtype.clone())
                                                        .collect::<Vec<_>>(),
                                                    returning: Box::new(e.return_type),
                                                },
                                            ),
                                        });
                                    }
                                    _ => (),
                                }
                            }
                            Ok(attributes)
                            */
                        }
                    }
                }

                let reference_type = match resolve_type(
                    found_reference.clone(),
                    options.page_id,
                    options.parser,
                    &mut errors,
                    Some(self.data.reference_pos),
                ) {
                    Some(e) => e,
                    None => return Err(errors),
                };

                let mut last_chain_attributes = (
                    reference_type.clone(),
                    resolve_chain(
                        reference_type,
                        self.data.reference_pos,
                        options.page_id,
                        options.parser,
                        options.include_setter,
                    ),
                );

                let mut index_chain = Vec::new();

                for chain in self.data.chain.clone() {
                    match last_chain_attributes.1.clone() {
                        Ok(e) => {
                            let attribute_index = e.iter().position(|a| a.name == chain.value);
                            let attribute = attribute_index.map(|a| e[a].clone());
                            match attribute {
                                Some(a) => {
                                    index_chain.push(IndexChainAttribute {
                                        rtype: a.rtype.clone(),
                                        idx: attribute_index.unwrap(),
                                        class_attribute_idx: a.class_attribute_idx,
                                        hash: a.hash,
                                        page_hash: a.page_hash,
                                    });
                                    last_chain_attributes = (
                                        a.value.clone(),
                                        resolve_chain(
                                            a.value.clone(),
                                            chain.pos,
                                            options.page_id,
                                            options.parser,
                                            options.include_setter,
                                        ),
                                    );
                                }
                                None => {
                                    errors.push(
                                        error::error_list::ERROR_S42.clone().build_with_path(
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "token".to_owned(),
                                                    value: chain.value,
                                                },
                                                error::ErrorBuildField {
                                                    key: "token1".to_owned(),
                                                    value: last_chain_attributes.0.to_string(),
                                                },
                                            ],
                                            alloc::format!(
                                                "{}:{}:{}",
                                                file!().to_owned(),
                                                line!(),
                                                column!()
                                            ),
                                            options
                                                .parser
                                                .find_page(options.page_id)
                                                .unwrap()
                                                .path
                                                .clone(),
                                            chain.pos,
                                        ),
                                    );
                                }
                            }
                        }
                        Err(err) => {
                            errors.extend(err);
                        }
                    }
                }

                if errors.is_empty() {
                    Ok(types::Types::Reference(types::reference::ReferenceType {
                        reference: Box::new(found_reference),
                        reference_pos: self.data.reference_pos,
                        chain: self
                            .data
                            .chain
                            .iter()
                            .map(|chain| types::reference::Chain {
                                pos: chain.pos,
                                value: chain.value.clone(),
                            })
                            .collect::<Vec<_>>(),
                        index_chain,
                        pos: self.data.pos,
                    }))
                } else {
                    Err(errors)
                }
            }
            Err(e) => Err(e),
        }
    }
}
