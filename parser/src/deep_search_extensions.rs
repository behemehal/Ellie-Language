use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use ellie_core::{
    definite::{definers, items::Collecting, types::Types},
    defs, error,
};
use ellie_tokenizer::tokenizer::Dependency;
use enum_as_inner::EnumAsInner;

use crate::parser::{Parser, ProcessedPage};

/*
    This folder contains parser extensions for deep search.
*/

#[derive(Debug)]
pub enum DeepTypeResult {
    Integer(ellie_core::definite::types::integer::IntegerType),
    Float(ellie_core::definite::types::float::FloatType),
    Bool(ellie_core::definite::types::bool::BoolType),
    String(ellie_core::definite::types::string::StringType),
    Char(ellie_core::definite::types::ellie_char::CharType),
    Collective(ellie_core::definite::types::collective::CollectiveType),
    Operator(ellie_core::definite::types::operator::OperatorType),
    Cloak(ellie_core::definite::types::cloak::CloakType),
    Array(ellie_core::definite::types::array::ArrayType),
    Vector(ellie_core::definite::types::vector::VectorType),
    ClassCall(ellie_core::definite::types::class_call::ClassCall),
    Function(ellie_core::definite::types::function::Function),
    FunctionCall(ellie_core::definite::types::function_call::FunctionCall),
    BraceReference(ellie_core::definite::types::brace_reference::BraceReferenceType),
    Void,
    Null,
    Dynamic,
    NotFound,
}

