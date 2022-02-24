use core::panic;
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, string::String};
use ellie_core::definite::definers::DefinerCollecting;
use ellie_core::definite::items::variable;
use ellie_core::{
    definite::{items::Collecting, types, Converter},
    error,
};
use ellie_tokenizer::processors::types::Processors;
use ellie_tokenizer::syntax::types::variable_type;
use enum_as_inner::EnumAsInner;

use crate::deep_search_extensions::{deep_search_hash, find_type, resolve_type};

pub fn process(
    from: Processors,
    parser: &mut super::Parser,
    page_id: u64,
    ignore_hash: Option<u64>,
) -> Result<types::Types, Vec<error::Error>> {
    let mut errors = Vec::new();

    let (type_allowed, err_str) = parser.parser_settings.is_type_allowed(from.clone());

    if !type_allowed {
        let path = parser.find_page(page_id).unwrap().path.clone();
        parser
            .informations
            .push(&error::error_list::ERROR_S47.clone().build_with_path(
                vec![error::ErrorBuildField {
                    key: "token".to_owned(),
                    value: err_str,
                }],
                file!().to_owned(),
                path,
                from.get_pos(),
            ));
    }

    match from.clone() {
        Processors::Variable(variable) => {
            let deep_search_result = parser.deep_search(
                page_id,
                variable.data.value.clone(),
                ignore_hash,
                Vec::new(),
                0,
            );

            if deep_search_result.found {
                match deep_search_result.found_item {
                    crate::parser::DeepSearchItems::Class(e) => Ok(types::Types::ClassCall(
                        ellie_core::definite::types::class_call::ClassCall {
                            target: Box::new(ellie_core::definite::types::Types::VariableType(
                                ellie_core::definite::types::variable::VariableType {
                                    value: e.name.clone(),
                                    reference: e.hash,
                                    pos: ellie_core::defs::Cursor::default(),
                                },
                            )),
                            params: vec![],
                            keyword_pos: ellie_core::defs::Cursor::default(),
                            target_pos: ellie_core::defs::Cursor::default(),
                            generic_parameters: vec![],
                            pos: ellie_core::defs::Cursor::default(),
                        },
                    )),
                    crate::parser::DeepSearchItems::Variable(e) => {
                        Ok(types::Types::VariableType(types::variable::VariableType {
                            value: e.name,
                            reference: e.hash,
                            pos: from.get_pos(),
                        }))
                    }
                    crate::parser::DeepSearchItems::Function(_) => {
                        todo!("function type not yet implemented")
                    }
                    crate::parser::DeepSearchItems::ImportReference(_) => {
                        todo!("import reference type not yet implemented")
                    }
                    crate::parser::DeepSearchItems::BrokenPageGraph => todo!(),
                    crate::parser::DeepSearchItems::MixUp(_) => todo!(),
                    crate::parser::DeepSearchItems::None => todo!(),
                    crate::parser::DeepSearchItems::SelfItem(_) => todo!(),
                    crate::parser::DeepSearchItems::GenericItem(_) => todo!(),
                    crate::parser::DeepSearchItems::FunctionParameter(e) => {
                        Ok(types::Types::VariableType(types::variable::VariableType {
                            value: e.name,
                            reference: 0,
                            pos: from.get_pos(),
                        }))
                    }
                    crate::parser::DeepSearchItems::ConstructorParameter(e) => {
                        Ok(types::Types::VariableType(types::variable::VariableType {
                            value: e.name,
                            reference: 0,
                            pos: e.pos,
                        }))
                    }
                }
            } else {
                errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: variable.data.value,
                    }],
                    file!().to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    from.get_pos(),
                ));
                Err(errors)
            }
        }
        Processors::Negative(_) => todo!("negative type not yet implemented"),
        Processors::Array(array_type) => {
            let mut collective = vec![];
            for i in array_type.data.collective {
                let response = process(i.value, parser, page_id, ignore_hash);
                if response.is_err() {
                    errors.append(&mut response.unwrap_err());
                } else {
                    collective.push(types::array::ArrayEntry {
                        value: response.unwrap(),
                        location: i.location,
                    });
                }
            }

            if errors.len() == 0 {
                Ok(types::Types::Array(types::array::ArrayType {
                    collective,
                    pos: from.get_pos(),
                }))
            } else {
                Err(errors)
            }
        }
        Processors::Operator(_) => todo!("operator type not yet implemented"),
        Processors::Reference(reference) => {
            let processed_reference = process(
                *reference.data.reference.clone(),
                parser,
                page_id,
                ignore_hash,
            );
            match processed_reference {
                Ok(found_reference) => {
                    #[derive(Debug, Clone, PartialEq)]
                    enum AttributeType {
                        Property,
                        Method,
                    }

                    #[derive(Debug, Clone)]
                    struct Attribute {
                        rtype: AttributeType,
                        name: String,
                        value: DefinerCollecting,
                    }

                    fn generate_type_from_defining(
                        rtype: ellie_core::definite::definers::DefinerCollecting,
                        page_id: u64,
                        parser: &mut crate::parser::Parser,
                    ) -> Option<types::Types> {
                        match rtype {
                            DefinerCollecting::Generic(generic) => {
                                if generic.rtype == "int" {
                                    Some(types::Types::Integer(
                                        ellie_core::definite::types::integer::IntegerType {
                                            value: ellie_core::definite::types::integer::IntegerSize::I8(0),
                                            rtype: ellie_core::definite::types::integer::IntegerTypes::I8,
                                            pos: ellie_core::defs::Cursor::default(),
                                        },
                                    ))
                                } else if generic.rtype == "float" {
                                    Some(types::Types::Float(
                                        ellie_core::definite::types::float::FloatType {
                                            value:
                                                ellie_core::definite::types::float::FloatSize::F32(
                                                    0.0,
                                                ),
                                            rtype:
                                                ellie_core::definite::types::float::FloatTypes::F32,
                                            pos: ellie_core::defs::Cursor::default(),
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
                                            crate::deep_search_extensions::ProcessedDeepSearchItems::Class(matched_class) => {

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
                                        unreachable!()
                                    }
                                }
                            }
                            DefinerCollecting::ParentGeneric(parent_generic) => {
                                if parent_generic.rtype == "array" {
                                    match generate_type_from_defining(
                                        parent_generic.generics[0].value.clone(),
                                        page_id,
                                        parser,
                                    ) {
                                        Some(t) => Some(types::Types::Array(
                                            ellie_core::definite::types::array::ArrayType {
                                                collective: vec![
                                                    ellie_core::definite::types::array::ArrayEntry {
                                                        value: t,
                                                        location: ellie_core::defs::Cursor::default(),
                                                    },
                                                ],
                                                pos: ellie_core::defs::Cursor::default(),
                                            },
                                        )),
                                        None => None,
                                    }
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
                                    match generate_type_from_defining(parent_generic.generics[0].value.clone(),  page_id,
                                    parser,) {
                                        Some(k) =>
                                        match generate_type_from_defining(parent_generic.generics[1].value.clone(), page_id,
                                        parser,) {
                                            Some(t) => Some(types::Types::Collective(
                                            ellie_core::definite::types::collective::CollectiveType {
                                                entries: vec![
                                                    ellie_core::definite::types::collective::CollectiveEntry {
                                                        key: k,
                                                        value: t,
                                                        key_pos: ellie_core::defs::Cursor::default(),
                                                        value_pos: ellie_core::defs::Cursor::default(),
                                                    },
                                                ],
                                                pos: ellie_core::defs::Cursor::default(),
                                            },
                                        )),
                                            None => None,
                                        },
                                        None => None,
                                    }
                                } else if parent_generic.rtype == "vector" {
                                    match generate_type_from_defining(
                                        parent_generic.generics[0].value.clone(),
                                        page_id,
                                        parser,
                                    ) {
                                        Some(t) => Some(types::Types::Vector(
                                            ellie_core::definite::types::vector::VectorType {
                                                collective: vec![
                                                    ellie_core::definite::types::vector::VectorEntry {
                                                        value: t,
                                                        location: ellie_core::defs::Cursor::default(),
                                                    },
                                                ],
                                                pos: ellie_core::defs::Cursor::default(),
                                            },
                                        )),
                                        None => None,
                                    }
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
                                            crate::deep_search_extensions::ProcessedDeepSearchItems::Class(matched_class) => {
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
                            DefinerCollecting::Function(function) => {
                                Some(types::Types::FunctionCall(
                                    ellie_core::definite::types::function_call::FunctionCall {
                                        target: Box::new(types::Types::Null),
                                        returning: *function.returning,
                                        target_pos: ellie_core::defs::Cursor::default(),
                                        params: vec![],
                                        pos: ellie_core::defs::Cursor::default(),
                                    }
                                ))
                            }
                            _ => unreachable!(),
                        }
                    }

                    fn resolve_chain(
                        reference_type: DefinerCollecting,
                        reference_pos: ellie_core::defs::Cursor,
                        page_id: u64,
                        parser: &mut crate::parser::Parser,
                    ) -> Result<Vec<Attribute>, Vec<error::Error>> {
                        let mut errors: Vec<error::Error> = Vec::new();
                        match reference_type.clone() {
                            ellie_core::definite::definers::DefinerCollecting::Array(_) => todo!(),
                            ellie_core::definite::definers::DefinerCollecting::Vector(_) => todo!(),
                            ellie_core::definite::definers::DefinerCollecting::Generic(generic) => {
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
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::Class(class_page) => {
                                            match parser.find_processed_page(class_page.inner_page_id).cloned() {
                                                Some(class_inner_page) => {
                                                    let attributes = class_inner_page.items.iter().filter_map(|item| {
                                                        match item.clone() {
                                                            Collecting::Variable(e) => {
                                                                let resolved_type = if e.has_type { e.rtype } else { resolve_type(e.value, class_inner_page.hash, parser, &mut errors) };
                                                                Some(Attribute {
                                                                    rtype: AttributeType::Property,
                                                                    name: e.name.clone(),
                                                                    value: resolved_type
                                                                })
                                                            },
                                                            Collecting::Function(e) => {
                                                                Some(Attribute {
                                                                    rtype: AttributeType::Method,
                                                                    name: e.name.clone(),
                                                                    value: DefinerCollecting::Function(
                                                                        ellie_core::definite::definers::FunctionType {
                                                                            params: e.parameters.iter().map(|param| {
                                                                                param.rtype.clone()
                                                                            }).collect::<Vec<_>>(),
                                                                            returning: Box::new(e.return_type),
                                                                        }
                                                                    )
                                                                })
                                                            },
                                                            Collecting::NativeFunction(e) => {
                                                                Some(Attribute {
                                                                    rtype: AttributeType::Method,
                                                                    name: e.name.clone(),
                                                                    value: DefinerCollecting::Function(
                                                                        ellie_core::definite::definers::FunctionType {
                                                                            params: e.parameters.iter().map(|param| {
                                                                                param.rtype.clone()
                                                                            }).collect::<Vec<_>>(),
                                                                            returning: Box::new(e.return_type),
                                                                        }
                                                                    )
                                                                })
                                                            }
                                                            _ => None,
                                                        }
                                                    }).collect::<Vec<_>>();
                                                    Ok(attributes)
                                                },
                                                None => {
                                                    unreachable!()
                                                }
                                            }
                                        },
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::Variable(_) => todo!(),
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::Function(_) => todo!(),
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::ImportReference(_) => todo!(),
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::None => todo!(),
                                    }
                                } else {
                                    errors.push(
                                        error::error_list::ERROR_S6.clone().build_with_path(
                                            vec![error::ErrorBuildField {
                                                key: "token".to_owned(),
                                                value: reference_type.to_string(),
                                            }],
                                            file!().to_owned(),
                                            parser.find_page(page_id).unwrap().path.clone(),
                                            reference_pos,
                                        ),
                                    );
                                    Err(errors)
                                }
                            }
                            ellie_core::definite::definers::DefinerCollecting::ParentGeneric(_) => {
                                todo!()
                            }
                            ellie_core::definite::definers::DefinerCollecting::Function(_) => {
                                let rtype = find_type("function".to_owned(), page_id, parser);
                                match resolve_chain(DefinerCollecting::Generic(rtype.unwrap()), ellie_core::defs::Cursor::default(), page_id, parser) {
                                    Ok(e) => {
                                        Ok(e)
                                    },
                                    Err(_) => todo!(),
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
                            DefinerCollecting::Array(_) => todo!(),
                            DefinerCollecting::Vector(_) => todo!(),
                            DefinerCollecting::Generic(_) => todo!(),
                            DefinerCollecting::ParentGeneric(_) => todo!(),
                            DefinerCollecting::Function(_) => todo!(),
                            DefinerCollecting::Cloak(_) => todo!(),
                            DefinerCollecting::Collective(_) => todo!(),
                            DefinerCollecting::Nullable(_) => todo!(),
                            DefinerCollecting::Dynamic => todo!(),
                        }
                    }

                    let reference_type =
                        resolve_type(found_reference, page_id, parser, &mut errors);
                    #[derive(Debug, EnumAsInner)]
                    enum LastEntry {
                        Type(types::Types),
                        Null,
                    }
                    let mut resolved_types = LastEntry::Null;
                    let mut last_chain_attributes = (
                        reference_type.clone(),
                        resolve_chain(reference_type, from.get_pos(), page_id, parser),
                    );
                    for chain in reference.data.chain {
                        match last_chain_attributes.1.clone() {
                            Ok(e) => {
                                let attribute = e.iter().find(|a| a.name == chain.value);
                                match attribute {
                                    Some(a) => {
                                        resolved_types = LastEntry::Type(
                                            generate_type_from_defining(
                                                a.value.clone(),
                                                page_id,
                                                parser,
                                            )
                                            .unwrap(),
                                        );
                                        last_chain_attributes = (
                                            a.value.clone(),
                                            resolve_chain(
                                                a.value.clone(),
                                                from.get_pos(),
                                                page_id,
                                                parser,
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
                                                file!().to_owned(),
                                                parser.find_page(page_id).unwrap().path.clone(),
                                                from.get_pos(),
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
                        //panic!("{:#?}", resolved_types);
                        Ok(resolved_types.as_type().unwrap().clone())
                    } else {
                        Err(errors)
                    }
                }
                Err(e) => Err(e),
            }
        }
        Processors::BraceReference(brace_reference) => {
            let index = process(*brace_reference.data.value, parser, page_id, ignore_hash);
            match index {
                Ok(index) => {
                    let index_type = resolve_type(index.clone(), page_id, parser, &mut errors);
                    let reference = process(
                        *brace_reference.data.reference,
                        parser,
                        page_id,
                        ignore_hash,
                    );
                    match reference {
                        Ok(found_reference) => {
                            let reference_type =
                                resolve_type(found_reference.clone(), page_id, parser, &mut errors);
                            // TODO Ellie should let developers implement indexable properties in classes
                            //Example
                            // class A {
                            //     pub v indexable_property : array<int>;
                            //
                            //     @index="GET"
                            //     fn get(index: int) : int {
                            //         ret indexable_property[index];
                            //     }
                            //
                            //     @index="SET"
                            //     fn set(index: int, value: int) {
                            //         indexable_property[index] = value;
                            //     }
                            //
                            // }
                            //
                            // v a = new A();
                            // a[0] = 1;
                            match reference_type.clone() {
                                ellie_core::definite::definers::DefinerCollecting::ParentGeneric(reference_generic) => {
                                    if reference_generic.rtype == "array" {
                                        match index_type.clone() {
                                            ellie_core::definite::definers::DefinerCollecting::Generic(index_generic_type) => {
                                                if index_generic_type.rtype == "int" {
                                                    Ok(types::Types::BraceReference(types::brace_reference::BraceReferenceType {
                                                        reference: Box::new(found_reference),
                                                        reference_pos: brace_reference.data.reference_pos,
                                                        brace_pos: brace_reference.data.brace_pos,
                                                        value: Box::new(index),
                                                        pos: brace_reference.data.pos,
                                                    }))
                                                } else {
                                                    errors.push(error::error_list::ERROR_S49.clone().build_with_path(
                                                        vec![error::ErrorBuildField {
                                                            key: "target".to_string(),
                                                            value: reference_type.to_string(),
                                                        },error::ErrorBuildField {
                                                            key: "token".to_string(),
                                                            value: index_type.to_string(),
                                                        }],
                                                        file!().to_owned(),
                                                        parser.find_page(page_id).unwrap().path.clone(),
                                                        brace_reference.data.brace_pos
                                                    ));
                                                    Err(errors)

                                                }
                                            },
                                            _ => {
                                                errors.push(error::error_list::ERROR_S49.clone().build_with_path(
                                                    vec![error::ErrorBuildField {
                                                        key: "target".to_string(),
                                                        value: reference_type.to_string(),
                                                    },error::ErrorBuildField {
                                                        key: "token".to_string(),
                                                        value: index_type.to_string(),
                                                    }],
                                                    file!().to_owned(),
                                                    parser.find_page(page_id).unwrap().path.clone(),
                                                    brace_reference.data.brace_pos
                                                ));
                                                Err(errors)
                                            }
                                        }
                                    } else if reference_generic.rtype == "cloak" {
                                        todo!("cloak index queries type not yet implemented")
                                    } else if reference_generic.rtype == "collective" {
                                        todo!("collective index queries type not yet implemented")
                                    } else {
                                        todo!("custom index queries type not yet implemented")
                                    }
                                },
                                _ => {
                                    errors.push(error::error_list::ERROR_S48.clone().build_with_path(
                                        vec![error::ErrorBuildField {
                                            key: "token".to_string(),
                                            value: reference_type.to_string(),
                                        }],
                                        file!().to_owned(),
                                        parser.find_page(page_id).unwrap().path.clone(),
                                        brace_reference.data.reference_pos
                                    ));
                                    Err(errors)
                                }
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        }
        Processors::FunctionCall(function_call) => {
            let index = process(*function_call.data.target, parser, page_id, ignore_hash);
            match index {
                Ok(index) => {
                    match index.clone() {
                        types::Types::FunctionCall(d) => {
                           Ok(
                               ellie_core::definite::types::Types::FunctionCall(
                                      ellie_core::definite::types::function_call::FunctionCall {
                                        target: Box::new(types::Types::Dynamic),
                                        target_pos: d.target_pos,
                                        returning: d.returning,
                                        params: d.params,
                                        pos: d.pos,
                                    }
                               )
                            )
                        },
                        _ => {
                            let reference_type =
                                resolve_type(index.clone(), page_id, parser, &mut errors);
                            errors.push(error::error_list::ERROR_S25.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: reference_type.to_string(),
                                }],
                                file!().to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                function_call.data.target_pos
                            ));
                            Err(errors)
                        }
                    }
                }
                Err(e) => Err(e),
            }
        }
        Processors::ClassCall(class_call) => match (*class_call.data.target).clone() {
            Processors::Integer(_) => {
                errors.push(error::error_list::ERROR_S11.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "int".to_string(),
                    }],
                    file!().to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    class_call.data.keyword_pos,
                ));
                Err(errors)
            }
            Processors::Float(_) => {
                errors.push(error::error_list::ERROR_S11.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "float".to_string(),
                    }],
                    file!().to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    class_call.data.keyword_pos,
                ));
                Err(errors)
            }
            Processors::Char(_) => {
                errors.push(error::error_list::ERROR_S11.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "char".to_string(),
                    }],
                    file!().to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    class_call.data.keyword_pos,
                ));
                Err(errors)
            }
            Processors::String(_) => {
                errors.push(error::error_list::ERROR_S11.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "string".to_string(),
                    }],
                    file!().to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    class_call.data.keyword_pos,
                ));
                Err(errors)
            }
            Processors::Collective(_) => {
                errors.push(error::error_list::ERROR_S11.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "collective".to_string(),
                    }],
                    file!().to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    class_call.data.keyword_pos,
                ));
                Err(errors)
            }
            Processors::Cloak(cloak) => {
                if cloak.data.collective.len() == 1 {
                    unimplemented!()
                } else {
                    errors.push(error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "cloak".to_string(),
                        }],
                        file!().to_owned(),
                        parser.find_page(page_id).unwrap().path.clone(),
                        class_call.data.keyword_pos,
                    ));
                    Err(errors)
                }
            }
            Processors::Array(_) => {
                errors.push(error::error_list::ERROR_S11.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "collective".to_string(),
                    }],
                    file!().to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    class_call.data.keyword_pos,
                ));
                Err(errors)
            }
            Processors::Variable(variable) => {
                let deep_search_result = parser.deep_search(
                    page_id,
                    variable.data.value.clone(),
                    ignore_hash,
                    Vec::new(),
                    0,
                );

                if deep_search_result.found {
                    match deep_search_result.found_item {
                        crate::parser::DeepSearchItems::Class(e) => {
                            let undefined_generics = class_call
                                .data
                                .generic_parameters
                                .iter()
                                .filter_map(
                                    |g| match crate::processors::definer_processor::process(
                                        g.value.clone(),
                                        parser,
                                        page_id,
                                        ignore_hash,
                                    ) {
                                        Ok(_) => None,
                                        Err(err) => {
                                            errors.extend(err);
                                            Some(g)
                                        }
                                    },
                                )
                                .collect::<Vec<_>>();
                            if e.generic_definings.len() != class_call.data.generic_parameters.len()
                            {
                                let mut error =
                                    error::error_list::ERROR_S44.clone().build_with_path(
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: e.generic_definings.len().to_string(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_string(),
                                                value: class_call
                                                    .data
                                                    .generic_parameters
                                                    .len()
                                                    .to_string(),
                                            },
                                        ],
                                        file!().to_owned(),
                                        parser.find_page(page_id).unwrap().path.clone(),
                                        class_call.data.target_pos,
                                    );
                                error.builded_message.builded +=
                                    " (https://github.com/behemehal/Ellie-Language/issues/59)";
                                error.reference_block = Some((
                                    e.name_pos,
                                    parser.find_page(page_id).unwrap().path.clone(),
                                ));
                                error.reference_message = "Defined here".to_owned();
                                errors.push(error);
                                Err(errors)
                            } else if undefined_generics.len() > 0 {
                                for g in undefined_generics {
                                    errors.push(
                                        error::error_list::ERROR_S6.clone().build_with_path(
                                            vec![error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: g.value.clone().to_definite().to_string(),
                                            }],
                                            file!().to_owned(),
                                            parser.find_page(page_id).unwrap().path.clone(),
                                            g.pos,
                                        ),
                                    );
                                }
                                Err(errors)
                            } else if let Some(_) = e.body.iter().find_map(|x| match x {
                                ellie_tokenizer::processors::items::Processors::Constructor(e) => {
                                    Some(e)
                                }
                                _ => None,
                            }) {
                                todo!();
                                Ok(types::Types::ClassCall(
                                    ellie_core::definite::types::class_call::ClassCall {
                                        target: Box::new(class_call.data.target.clone().to_definite()),
                                        keyword_pos: class_call.data.keyword_pos,
                                        target_pos: class_call.data.target_pos,
                                        generic_parameters: class_call.data.generic_parameters.iter().map(|x| ellie_core::definite::types::class_call::ClassCallGenericParameter { value: ellie_core::definite::Converter::to_definite(x.value.clone()), pos: x.pos }).collect::<Vec<_>>(),
                                        params: class_call.data.parameters.iter().map(|x| types::class_call::ClassCallParameter { value: x.value.to_definite(), pos: x.pos }).collect::<Vec<_>>(),
                                        pos: class_call.data.pos,
                                    },
                                ))
                            } else if class_call.data.parameters.len() != 0 {
                                errors.push(
                                    error::error_list::ERROR_S44.clone().build_with_path(
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token".to_string(),
                                                value: "0".to_string(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_string(),
                                                value: class_call
                                                    .data
                                                    .generic_parameters
                                                    .len()
                                                    .to_string(),
                                            },
                                        ],
                                        file!().to_owned(),
                                        parser.find_page(page_id).unwrap().path.clone(),
                                        class_call.data.target_pos,
                                    ),
                                );
                                Err(errors)
                            } else {
                                Ok(types::Types::ClassCall(
                                    ellie_core::definite::types::class_call::ClassCall {
                                        target: Box::new(ellie_core::definite::types::Types::VariableType(
                                            ellie_core::definite::types::variable::VariableType {
                                                value: variable.data.value,
                                                pos: class_call.data.target_pos,
                                                reference: e.hash,
                                            },
                                        )),
                                        keyword_pos: class_call.data.keyword_pos,
                                        target_pos: class_call.data.target_pos,
                                        generic_parameters: class_call.data.generic_parameters.iter().map(|x| {
                                            let definite_type = match x.value.clone() {
                                                ellie_tokenizer::syntax::items::definers::DefinerTypes::ParentGeneric(_) => todo!(),
                                                ellie_tokenizer::syntax::items::definers::DefinerTypes::Generic(generic) => {
                                                    //ellie_core::definite::Converter::to_definite(x.value.clone())
                                                    let found_type = find_type(generic.rtype.clone(), page_id, parser).unwrap();
                                                    ellie_core::definite::definers::DefinerCollecting::Generic(
                                                        ellie_core::definite::definers::GenericType {
                                                            rtype: generic.rtype.clone(),
                                                            pos: x.pos,
                                                            hash: found_type.hash,
                                                        }
                                                    )
                                                },
                                                _ => todo!(),
                                            };


                                            ellie_core::definite::types::class_call::ClassCallGenericParameter {
                                                value: definite_type,
                                                pos: x.pos
                                            }
                                        }).collect::<Vec<_>>(),
                                        params: class_call.data.parameters.iter().map(|x| types::class_call::ClassCallParameter { value: x.value.to_definite(), pos: x.pos }).collect::<Vec<_>>(),
                                        pos: class_call.data.pos,
                                    },
                                ))
                            }
                        }
                        crate::parser::DeepSearchItems::Variable(e) => {
                            errors.push(error::error_list::ERROR_S31.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "variable".to_string(),
                                }],
                                file!().to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.pos,
                            ));
                            Err(errors)
                        }
                        crate::parser::DeepSearchItems::Function(e) => {
                            errors.push(error::error_list::ERROR_S31.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "function".to_string(),
                                }],
                                file!().to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.name_pos,
                            ));
                            Err(errors)
                        }
                        crate::parser::DeepSearchItems::ImportReference(e) => {
                            errors.push(error::error_list::ERROR_S31.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "variable".to_string(),
                                }],
                                file!().to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                e.reference_pos,
                            ));
                            Err(errors)
                        }
                        crate::parser::DeepSearchItems::SelfItem(_) => todo!(),
                        crate::parser::DeepSearchItems::GenericItem(_) => todo!(),
                        crate::parser::DeepSearchItems::FunctionParameter(_) => {
                            unimplemented!()
                        }
                        crate::parser::DeepSearchItems::ConstructorParameter(_) => {
                            unimplemented!()
                        }
                        _ => {
                            errors.push(error::error_list::ERROR_S31.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: variable.data.value,
                                }],
                                file!().to_owned(),
                                parser.find_page(page_id).unwrap().path.clone(),
                                variable.data.pos,
                            ));
                            Err(errors)
                        }
                    }
                } else {
                    errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: variable.data.value,
                        }],
                        file!().to_owned(),
                        parser.find_page(page_id).unwrap().path.clone(),
                        variable.data.pos,
                    ));
                    Err(errors)
                }
            }
            Processors::Negative(_) => {
                errors.push(error::error_list::ERROR_S11.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "bool".to_string(),
                    }],
                    file!().to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    class_call.data.keyword_pos,
                ));
                Err(errors)
            }
            Processors::ClassCall(_) => {
                errors.push(error::error_list::ERROR_S11.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: "classCall".to_string(),
                    }],
                    file!().to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    class_call.data.keyword_pos,
                ));
                Err(errors)
            }
            Processors::Operator(_) => todo!(),
            Processors::Reference(_) => todo!(),
            Processors::BraceReference(_) => todo!(),
            Processors::FunctionCall(_) => todo!(),
            Processors::AsKeyword(_) => todo!(),
            Processors::NullResolver(_) => todo!(),
        },
        Processors::Cloak(_) => todo!("cloak type not yet implemented"),
        Processors::Collective(_) => todo!("collective type not yet implemented"),
        Processors::AsKeyword(as_keyword) => {
            match process(
                *as_keyword.data.target,
                parser,
                page_id,
                ignore_hash.clone(),
            ) {
                Ok(resolved_types) => {
                    match crate::processors::definer_processor::process(
                        as_keyword.data.rtype.definer_type,
                        parser,
                        page_id,
                        ignore_hash,
                    ) {
                        Ok(resolved_definer) => {
                            Ok(types::Types::AsKeyword(types::as_keyword::AsKeyword {
                                target: Box::new(resolved_types),
                                pos: as_keyword.data.pos,
                                target_pos: as_keyword.data.target_pos,
                                type_pos: as_keyword.data.type_pos,
                                rtype: resolved_definer,
                            }))
                        }
                        Err(type_errors) => {
                            errors.extend(type_errors);
                            Err(errors)
                        }
                    }
                }
                Err(val_errors) => {
                    errors.extend(val_errors);
                    Err(errors)
                }
            }
        }
        Processors::NullResolver(null_resolver) => {
            match process(*null_resolver.target, parser, page_id, ignore_hash.clone()) {
                Ok(resolved_types) => Ok(types::Types::NullResolver(
                    types::null_resolver::NullResolver {
                        target: Box::new(resolved_types),
                        pos: null_resolver.pos,
                        target_pos: null_resolver.target_pos,
                    },
                )),
                Err(val_errors) => {
                    errors.extend(val_errors);
                    Err(errors)
                }
            }
        }
        _ => Ok(from.to_definite()),
    }
}