fn iterate_deep_type(
    parser: &mut Parser,
    page_id: u64,
    rtype: Types,
    errors: &mut Vec<error::Error>,
) -> DeepTypeResult {
    match rtype.clone() {
        Types::Integer(integer) => DeepTypeResult::Integer(integer),
        Types::Float(float) => DeepTypeResult::Float(float),
        Types::String(string) => DeepTypeResult::String(string),
        Types::Char(char) => DeepTypeResult::Char(char),
        Types::Collective(collective) => DeepTypeResult::Collective(collective),
        Types::Reference(reference) => {
            #[derive(Debug, Clone, PartialEq)]
            enum AttributeType {
                Property,
                Method,
            }

            #[derive(Debug, Clone)]
            struct Attribute {
                rtype: AttributeType,
                name: String,
                value: definers::DefinerCollecting,
            }

            fn generate_type_from_defining(
                rtype: ellie_core::definite::definers::DefinerCollecting,
                page_id: u64,
                parser: &mut crate::parser::Parser,
            ) -> Option<Types> {
                match rtype {
                    definers::DefinerCollecting::Generic(generic) => {
                        if generic.rtype == "int" {
                            Some(Types::Integer(
                                ellie_core::definite::types::integer::IntegerType {
                                    value: ellie_core::definite::types::integer::IntegerSize::I8(0),
                                    rtype: ellie_core::definite::types::integer::IntegerTypes::I8,
                                    pos: ellie_core::defs::Cursor::default(),
                                },
                            ))
                        } else if generic.rtype == "float" {
                            Some(Types::Float(
                                ellie_core::definite::types::float::FloatType {
                                    value: ellie_core::definite::types::float::FloatSize::F32(0.0),
                                    rtype: ellie_core::definite::types::float::FloatTypes::F32,
                                    pos: ellie_core::defs::Cursor::default(),
                                },
                            ))
                        } else if generic.rtype == "string" {
                            Some(Types::String(
                                ellie_core::definite::types::string::StringType {
                                    value: "".to_owned(),
                                    pos: ellie_core::defs::Cursor::default(),
                                },
                            ))
                        } else if generic.rtype == "bool" {
                            Some(Types::Bool(ellie_core::definite::types::bool::BoolType {
                                value: true,
                            }))
                        } else if generic.rtype == "dyn" {
                            Some(Types::Dynamic)
                        } else if generic.rtype == "void" {
                            Some(Types::Void)
                        } else if generic.rtype == "char" {
                            Some(Types::Char(
                                ellie_core::definite::types::ellie_char::CharType { value: '\0' },
                            ))
                        } else if generic.rtype == "null" {
                            Some(Types::Null)
                        } else {
                            let hash_deep_search = crate::deep_search_extensions::deep_search_hash(
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
                                                        Types::ClassCall(
                                                            ellie_core::definite::types::class_call::ClassCall {
                                                                target: Box::new(Types::VariableType(
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
                    definers::DefinerCollecting::ParentGeneric(parent_generic) => {
                        if parent_generic.rtype == "array" {
                            match generate_type_from_defining(
                                parent_generic.generics[0].value.clone(),
                                page_id,
                                parser,
                            ) {
                                Some(t) => Some(Types::Array(
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
                                match generate_type_from_defining(generic.value, page_id, parser) {
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
                                Some(Types::Cloak(
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
                                            Some(t) => Some(Types::Collective(
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
                                Some(t) => Some(Types::Vector(
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
                            let hash_deep_search = crate::deep_search_extensions::deep_search_hash(
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
                                                        Types::ClassCall(
                                                            ellie_core::definite::types::class_call::ClassCall {
                                                                target: Box::new(Types::VariableType(
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
                    definers::DefinerCollecting::Function(function) => Some(Types::Function(
                        ellie_core::definite::types::function::Function {
                            parameters: function
                                .params
                                .iter()
                                .map(|parameter| {
                                    ellie_core::definite::types::function::FunctionParameter {
                                        name: "anonymous".to_string(),
                                        rtype: Some(parameter.clone()),
                                        pos: defs::Cursor::default(),
                                    }
                                })
                                .collect::<Vec<_>>(),
                            return_type: *function.returning,
                            has_parameter_definings: false,
                            arrow_function: false,
                            inside_code: vec![],
                            return_pos: defs::Cursor::default(),
                        },
                    )),
                    _ => unreachable!(),
                }
            }

            fn resolve_chain(
                reference_type: definers::DefinerCollecting,
                reference_pos: ellie_core::defs::Cursor,
                page_id: u64,
                parser: &mut crate::parser::Parser,
            ) -> Result<Vec<Attribute>, Vec<error::Error>> {
                let mut errors: Vec<error::Error> = Vec::new();
                match reference_type.clone() {
                    ellie_core::definite::definers::DefinerCollecting::Array(_) => todo!(),
                    ellie_core::definite::definers::DefinerCollecting::Vector(_) => todo!(),
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
                                                                    value: definers::DefinerCollecting::Function(
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
                                                                    value: definers::DefinerCollecting::Function(
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
                            errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: reference_type.to_string(),
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                parser.find_page(page_id).unwrap().path.clone(),
                                reference_pos,
                            ));
                            Err(errors)
                        }
                    }
                    ellie_core::definite::definers::DefinerCollecting::ParentGeneric(_) => {
                        todo!()
                    }
                    ellie_core::definite::definers::DefinerCollecting::Function(_) => {
                        let rtype = find_type("function".to_owned(), page_id, parser);
                        match resolve_chain(
                            definers::DefinerCollecting::Generic(rtype.unwrap()),
                            ellie_core::defs::Cursor::default(),
                            page_id,
                            parser,
                        ) {
                            Ok(e) => Ok(e),
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
                }
            }

            let reference_type = resolve_type(*reference.reference, page_id, parser, errors);
            #[derive(Debug, EnumAsInner)]
            enum LastEntry {
                Type(Types),
                Null,
            }
            let mut resolved_types = LastEntry::Null;
            let mut last_chain_attributes = (
                reference_type.clone(),
                resolve_chain(reference_type, reference.pos, page_id, parser),
            );
            for chain in reference.chain {
                match last_chain_attributes.1.clone() {
                    Ok(e) => {
                        let attribute = e.iter().find(|a| a.name == chain.value);
                        match attribute {
                            Some(a) => {
                                resolved_types = LastEntry::Type(
                                    generate_type_from_defining(a.value.clone(), page_id, parser)
                                        .unwrap(),
                                );
                                last_chain_attributes = (
                                    a.value.clone(),
                                    resolve_chain(a.value.clone(), chain.pos, page_id, parser),
                                );
                            }
                            None => {
                                errors.push(error::error_list::ERROR_S42.clone().build_with_path(
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
                                    parser.find_page(page_id).unwrap().path.clone(),
                                    chain.pos,
                                ));
                            }
                        }
                    }
                    Err(err) => {
                        errors.extend(err);
                    }
                }
            }

            resolve_deep_type(
                parser,
                page_id,
                resolved_types.as_type().unwrap().clone(),
                errors,
            )
        }
        Types::BraceReference(e) => {
            let resolved_reference =
                resolve_deep_type(parser, page_id, *e.reference.clone(), errors);
            let resolved_index = resolve_deep_type(parser, page_id, *e.value, errors);

            if matches!(resolved_reference, DeepTypeResult::NotFound)
                || matches!(resolved_index, DeepTypeResult::NotFound)
            {
                DeepTypeResult::NotFound
            } else {
                DeepTypeResult::BraceReference(
                    ellie_core::definite::types::brace_reference::BraceReferenceType {
                        reference: Box::new(match resolved_reference {
                            DeepTypeResult::Integer(e) => Types::Integer(e),
                            DeepTypeResult::Float(e) => Types::Float(e),
                            DeepTypeResult::Bool(e) => Types::Bool(e),
                            DeepTypeResult::String(e) => Types::String(e),
                            DeepTypeResult::Char(e) => Types::Char(e),
                            DeepTypeResult::Collective(e) => Types::Collective(e),
                            DeepTypeResult::Operator(e) => Types::Operator(e),
                            DeepTypeResult::Cloak(e) => Types::Cloak(e),
                            DeepTypeResult::Array(e) => Types::Array(e),
                            DeepTypeResult::Vector(e) => Types::Vector(e),
                            DeepTypeResult::ClassCall(e) => Types::ClassCall(e),
                            DeepTypeResult::FunctionCall(e) => Types::FunctionCall(e),
                            DeepTypeResult::BraceReference(e) => Types::BraceReference(e),
                            DeepTypeResult::Dynamic => Types::Dynamic,
                            DeepTypeResult::Void => unreachable!(),
                            DeepTypeResult::Null => unreachable!(),
                            DeepTypeResult::NotFound => {
                                unreachable!("cannot find reference: {:?}", *e.reference)
                            }
                            DeepTypeResult::Function(_) => todo!(),
                        }),
                        reference_pos: e.reference_pos,
                        brace_pos: e.brace_pos,
                        value: Box::new(match resolved_index {
                            DeepTypeResult::Integer(e) => Types::Integer(e),
                            DeepTypeResult::Float(e) => Types::Float(e),
                            DeepTypeResult::Bool(e) => Types::Bool(e),
                            DeepTypeResult::String(e) => Types::String(e),
                            DeepTypeResult::Char(e) => Types::Char(e),
                            DeepTypeResult::Collective(e) => Types::Collective(e),
                            DeepTypeResult::Operator(e) => Types::Operator(e),
                            DeepTypeResult::Cloak(e) => Types::Cloak(e),
                            DeepTypeResult::Array(e) => Types::Array(e),
                            DeepTypeResult::Vector(e) => Types::Vector(e),
                            DeepTypeResult::ClassCall(e) => Types::ClassCall(e),
                            DeepTypeResult::FunctionCall(e) => Types::FunctionCall(e),
                            DeepTypeResult::BraceReference(e) => Types::BraceReference(e),
                            DeepTypeResult::Void => unreachable!(),
                            DeepTypeResult::Null => unreachable!(),
                            DeepTypeResult::NotFound => unreachable!(),
                            DeepTypeResult::Dynamic => Types::Dynamic,
                            DeepTypeResult::Function(e) => Types::Function(e),
                        }),
                        pos: e.pos,
                    },
                )
            }
        }
        Types::Operator(_) => todo!(),
        Types::Cloak(cloak) => {
            if cloak.collective.len() == 1 {
                iterate_deep_type(
                    parser,
                    page_id,
                    cloak.collective.last().unwrap().clone().value,
                    errors,
                )
            } else {
                DeepTypeResult::Cloak(cloak)
            }
        }
        Types::Array(array) => {
            let mut collective = vec![];
            for i in array.collective {
                let resolved_collective = resolve_deep_type(parser, page_id, i.value, errors);
                match resolved_collective {
                    DeepTypeResult::Integer(integer_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Integer(integer_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Float(float_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Float(float_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Bool(bool_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Bool(bool_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::String(string_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::String(string_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Char(char_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Char(char_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Collective(collective_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Collective(collective_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Operator(operator_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Operator(operator_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Cloak(cloak_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Cloak(cloak_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Array(array_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Array(array_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Vector(vector_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Vector(vector_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::ClassCall(class_call) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::ClassCall(class_call),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::FunctionCall(function_call) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::FunctionCall(function_call),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Void => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Void,
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Null => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Null,
                            location: i.location,
                        });
                    }
                    DeepTypeResult::NotFound => {
                        return DeepTypeResult::NotFound;
                    }
                    DeepTypeResult::BraceReference(brace_reference) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::BraceReference(brace_reference),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Dynamic => todo!(),
                    DeepTypeResult::Function(e) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Function(e),
                            location: i.location,
                        });
                    }
                }
            }
            DeepTypeResult::Array(ellie_core::definite::types::array::ArrayType {
                collective,
                pos: array.pos,
            })
        }
        Types::Vector(_) => todo!(),
        Types::ClassCall(class_call) => DeepTypeResult::ClassCall(class_call),
        Types::FunctionCall(e) => DeepTypeResult::FunctionCall(e),
        Types::NullResolver(null_resolver) => {
            let from_type = resolve_type(*null_resolver.target.clone(), page_id, parser, errors);
            match from_type {
                definers::DefinerCollecting::ParentGeneric(e) => {
                    fn generate_type_from_defining(
                        rtype: definers::DefinerCollecting,
                        page_id: u64,
                        parser: &mut Parser,
                    ) -> Option<Types> {
                        match rtype {
                            definers::DefinerCollecting::Generic(generic) => {
                                if generic.rtype == "int" {
                                    Some(Types::Integer(
                                        ellie_core::definite::types::integer::IntegerType {
                                            value: ellie_core::definite::types::integer::IntegerSize::I8(0),
                                            rtype: ellie_core::definite::types::integer::IntegerTypes::I8,
                                            pos: defs::Cursor::default(),
                                        },
                                    ))
                                } else if generic.rtype == "float" {
                                    Some(Types::Float(
                                        ellie_core::definite::types::float::FloatType {
                                            value:
                                                ellie_core::definite::types::float::FloatSize::F32(
                                                    0.0,
                                                ),
                                            rtype:
                                                ellie_core::definite::types::float::FloatTypes::F32,
                                            pos: defs::Cursor::default(),
                                        },
                                    ))
                                } else if generic.rtype == "string" {
                                    Some(Types::String(
                                        ellie_core::definite::types::string::StringType {
                                            value: "".to_owned(),
                                            pos: defs::Cursor::default(),
                                        },
                                    ))
                                } else if generic.rtype == "bool" {
                                    Some(Types::Bool(ellie_core::definite::types::bool::BoolType {
                                        value: true,
                                    }))
                                } else if generic.rtype == "dyn" {
                                    Some(Types::Dynamic)
                                } else if generic.rtype == "void" {
                                    Some(Types::Void)
                                } else if generic.rtype == "char" {
                                    Some(Types::Char(
                                        ellie_core::definite::types::ellie_char::CharType {
                                            value: '\0',
                                        },
                                    ))
                                } else if generic.rtype == "null" {
                                    Some(Types::Null)
                                } else {
                                    let hash_deep_search =
                                        deep_search_hash(parser, page_id, generic.hash, vec![], 0);
                                    if hash_deep_search.found {
                                        match hash_deep_search.found_item {
                                            ProcessedDeepSearchItems::Class(matched_class) => {
                                                if matched_class.generic_definings.is_empty() {
                                                    Some(
                                                        Types::ClassCall(
                                                            ellie_core::definite::types::class_call::ClassCall {
                                                                target: Box::new(Types::VariableType(
                                                                    ellie_core::definite::types::variable::VariableType {
                                                                        value: matched_class.name.clone(),
                                                                        reference: matched_class.hash,
                                                                        pos: defs::Cursor::default(),
                                                                    },
                                                                )),
                                                                generic_parameters: vec![],
                                                                keyword_pos: defs::Cursor::default(),
                                                                pos: defs::Cursor::default(),
                                                                target_pos: defs::Cursor::default(),
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
                            definers::DefinerCollecting::ParentGeneric(parent_generic) => {
                                if parent_generic.rtype == "array" {
                                    match generate_type_from_defining(
                                        parent_generic.generics[0].value.clone(),
                                        page_id,
                                        parser,
                                    ) {
                                        Some(t) => Some(Types::Array(
                                            ellie_core::definite::types::array::ArrayType {
                                                collective: vec![
                                                    ellie_core::definite::types::array::ArrayEntry {
                                                        value: t,
                                                        location: defs::Cursor::default(),
                                                    },
                                                ],
                                                pos: defs::Cursor::default(),
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
                                                    location: defs::Cursor::default(),
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
                                        Some(Types::Cloak(
                                            ellie_core::definite::types::cloak::CloakType {
                                                collective: cloak_entries,
                                                pos: defs::Cursor::default(),
                                            },
                                        ))
                                    }
                                } else if parent_generic.rtype == "collective" {
                                    match generate_type_from_defining(parent_generic.generics[0].value.clone(),  page_id,
                                    parser,) {
                                        Some(k) =>
                                        match generate_type_from_defining(parent_generic.generics[1].value.clone(), page_id,
                                        parser,) {
                                            Some(t) => Some(Types::Collective(
                                            ellie_core::definite::types::collective::CollectiveType {
                                                entries: vec![
                                                    ellie_core::definite::types::collective::CollectiveEntry {
                                                        key: k,
                                                        value: t,
                                                        key_pos: defs::Cursor::default(),
                                                        value_pos: defs::Cursor::default(),
                                                    },
                                                ],
                                                pos: defs::Cursor::default(),
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
                                        Some(t) => Some(Types::Vector(
                                            ellie_core::definite::types::vector::VectorType {
                                                collective: vec![
                                                    ellie_core::definite::types::vector::VectorEntry {
                                                        value: t,
                                                        location: defs::Cursor::default(),
                                                    },
                                                ],
                                                pos: defs::Cursor::default(),
                                            },
                                        )),
                                        None => None,
                                    }
                                } else {
                                    let hash_deep_search = deep_search_hash(
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
                                                        Types::ClassCall(
                                                            ellie_core::definite::types::class_call::ClassCall {
                                                                target: Box::new(Types::VariableType(
                                                                    ellie_core::definite::types::variable::VariableType {
                                                                        value: matched_class.name.clone(),
                                                                        reference: matched_class.hash,
                                                                        pos: defs::Cursor::default(),
                                                                    },
                                                                )),
                                                                generic_parameters: parent_generic.generics.iter().map(|generic| {
                                                                    ellie_core::definite::types::class_call::ClassCallGenericParameter {
                                                                        value: generic.value.clone(),
                                                                        pos: defs::Cursor::default(),
                                                                    }
                                                                }).collect::<Vec<_>>(),
                                                                keyword_pos: defs::Cursor::default(),
                                                                pos: defs::Cursor::default(),
                                                                target_pos: defs::Cursor::default(),
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
                            _ => unreachable!(),
                        }
                    }

                    if e.rtype == "nullAble" {
                        let resolved_type = generate_type_from_defining(
                            e.generics[0].value.clone(),
                            page_id,
                            parser,
                        );
                        match resolved_type {
                            Some(types) => match types {
                                Types::Integer(e) => DeepTypeResult::Integer(e),
                                Types::Float(e) => DeepTypeResult::Float(e),
                                Types::Bool(e) => DeepTypeResult::Bool(e),
                                Types::String(e) => DeepTypeResult::String(e),
                                Types::Char(e) => DeepTypeResult::Char(e),
                                Types::Collective(e) => DeepTypeResult::Collective(e),
                                Types::Cloak(e) => DeepTypeResult::Cloak(e),
                                Types::Array(e) => DeepTypeResult::Array(e),
                                Types::Vector(e) => DeepTypeResult::Vector(e),
                                Types::Dynamic => DeepTypeResult::Dynamic,
                                Types::ClassCall(e) => DeepTypeResult::ClassCall(e),
                                _ => unreachable!(),
                            },
                            None => {
                                let path = parser.find_page(page_id).unwrap().path.clone();
                                errors.push(error::error_list::ERROR_S51.clone().build_with_path(
                                    vec![],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    path,
                                    null_resolver.pos,
                                ));
                                DeepTypeResult::NotFound
                            }
                        }
                    } else {
                        let path = parser.find_page(page_id).unwrap().path.clone();
                        errors.push(error::error_list::ERROR_S51.clone().build_with_path(
                            vec![],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            path,
                            null_resolver.pos,
                        ));
                        DeepTypeResult::NotFound
                    }
                }
                _ => {
                    let path = parser.find_page(page_id).unwrap().path.clone();
                    errors.push(error::error_list::ERROR_S51.clone().build_with_path(
                        vec![],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path,
                        null_resolver.pos,
                    ));
                    DeepTypeResult::NotFound
                }
            }
        }
        Types::Negative(_) => todo!(),
        Types::VariableType(variable) => {
            let hash_deep_search =
                deep_search(parser, page_id, variable.value.clone(), None, vec![], 0);
            if hash_deep_search.found {
                match hash_deep_search.found_item {
                    ProcessedDeepSearchItems::Class(e) => {
                        //This is the class elements defining class. We're virtually building it
                        //See lib/class.ei
                        let class_class =
                            deep_search(parser, page_id, "class".to_owned(), None, vec![], 0);
                        if class_class.found {
                            if let ProcessedDeepSearchItems::Class(e) = class_class.found_item {
                                DeepTypeResult::ClassCall(
                                    ellie_core::definite::types::class_call::ClassCall {
                                        target: Box::new(Types::VariableType(
                                            ellie_core::definite::types::variable::VariableType {
                                                value: "class".to_owned(),
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
                                )
                            } else {
                                unreachable!("Ellie must ensure that class is a class, and no one can replace it");
                            }
                        } else {
                            let path = parser.find_page(page_id).unwrap().path.clone();
                            errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "class".to_string(),
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                path,
                                e.pos,
                            ));
                            DeepTypeResult::NotFound
                        }
                    }
                    ProcessedDeepSearchItems::Variable(e) => {
                        iterate_deep_type(parser, page_id, e.value, errors)
                    }
                    ProcessedDeepSearchItems::Function(_) => todo!(),
                    ProcessedDeepSearchItems::ImportReference(_) => todo!(),
                    ProcessedDeepSearchItems::None => todo!(),
                }
            } else {
                let path = parser.find_page(page_id).unwrap().path.clone();
                parser
                    .informations
                    .push(&error::error_list::ERROR_S6.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: variable.value.to_owned(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path,
                        variable.pos,
                    ));
                DeepTypeResult::NotFound
            }
        }
        Types::AsKeyword(as_keyword) => {
            let targeted = resolve_type(*as_keyword.target.clone(), page_id, parser, errors);

            fn generate_type_from_defining(rtype: definers::DefinerCollecting) -> Option<Types> {
                match rtype {
                    definers::DefinerCollecting::Generic(generic) => {
                        if generic.rtype == "int" {
                            Some(Types::Integer(
                                ellie_core::definite::types::integer::IntegerType {
                                    value: ellie_core::definite::types::integer::IntegerSize::I8(0),
                                    rtype: ellie_core::definite::types::integer::IntegerTypes::I8,
                                    pos: defs::Cursor::default(),
                                },
                            ))
                        } else if generic.rtype == "float" {
                            Some(Types::Float(
                                ellie_core::definite::types::float::FloatType {
                                    value: ellie_core::definite::types::float::FloatSize::F32(0.0),
                                    rtype: ellie_core::definite::types::float::FloatTypes::F32,
                                    pos: defs::Cursor::default(),
                                },
                            ))
                        } else if generic.rtype == "string" {
                            Some(Types::String(
                                ellie_core::definite::types::string::StringType {
                                    value: "".to_owned(),
                                    pos: defs::Cursor::default(),
                                },
                            ))
                        } else if generic.rtype == "bool" {
                            Some(Types::Bool(ellie_core::definite::types::bool::BoolType {
                                value: true,
                            }))
                        } else if generic.rtype == "dyn" {
                            Some(Types::Dynamic)
                        } else if generic.rtype == "void" {
                            Some(Types::Void)
                        } else if generic.rtype == "char" {
                            Some(Types::Char(
                                ellie_core::definite::types::ellie_char::CharType { value: '\0' },
                            ))
                        } else if generic.rtype == "null" {
                            Some(Types::Null)
                        } else {
                            None
                        }
                    }
                    definers::DefinerCollecting::ParentGeneric(parent_generic) => {
                        if parent_generic.rtype == "array" {
                            match generate_type_from_defining(
                                parent_generic.generics[0].value.clone(),
                            ) {
                                Some(t) => Some(Types::Array(
                                    ellie_core::definite::types::array::ArrayType {
                                        collective: vec![
                                            ellie_core::definite::types::array::ArrayEntry {
                                                value: t,
                                                location: defs::Cursor::default(),
                                            },
                                        ],
                                        pos: defs::Cursor::default(),
                                    },
                                )),
                                None => None,
                            }
                        } else if parent_generic.rtype == "cloak" {
                            let mut cloak_entries = vec![];
                            let mut unresolved_element_available = false;
                            for generic in parent_generic.generics {
                                match generate_type_from_defining(generic.value) {
                                    Some(t) => cloak_entries.push(
                                        ellie_core::definite::types::cloak::CloakEntry {
                                            value: t,
                                            location: defs::Cursor::default(),
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
                                Some(Types::Cloak(
                                    ellie_core::definite::types::cloak::CloakType {
                                        collective: cloak_entries,
                                        pos: defs::Cursor::default(),
                                    },
                                ))
                            }
                        } else if parent_generic.rtype == "collective" {
                            match generate_type_from_defining(parent_generic.generics[0].value.clone()) {
                                Some(k) =>
                                match generate_type_from_defining(parent_generic.generics[1].value.clone()) {
                                    Some(t) => Some(Types::Collective(
                                    ellie_core::definite::types::collective::CollectiveType {
                                        entries: vec![
                                            ellie_core::definite::types::collective::CollectiveEntry {
                                                key: k,
                                                value: t,
                                                key_pos: defs::Cursor::default(),
                                                value_pos: defs::Cursor::default(),
                                            },
                                        ],
                                        pos: defs::Cursor::default(),
                                    },
                                )),
                                    None => None,
                                },
                                None => None,
                            }
                        } else if parent_generic.rtype == "vector" {
                            match generate_type_from_defining(
                                parent_generic.generics[0].value.clone(),
                            ) {
                                Some(t) => Some(Types::Vector(
                                    ellie_core::definite::types::vector::VectorType {
                                        collective: vec![
                                            ellie_core::definite::types::vector::VectorEntry {
                                                value: t,
                                                location: defs::Cursor::default(),
                                            },
                                        ],
                                        pos: defs::Cursor::default(),
                                    },
                                )),
                                None => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => unreachable!(),
                }
            }

            let resolved_type = generate_type_from_defining(as_keyword.rtype.clone());

            match resolved_type {
                Some(types) => match types {
                    Types::Integer(e) => DeepTypeResult::Integer(e),
                    Types::Float(e) => DeepTypeResult::Float(e),
                    Types::Bool(e) => DeepTypeResult::Bool(e),
                    Types::String(e) => DeepTypeResult::String(e),
                    Types::Char(e) => DeepTypeResult::Char(e),
                    Types::Collective(e) => DeepTypeResult::Collective(e),
                    Types::Cloak(e) => DeepTypeResult::Cloak(e),
                    Types::Array(e) => DeepTypeResult::Array(e),
                    Types::Vector(e) => DeepTypeResult::Vector(e),
                    Types::Dynamic => DeepTypeResult::Dynamic,
                    _ => unreachable!("Unexpected return: {:#?}", types),
                },
                None => {
                    let path = parser.find_page(page_id).unwrap().path.clone();
                    errors.push(error::error_list::ERROR_S50.clone().build_with_path(
                        vec![
                            error::ErrorBuildField {
                                key: "target".to_string(),
                                value: targeted.to_string(),
                            },
                            error::ErrorBuildField {
                                key: "type".to_string(),
                                value: as_keyword.rtype.to_string(),
                            },
                        ],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        path,
                        as_keyword.pos,
                    ));
                    DeepTypeResult::NotFound
                }
            }
        }
        Types::Bool(bool) => DeepTypeResult::Bool(bool),
        Types::Void => DeepTypeResult::Void,
        Types::Null => DeepTypeResult::Null,
        Types::Dynamic => DeepTypeResult::Dynamic,
        Types::Function(f) => DeepTypeResult::Function(f),
    }
}

pub fn resolve_deep_type(
    parser: &mut Parser,
    page_id: u64,
    rtype: Types,
    errors: &mut Vec<error::Error>,
) -> DeepTypeResult {
    iterate_deep_type(parser, page_id, rtype, errors)
}

#[derive(Debug, Clone)]
pub enum ProcessedDeepSearchItems {
    Class(ellie_core::definite::items::class::Class),
    Variable(ellie_core::definite::items::variable::Variable),
    Function(ellie_core::definite::items::function::Function),
    ImportReference(ellie_core::definite::items::import::Import),
    //SelfItem(ellie_core::definite::items::::SelfItem),
    //GenericItem(ellie_core::definite::items::generic::Generic),
    //FunctionParameter(ellie_core::definite::items::function::FunctionParameter),
    //ConstructorParameter(ellie_tokenizer::syntax::items::constructor_parameter::ConstructorParameter),
    //MixUp(Vec<(String, String)>),
    //BrokenPageGraph,
    None,
}

#[derive(Debug, Clone)]
pub struct ProcessedDeepSearchResult {
    pub found: bool,
    pub found_item: ProcessedDeepSearchItems,
    pub found_pos: Option<defs::Cursor>,
    pub found_page: ProcessedPage,
}

pub fn deep_search_hash(
    parser: &mut Parser,
    target_page: u64,
    target_hash: u64,
    searched: Vec<u64>,
    _level: u32,
) -> ProcessedDeepSearchResult {
    let mut level = _level;
    let mut found = false;
    let mut found_type = ProcessedDeepSearchItems::None;
    let mut found_pos = None;
    let mut found_page = ProcessedPage::default();
    let has_mixup = false;
    let mut inner_page = None;
    let mut searched: Vec<u64> = searched;
    //let mixup_hashes: Vec<(String, String)> = Vec::new();
    let mut self_dependencies = vec![];

    match parser.find_processed_page(target_page) {
        Some(page) => {
            self_dependencies.push(Dependency {
                hash: target_page,
                processed: true,
                ..Default::default()
            });
            self_dependencies.extend(
                page.dependencies
                    .iter()
                    .filter_map(|x| {
                        if x.processed && x.module.is_none() {
                            Some(x.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Dependency>>(),
            );
            inner_page = page.inner;
        }
        None => (),
    };

    if !searched.contains(&target_page) {
        let mut i = 0;
        loop {
            let dep = self_dependencies[i].clone();
            searched.push(target_page);
            match parser.find_processed_page(dep.hash) {
                Some(page) => {
                    let page = page.clone();
                    let internal_deps = page
                        .dependencies
                        .iter()
                        .filter_map(|x| if x.public { Some(x.clone()) } else { None })
                        .collect::<Vec<Dependency>>();
                    self_dependencies.extend(internal_deps);

                    for item in &page.items {
                        match item.clone() {
                            Collecting::Variable(e) => {
                                if e.hash == target_hash && (e.public || level == 0) {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Variable(e);
                                }
                            }
                            Collecting::Function(e) => {
                                if e.hash == target_hash && (e.public || level == 0) {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Function(e);
                                }
                            }
                            Collecting::Import(e) => {
                                if e.reference != ""
                                    && e.hash == target_hash
                                    && (e.public
                                        || level == 0
                                        || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::ImportReference(e);
                                }
                            }
                            Collecting::Class(e) => {
                                if e.hash == target_hash
                                    && (e.public
                                        || level == 0
                                        || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Class(e);
                                }
                            }
                            _ => (),
                        }
                    }
                }
                None => {
                    panic!("Broken Page structure; Failed to find page {}", dep.hash);
                }
            }
            level += 1;
            level += 1;
            if i == self_dependencies.len() - 1 {
                break;
            }
            i += 1;
        }
    }

    if has_mixup {
        unreachable!();
        /*
        ProcessedDeepSearchResult {
            found: true,
            found_pos,
            found_item: DeepSearchItems::MixUp(mixup_hashes),
            found_page,
        }
        */
    } else if found {
        ProcessedDeepSearchResult {
            found: true,
            found_pos,
            found_item: found_type,
            found_page,
        }
    } else {
        ProcessedDeepSearchResult {
            found: false,
            found_pos,
            found_item: ProcessedDeepSearchItems::None,
            found_page,
        }
    }
}

pub fn deep_search(
    parser: &mut Parser,
    target_page: u64,
    name: String,
    ignore_hash: Option<u64>,
    searched: Vec<u64>,
    _level: u32,
) -> ProcessedDeepSearchResult {
    let mut level = _level;
    let mut found = false;
    let mut found_type = ProcessedDeepSearchItems::None;
    let mut found_pos = None;
    let mut found_page = ProcessedPage::default();
    let has_mixup = false;
    let mut inner_page = None;
    let mut searched: Vec<u64> = searched;
    let mut self_dependencies = vec![Dependency {
        hash: target_page,
        ..Default::default()
    }];

    match parser.find_processed_page(target_page).cloned() {
        Some(page) => {
            self_dependencies.push(Dependency {
                hash: target_page,
                processed: true,
                ..Default::default()
            });
            self_dependencies.extend(
                page.dependencies
                    .iter()
                    .filter_map(|x| {
                        let dependency = parser.find_processed_page(x.hash);
                        if (x.processed || dependency.is_some()) && x.module.is_none() {
                            Some(x.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Dependency>>(),
            );
            inner_page = page.inner;
        }
        None => (),
    };

    if !searched.contains(&target_page) {
        let mut i = 0;
        loop {
            let dep = self_dependencies[i].clone();
            searched.push(target_page);
            match parser.find_processed_page(dep.hash) {
                Some(page) => {
                    let page = page.clone();
                    let internal_deps = page
                        .dependencies
                        .iter()
                        .filter_map(|x| if x.public { Some(x.clone()) } else { None })
                        .collect::<Vec<Dependency>>();
                    self_dependencies.extend(internal_deps);

                    for item in page.items.iter() {
                        match item.clone() {
                            Collecting::Variable(e) => {
                                if e.name == name
                                    && (e.public || level == 0)
                                    && (ignore_hash.is_none()
                                        || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Variable(e);
                                }
                            }
                            Collecting::Function(e) => {
                                if e.name == name
                                    && (e.public || level == 0)
                                    && (ignore_hash.is_none()
                                        || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Function(e);
                                }
                            }
                            Collecting::Import(e) => {
                                if e.reference != ""
                                    && e.reference == name
                                    && (e.public
                                        || level == 0
                                        || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                    && (ignore_hash.is_none()
                                        || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::ImportReference(e);
                                }
                            }
                            Collecting::Class(e) => {
                                if e.name == name
                                    && (e.public
                                        || level == 0
                                        || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                    && (ignore_hash.is_none()
                                        || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = page.clone();
                                    found_type = ProcessedDeepSearchItems::Class(e);
                                }
                            }
                            _ => (),
                        }
                    }
                }
                None => {
                    panic!("Broken Page structure; Failed to find page {}", dep.hash);
                }
            }

            level += 1;
            if i == self_dependencies.len() - 1 {
                break;
            }
            i += 1;
        }
    }

    if has_mixup {
        unreachable!();
    } else if found {
        ProcessedDeepSearchResult {
            found: true,
            found_pos,
            found_item: found_type,
            found_page,
        }
    } else {
        ProcessedDeepSearchResult {
            found: false,
            found_pos,
            found_item: ProcessedDeepSearchItems::None,
            found_page,
        }
    }
}

pub fn find_type(
    rtype: String,
    target_page: u64,
    parser: &mut Parser,
) -> Option<definers::GenericType> {
    let result = deep_search(parser, target_page, rtype.clone(), None, vec![], 0);
    if result.found {
        match result.found_item {
            ProcessedDeepSearchItems::Class(e) => Some(definers::GenericType {
                rtype,
                pos: e.pos,
                hash: e.hash,
            }),
            ProcessedDeepSearchItems::Variable(_) => {
                panic!("Unexpected internal crash, parser should have prevented this, {:?}", result);
            }
            ProcessedDeepSearchItems::Function(_) => {
                panic!("Unexpected internal crash, parser should have prevented this, {:?}", result);
            }
            ProcessedDeepSearchItems::ImportReference(_) => {
                panic!("Unexpected internal crash, parser should have prevented this, {:?}", result);
            }
            ProcessedDeepSearchItems::None => None,
        }
    } else {
        None
    }
}

pub fn resolve_type(
    target_type: Types,
    target_page: u64,
    parser: &mut Parser,
    errors: &mut Vec<error::Error>,
) -> definers::DefinerCollecting {
    let deep_type = crate::deep_search_extensions::resolve_deep_type(
        parser,
        target_page,
        target_type.clone(),
        errors,
    );

    match deep_type {
        DeepTypeResult::Integer(_) => {
            let int_type = find_type("int".to_string(), target_page, parser);
            match int_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find int type");
                }
            }
        }
        DeepTypeResult::Float(_) => {
            let float_type = find_type("float".to_string(), target_page, parser);
            match float_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find string type");
                }
            }
        }
        DeepTypeResult::Bool(_) => {
            let bool_type = find_type("bool".to_string(), target_page, parser);
            match bool_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find string type");
                }
            }
        }
        DeepTypeResult::String(_) => {
            let string_type = find_type("string".to_string(), target_page, parser);
            match string_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find string type");
                }
            }
        }
        DeepTypeResult::Char(_) => {
            let char_type = find_type("char".to_string(), target_page, parser);
            match char_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find char type");
                }
            }
        }
        DeepTypeResult::Collective(_) => todo!(),
        DeepTypeResult::Operator(_) => todo!(),
        DeepTypeResult::Cloak(_) => todo!(),
        DeepTypeResult::Array(array_type) => {
            #[derive(PartialEq, EnumAsInner, Clone)]
            enum GenericExists {
                Generic(definers::DefinerCollecting),
                Null,
            }
            let mut child_generic = GenericExists::Null;
            for entry in array_type.collective {
                let resolved = resolve_type(entry.value, target_page, parser, errors);
                if child_generic != GenericExists::Null
                    && resolved != *child_generic.as_generic().unwrap()
                {
                    let dyn_type = find_type("dyn".to_string(), target_page, parser);
                    match dyn_type {
                        Some(dynamic_type) => {
                            child_generic = GenericExists::Generic(
                                definers::DefinerCollecting::Generic(dynamic_type),
                            );
                        }
                        None => {
                            panic!("Unhandled behaviour, failed to find string type");
                        }
                    }

                    break;
                }
                child_generic = GenericExists::Generic(resolved);
            }

            let array_type = find_type("array".to_string(), target_page, parser);
            match array_type {
                Some(array_generic) => {
                    let val = child_generic
                        .as_generic()
                        .unwrap_or(&definers::DefinerCollecting::Dynamic)
                        .clone();
                    definers::DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                        rtype: "array".to_string(),
                        generics: vec![definers::GenericParameter {
                            value: val,
                            pos: ellie_core::defs::Cursor::default(),
                        }],
                        hash: array_generic.hash,
                        parent_pos: ellie_core::defs::Cursor::default(),
                    })
                }
                None => panic!("Unhandled behaviour"),
            }
        }
        DeepTypeResult::Vector(_) => todo!(),
        DeepTypeResult::ClassCall(class_call) => match (*class_call.target).clone() {
            Types::Cloak(cloak) => {
                if cloak.collective.len() == 1 {
                    unimplemented!()
                } else {
                    unreachable!()
                }
            }
            Types::VariableType(variable) => {
                let deep_search_result =
                    parser.deep_search(target_page, variable.value.clone(), None, Vec::new(), 0);
                let targeted_class =
                    find_type(variable.value.clone(), target_page, parser).unwrap();

                if deep_search_result.found {
                    match deep_search_result.found_item {
                        crate::parser::DeepSearchItems::Class(e) => {
                            if e.generic_definings.len() != class_call.generic_parameters.len() {
                                unreachable!()
                            } else if let Some(_) = e.body.iter().find_map(|x| match x {
                                ellie_tokenizer::processors::items::Processors::Constructor(e) => {
                                    Some(e)
                                }
                                _ => None,
                            }) {
                                if class_call.generic_parameters.is_empty() {
                                    definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: variable.value.clone(),
                                        hash: targeted_class.hash,
                                        pos: defs::Cursor::default(),
                                    })
                                } else {
                                    definers::DefinerCollecting::ParentGeneric(
                                        definers::ParentGenericType {
                                            rtype: variable.value.clone(),
                                            generics: class_call.generic_parameters.iter().map(|x| {
                                                ellie_core::definite::definers::GenericParameter {
                                                    value: x.value.clone(),
                                                    pos: defs::Cursor::default(),
                                                }
                                            }).collect::<Vec<_>>(),
                                            parent_pos: defs::Cursor::default(),
                                            hash: targeted_class.hash,
                                        },
                                    )
                                }
                            } else {
                                if class_call.generic_parameters.is_empty() {
                                    definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: variable.value.clone(),
                                        hash: targeted_class.hash,
                                        pos: defs::Cursor::default(),
                                    })
                                } else {
                                    definers::DefinerCollecting::ParentGeneric(
                                        definers::ParentGenericType {
                                            rtype: variable.value.clone(),
                                            generics: class_call.generic_parameters.iter().map(|x| {
                                                ellie_core::definite::definers::GenericParameter {
                                                    value: x.value.clone(),
                                                    pos: defs::Cursor::default(),
                                                }
                                            }).collect::<Vec<_>>(),
                                            parent_pos: defs::Cursor::default(),
                                            hash: targeted_class.hash,
                                        },
                                    )
                                }
                            }
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
                            unreachable!()
                        }
                    }
                } else {
                    unreachable!()
                }
            }

            _ => unreachable!(),
        },
        DeepTypeResult::FunctionCall(e) => {
            let dyn_type = find_type("function".to_string(), target_page, parser);
            match dyn_type {
                Some(dynamic_type) => definers::DefinerCollecting::Generic(dynamic_type),
                None => panic!("Unhandled behaviour"),
            }
        }
        DeepTypeResult::Void => {
            let void_type = find_type("void".to_string(), target_page, parser);
            match void_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find void type");
                }
            }
        }
        DeepTypeResult::Null => {
            let null_type = find_type("null".to_string(), target_page, parser);
            match null_type {
                Some(e) => definers::DefinerCollecting::Generic(e),
                None => {
                    panic!("Unhandled behaviour, failed to find null type");
                }
            }
        }
        DeepTypeResult::NotFound => unreachable!(),
        DeepTypeResult::BraceReference(e) => {
            let nullable_type = find_type("nullAble".to_string(), target_page, parser);
            match nullable_type {
                Some(nullable_generic) => {
                    let nullable_child_generic = match *e.reference.clone() {
                        Types::Array(_) => {
                            let array_type =
                                resolve_type(*e.reference, target_page, parser, errors);
                            array_type.as_parent_generic().unwrap().generics[0]
                                .value
                                .clone()
                        }
                        _ => {
                            unimplemented!("Custom index queries are not yet supported",)
                        }
                    };
                    definers::DefinerCollecting::ParentGeneric(definers::ParentGenericType {
                        rtype: "nullAble".to_string(),
                        generics: vec![definers::GenericParameter {
                            value: nullable_child_generic,
                            pos: ellie_core::defs::Cursor::default(),
                        }],
                        hash: nullable_generic.hash,
                        parent_pos: ellie_core::defs::Cursor::default(),
                    })
                }
                None => panic!("Unhandled behaviour"),
            }
        }
        DeepTypeResult::Dynamic => {
            let dyn_type = find_type("dyn".to_string(), target_page, parser);
            match dyn_type {
                Some(dynamic_type) => definers::DefinerCollecting::Generic(dynamic_type),
                None => panic!("Unhandled behaviour"),
            }
        }
        DeepTypeResult::Function(e) => {
            definers::DefinerCollecting::Function(definers::FunctionType {
                params: e
                    .parameters
                    .iter()
                    .map(|param| {
                        param
                            .rtype
                            .clone()
                            .unwrap_or(definers::DefinerCollecting::Dynamic)
                    })
                    .collect::<Vec<_>>(),
                returning: Box::new(e.return_type),
            })
        }
    }
}