use crate::parser::{DeepSearchItems, FoundPage, Parser};
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use ellie_core::{
    definite::{
        definers::{self, DefinerCollecting},
        items::Collecting,
        types::{class_instance::AttributeType, Types},
        Converter,
    },
    defs, error,
};
use ellie_tokenizer::tokenizer::Dependency;
use enum_as_inner::EnumAsInner;

/*
    This folder contains parser extensions for deep search.
*/

pub fn generate_type_from_defining(
    rtype: ellie_core::definite::definers::DefinerCollecting,
    page_id: usize,
    parser: &mut crate::parser::Parser,
) -> Option<Types> {
    match rtype {
        definers::DefinerCollecting::Generic(generic) => {
            if generic.rtype == "int" {
                Some(Types::Integer(
                    ellie_core::definite::types::integer::IntegerType {
                        value: 0,
                        pos: defs::Cursor::default(),
                    },
                ))
            } else if generic.rtype == "float" {
                Some(Types::Decimal(
                    ellie_core::definite::types::decimal::DecimalType {
                        value: ellie_core::definite::types::decimal::DecimalTypeEnum::Float(0.0),
                        pos: defs::Cursor::default(),
                        is_double: false,
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
                let hash_deep_search = crate::deep_search_extensions::deep_search_hash(
                    parser,
                    page_id,
                    generic.hash,
                    vec![],
                    0,
                );
                if hash_deep_search.found {
                    match hash_deep_search.found_item {
                        crate::deep_search_extensions::ProcessedDeepSearchItems::Class(
                            matched_class,
                        ) => {
                            if matched_class.generic_definings.is_empty() {
                                Some(Types::ClassCall(
                                    ellie_core::definite::types::class_call::ClassCall {
                                        target: Box::new(Types::VariableType(
                                            ellie_core::definite::types::variable::VariableType {
                                                value: matched_class.name.clone(),
                                                reference: matched_class.hash,
                                                pos: defs::Cursor::default(),
                                            },
                                        )),
                                        resolved_generics: vec![],
                                        generic_parameters: vec![],
                                        keyword_pos: defs::Cursor::default(),
                                        pos: defs::Cursor::default(),
                                        target_pos: defs::Cursor::default(),
                                        params: vec![],
                                    },
                                ))
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
                            collective: vec![ellie_core::definite::types::array::ArrayEntry {
                                value: t,
                                location: defs::Cursor::default(),
                            }],
                            pos: defs::Cursor::default(),
                        },
                    )),
                    None => None,
                }
            } else if parent_generic.rtype == "cloak" {
                let mut cloak_entries = vec![];
                let mut unresolved_element_available = false;
                for generic in parent_generic.generics {
                    match generate_type_from_defining(generic.value, page_id, parser) {
                        Some(t) => {
                            cloak_entries.push(ellie_core::definite::types::cloak::CloakEntry {
                                value: t,
                                location: defs::Cursor::default(),
                            })
                        }
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
                match generate_type_from_defining(
                    parent_generic.generics[0].value.clone(),
                    page_id,
                    parser,
                ) {
                    Some(t) => Some(Types::Collective(
                        ellie_core::definite::types::collective::CollectiveType {
                            entries: vec![
                                ellie_core::definite::types::collective::CollectiveEntry {
                                    key: "?".to_string(),
                                    value: t,
                                    key_pos: defs::Cursor::default(),
                                    value_pos: defs::Cursor::default(),
                                },
                            ],
                            pos: defs::Cursor::default(),
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
                                                            pos: defs::Cursor::default(),
                                                        },
                                                    )),
                                                    resolved_generics: parent_generic.generics.iter().map(|generic| {
                                                        generic.value.clone()
                                                    }).collect::<Vec<_>>(),
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
        definers::DefinerCollecting::Function(function) => Some(Types::Function(
            ellie_core::definite::types::function::Function {
                native: false,
                parameters: function
                    .params
                    .iter()
                    .map(
                        |parameter| ellie_core::definite::types::function::FunctionParameter {
                            name: "anonymous".to_string(),
                            rtype: Some(parameter.clone()),
                            rtype_pos: defs::Cursor::default(),
                            name_pos: defs::Cursor::default(),
                        },
                    )
                    .collect::<Vec<_>>(),
                return_type: *function.returning,
                has_parameter_definings: false,
                arrow_function: false,
                inside_code: vec![],
                return_pos: defs::Cursor::default(),
            },
        )),
        definers::DefinerCollecting::EnumField(enum_data) => Some(Types::EnumData(
            ellie_core::definite::types::enum_data::EnumData {
                reference: Box::new(Types::VariableType(
                    ellie_core::definite::types::variable::VariableType {
                        value: enum_data.name.clone(),
                        reference: enum_data.hash,
                        pos: defs::Cursor::default(),
                    },
                )),
                field_name: enum_data.field_name,
                reference_pos: defs::Cursor::default(),
                brace_pos: defs::Cursor::default(),
                value: match enum_data.field_data {
                    definers::EnumFieldData::NoData => {
                        ellie_core::definite::types::enum_data::Pointer::NoData
                    }
                    definers::EnumFieldData::Data(e) => {
                        ellie_core::definite::types::enum_data::Pointer::Data(Box::new(
                            generate_type_from_defining(*e, page_id, parser).unwrap(),
                        ))
                    }
                },
                pos: defs::Cursor::default(),
            },
        )),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub enum DeepTypeResult {
    Integer(ellie_core::definite::types::integer::IntegerType),
    Byte(ellie_core::definite::types::byte::ByteType),
    Decimal(ellie_core::definite::types::decimal::DecimalType),
    Bool(ellie_core::definite::types::bool::BoolType),
    String(ellie_core::definite::types::string::StringType),
    Char(ellie_core::definite::types::ellie_char::CharType),
    EnumData(ellie_core::definite::types::enum_data::EnumData),
    Enum(ellie_core::definite::items::enum_type::EnumType),
    Collective(ellie_core::definite::types::collective::CollectiveType),
    Operator(ellie_core::definite::types::operator::OperatorType),
    Cloak(ellie_core::definite::types::cloak::CloakType),
    Array(ellie_core::definite::types::array::ArrayType),
    ClassCall(ellie_core::definite::types::class_call::ClassCall),
    Function(ellie_core::definite::types::function::Function),
    FunctionCall(ellie_core::definite::types::function_call::FunctionCall),
    BraceReference(ellie_core::definite::types::brace_reference::BraceReferenceType),
    ClassInstance(ellie_core::definite::types::class_instance::ClassInstance),
    SelfItem(ellie_core::definite::items::self_item::SelfItem),
    Void,
    Null,
    Dynamic,
    NotFound,
}

fn iterate_deep_type(
    parser: &mut Parser,
    page_id: usize,
    rtype: Types,
    errors: &mut Vec<error::Error>,
) -> DeepTypeResult {
    match rtype.clone() {
        Types::Integer(integer) => DeepTypeResult::Integer(integer),
        Types::Decimal(float) => DeepTypeResult::Decimal(float),
        Types::String(string) => DeepTypeResult::String(string),
        Types::Char(char) => DeepTypeResult::Char(char),
        Types::Collective(collective) => DeepTypeResult::Collective(collective),
        Types::Reference(reference) => {
            #[derive(Debug, Clone)]
            struct Attribute {
                _rtype: AttributeType,
                name: String,
                value: definers::DefinerCollecting,
                page: usize,
            }

            fn resolve_chain(
                reference_type: definers::DefinerCollecting,
                reference_pos: defs::Cursor,
                page_id: usize,
                parser: &mut crate::parser::Parser,
            ) -> Result<Vec<Attribute>, Vec<error::Error>> {
                let mut errors: Vec<error::Error> = Vec::new();
                match reference_type.clone() {
                    ellie_core::definite::definers::DefinerCollecting::Generic(generic) => {
                        let hash_deep_search =
                            deep_search_hash(parser, page_id, generic.hash, vec![], 0);

                        if hash_deep_search.found {
                            match hash_deep_search.found_item {
                                ProcessedDeepSearchItems::Class(class_page) => {
                                    match parser
                                        .find_processed_page(class_page.inner_page_id)
                                        .cloned()
                                    {
                                        Some(class_inner_page) => {
                                            let attributes = class_inner_page.items.iter().filter_map(|item| {
                                                        match item.clone() {
                                                            Collecting::Variable(e) => {
                                                                let resolved_type = if e.has_type { e.rtype } else { match resolve_type(e.value, class_inner_page.hash, parser, &mut errors, Some(reference_pos)) {
                                                                    Some(x) => x,
                                                                    None => {
                                                                        return None;
                                                                    },
                                                                } };
                                                                Some(Attribute {
                                                                    _rtype: AttributeType::Property,
                                                                    name: e.name.clone(),
                                                                    value: resolved_type,
                                                                    page: class_inner_page.hash,
                                                                })
                                                            },
                                                            Collecting::Function(e) => {
                                                                Some(Attribute {
                                                                    _rtype: AttributeType::Method,
                                                                    name: e.name.clone(),
                                                                    value: definers::DefinerCollecting::Function(
                                                                        ellie_core::definite::definers::FunctionType {
                                                                            params: e.parameters.iter().map(|param| {
                                                                                param.rtype.clone()
                                                                            }).collect::<Vec<_>>(),
                                                                            returning: Box::new(e.return_type),
                                                                        }
                                                                    ),
                                                                    page: class_inner_page.hash,
                                                                })
                                                            },
                                                            Collecting::NativeFunction(e) => {
                                                                Some(Attribute {
                                                                    _rtype: AttributeType::Method,
                                                                    name: e.name.clone(),
                                                                    value: definers::DefinerCollecting::Function(
                                                                        ellie_core::definite::definers::FunctionType {
                                                                            params: e.parameters.iter().map(|param| {
                                                                                param.rtype.clone()
                                                                            }).collect::<Vec<_>>(),
                                                                            returning: Box::new(e.return_type),
                                                                        }
                                                                    ),
                                                                    page: class_inner_page.hash,
                                                                })
                                                            }
                                                            Collecting::Getter(e) => {
                                                                Some(Attribute {
                                                                    _rtype: AttributeType::Method,
                                                                    name: e.name.clone(),
                                                                    value: e.return_type,
                                                                    page: class_inner_page.hash,
                                                                })
                                                            }
                                                            Collecting::Setter(e) => {
                                                                Some(Attribute {
                                                                    _rtype: AttributeType::Method,
                                                                    name: e.name.clone(),
                                                                    value: e.rtype,
                                                                    page: class_inner_page.hash,
                                                                })
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
                                ProcessedDeepSearchItems::Enum(enum_data) => {
                                    Ok(
                                        enum_data.items.iter().map(|item| {
                                            Attribute {
                                                _rtype: match &item.value {
                                                    ellie_core::definite::items::enum_type::EnumValue::NoValue => AttributeType::EnumItemNoData,
                                                    ellie_core::definite::items::enum_type::EnumValue::Value(_) => AttributeType::EnumItemData
                                                },
                                                name: item.identifier.clone(),
                                                value: definers::DefinerCollecting::EnumField(
                                                    definers::EnumField {
                                                        field_name: item.identifier.clone(),
                                                        field_data:  match &item.value {
                                                            ellie_core::definite::items::enum_type::EnumValue::NoValue => definers::EnumFieldData::NoData,
                                                            ellie_core::definite::items::enum_type::EnumValue::Value(e) => definers::EnumFieldData::Data(Box::new(e.clone())),
                                                        },
                                                        name: enum_data.name.clone(),
                                                        hash: enum_data.hash,
                                                    }
                                                ),
                                                page: page_id,
                                            }
                                        }).collect()
                                    )
                                }
                                _ => unreachable!(),
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
                    ellie_core::definite::definers::DefinerCollecting::ParentGeneric(generic) => {
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
                                                        match item.clone() {
                                                            Collecting::Variable(e) => {
                                                                let resolved_type = if e.has_type { e.rtype } else { match resolve_type(e.value, class_inner_page.hash, parser, &mut errors, Some(reference_pos)) {
                                                                    Some(x) => x,
                                                                    None => {
                                                                        return None;
                                                                    },
                                                                } };
                                                                Some(Attribute {
                                                                    _rtype: AttributeType::Property,
                                                                    name: e.name.clone(),
                                                                    value: resolved_type,
                                                                    page: class_inner_page.hash,
                                                                })
                                                            },
                                                            Collecting::Function(e) => {
                                                                Some(Attribute {
                                                                    _rtype: AttributeType::Method,
                                                                    name: e.name.clone(),
                                                                    value: definers::DefinerCollecting::Function(
                                                                        ellie_core::definite::definers::FunctionType {
                                                                            params: e.parameters.iter().map(|param| {
                                                                                param.rtype.clone()
                                                                            }).collect::<Vec<_>>(),
                                                                            returning: Box::new(e.return_type),
                                                                        }
                                                                    ),
                                                                    page: class_inner_page.hash,
                                                                })
                                                            },
                                                            Collecting::NativeFunction(e) => {
                                                                Some(Attribute {
                                                                    _rtype: AttributeType::Method,
                                                                    name: e.name.clone(),
                                                                    value: definers::DefinerCollecting::Function(
                                                                        ellie_core::definite::definers::FunctionType {
                                                                            params: e.parameters.iter().map(|param| {
                                                                                param.rtype.clone()
                                                                            }).collect::<Vec<_>>(),
                                                                            returning: Box::new(e.return_type),
                                                                        }
                                                                    ),
                                                                    page: class_inner_page.hash,
                                                                })
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
                                _ => unreachable!(),
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
                    ellie_core::definite::definers::DefinerCollecting::Function(_) => {
                        let result = parser.deep_search(
                            page_id,
                            "function".to_owned(),
                            None,
                            vec![],
                            0,
                            Some(reference_pos),
                        );
                        if result.found {
                            let rtype = match result.found_item {
                                DeepSearchItems::Class(e) => definers::GenericType {
                                    rtype: e.name,
                                    pos: e.pos,
                                    hash: e.hash,
                                },
                                _ => unreachable!(),
                            };
                            let pos = rtype.pos;
                            match resolve_chain(
                                definers::DefinerCollecting::Generic(rtype),
                                pos,
                                result.found_page.hash,
                                parser,
                            ) {
                                Ok(e) => Ok(e),
                                Err(e) => {
                                    errors.extend(e);
                                    Err(errors)
                                }
                            }
                        } else {
                            todo!()
                        }
                    }
                    ellie_core::definite::definers::DefinerCollecting::EnumField(_) => {
                        let rtype = find_type("enum".to_owned(), page_id, parser);
                        match resolve_chain(
                            definers::DefinerCollecting::Generic(rtype.unwrap()),
                            defs::Cursor::default(),
                            page_id,
                            parser,
                        ) {
                            Ok(e) => Ok(e),
                            Err(e) => {
                                errors.extend(e);
                                Err(errors)
                            }
                        }
                    }
                    ellie_core::definite::definers::DefinerCollecting::ClassInstance(
                        class_instance,
                    ) => {
                        let mut attributes = Vec::new();

                        for attribute in &class_instance.attributes {
                            let page = parser.find_processed_page(attribute.page).unwrap();
                            let item = page.find_item_by_hash(attribute.hash).unwrap();
                            let page_id = page.hash;

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
                                        _rtype: attribute._rtype.clone(),
                                        name: attribute.name.clone(),
                                        value,
                                        page: page_id,
                                    });
                                }
                                Collecting::Function(e) => {
                                    attributes.push(Attribute {
                                        _rtype: attribute._rtype.clone(),
                                        name: attribute.name.clone(),
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
                                        page: page.hash,
                                    });
                                }
                                Collecting::Getter(e) => {
                                    attributes.push(Attribute {
                                        _rtype: attribute._rtype.clone(),
                                        name: attribute.name.clone(),
                                        value: e.return_type,
                                        page: page.hash,
                                    });
                                }
                                Collecting::Setter(e) => {
                                    attributes.push(Attribute {
                                        _rtype: attribute._rtype.clone(),
                                        name: attribute.name.clone(),
                                        value: e.rtype,
                                        page: page.hash,
                                    });
                                }
                                Collecting::NativeFunction(e) => {
                                    attributes.push(Attribute {
                                        _rtype: attribute._rtype.clone(),
                                        name: attribute.name.clone(),
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
                                        page: page.hash,
                                    });
                                }
                                _ => (),
                            }
                        }
                        Ok(attributes)
                    }
                    _ => match resolve_absolute_definer(parser, page_id, reference_type) {
                        Ok(e) => resolve_chain(e, reference_pos, page_id, parser),
                        Err(e) => {
                            errors.extend(e);
                            Err(errors)
                        }
                    },
                }
            }

            let reference_type = match resolve_type(
                *reference.reference.clone(),
                page_id,
                parser,
                errors,
                Some(reference.reference_pos),
            ) {
                Some(e) => e,
                None => {
                    return DeepTypeResult::NotFound;
                }
            };
            #[derive(Debug, EnumAsInner)]
            enum LastEntry {
                Type(Types),
                Null,
            }
            let mut resolved_types = LastEntry::Null;
            let mut last_chain_attributes = (
                reference_type.clone(),
                match resolve_chain(reference_type, reference.pos, page_id, parser) {
                    Ok(e) => e,
                    Err(e) => {
                        errors.extend(e);
                        return DeepTypeResult::NotFound;
                    }
                },
            );
            for chain in &reference.chain {
                let attribute = last_chain_attributes
                    .1
                    .iter()
                    .find(|a| a.name == chain.value);
                match attribute {
                    Some(a) => {
                        resolved_types = LastEntry::Type(
                            generate_type_from_defining(a.value.clone(), page_id, parser).unwrap(),
                        );
                        last_chain_attributes = (
                            a.value.clone(),
                            match resolve_chain(a.value.clone(), chain.pos, a.page, parser) {
                                Ok(e) => e,
                                Err(e) => {
                                    errors.extend(e);
                                    return DeepTypeResult::NotFound;
                                }
                            },
                        );
                    }
                    None => {
                        errors.push(error::error_list::ERROR_S42.clone().build_with_path(
                            vec![
                                error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: chain.value.clone(),
                                },
                                error::ErrorBuildField {
                                    key: "token1".to_owned(),
                                    value: last_chain_attributes.0.to_string(),
                                },
                            ],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            parser.find_page(page_id).unwrap().path.clone(),
                            chain.pos,
                        ));
                    }
                }
            }

            if errors.is_empty() {
                resolve_deep_type(
                    parser,
                    page_id,
                    resolved_types
                        .as_type()
                        .unwrap_or_else(|| panic!("resolve_deep_type: {:?}", resolved_types))
                        .clone(),
                    errors,
                )
            } else {
                DeepTypeResult::NotFound
            }
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
                            DeepTypeResult::Byte(e) => Types::Byte(e),
                            DeepTypeResult::Decimal(e) => Types::Decimal(e),
                            DeepTypeResult::Bool(e) => Types::Bool(e),
                            DeepTypeResult::String(e) => Types::String(e),
                            DeepTypeResult::Char(e) => Types::Char(e),
                            DeepTypeResult::Collective(e) => Types::Collective(e),
                            DeepTypeResult::Operator(e) => Types::Operator(e),
                            DeepTypeResult::Cloak(e) => Types::Cloak(e),
                            DeepTypeResult::Array(e) => Types::Array(e),
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
                            DeepTypeResult::EnumData(_) => todo!(),
                            DeepTypeResult::Enum(_) => todo!(),
                            DeepTypeResult::ClassInstance(_) => todo!(),
                            DeepTypeResult::SelfItem(_) => todo!(),
                        }),
                        reference_pos: e.reference_pos,
                        brace_pos: e.brace_pos,
                        value: Box::new(match resolved_index {
                            DeepTypeResult::Integer(e) => Types::Integer(e),
                            DeepTypeResult::Byte(e) => Types::Byte(e),
                            DeepTypeResult::Decimal(e) => Types::Decimal(e),
                            DeepTypeResult::Bool(e) => Types::Bool(e),
                            DeepTypeResult::String(e) => Types::String(e),
                            DeepTypeResult::Char(e) => Types::Char(e),
                            DeepTypeResult::Collective(e) => Types::Collective(e),
                            DeepTypeResult::Operator(e) => Types::Operator(e),
                            DeepTypeResult::Cloak(e) => Types::Cloak(e),
                            DeepTypeResult::Array(e) => Types::Array(e),
                            DeepTypeResult::ClassCall(e) => Types::ClassCall(e),
                            DeepTypeResult::FunctionCall(e) => Types::FunctionCall(e),
                            DeepTypeResult::BraceReference(e) => Types::BraceReference(e),
                            DeepTypeResult::Void => unreachable!(),
                            DeepTypeResult::Null => unreachable!(),
                            DeepTypeResult::NotFound => unreachable!(),
                            DeepTypeResult::Dynamic => Types::Dynamic,
                            DeepTypeResult::Function(e) => Types::Function(e),
                            DeepTypeResult::EnumData(_) => todo!(),
                            DeepTypeResult::Enum(_) => todo!(),
                            DeepTypeResult::ClassInstance(_) => todo!(),
                            DeepTypeResult::SelfItem(_) => todo!(),
                        }),
                        pos: e.pos,
                    },
                )
            }
        }
        Types::Operator(e) => {
            let first = match resolve_deep_type(parser, page_id, *e.first, errors) {
                DeepTypeResult::Integer(e) => Types::Integer(e),
                DeepTypeResult::Byte(e) => Types::Byte(e),
                DeepTypeResult::Decimal(e) => Types::Decimal(e),
                DeepTypeResult::Bool(e) => Types::Bool(e),
                DeepTypeResult::String(e) => Types::String(e),
                DeepTypeResult::Char(e) => Types::Char(e),
                DeepTypeResult::Collective(e) => Types::Collective(e),
                DeepTypeResult::Operator(e) => Types::Operator(e),
                DeepTypeResult::Cloak(e) => Types::Cloak(e),
                DeepTypeResult::Array(e) => Types::Array(e),
                DeepTypeResult::ClassCall(e) => Types::ClassCall(e),
                DeepTypeResult::Function(e) => Types::Function(e),
                DeepTypeResult::FunctionCall(e) => Types::FunctionCall(e),
                DeepTypeResult::BraceReference(e) => Types::BraceReference(e),
                DeepTypeResult::Void => Types::Void,
                DeepTypeResult::Null => Types::Null,
                DeepTypeResult::Dynamic => Types::Dynamic,
                DeepTypeResult::NotFound => unreachable!(),
                DeepTypeResult::EnumData(_) => todo!(),
                DeepTypeResult::Enum(_) => todo!(),
                DeepTypeResult::ClassInstance(_) => todo!(),
                DeepTypeResult::SelfItem(_) => todo!(),
            };

            let second = match resolve_deep_type(parser, page_id, *e.second, errors) {
                DeepTypeResult::Integer(e) => Types::Integer(e),
                DeepTypeResult::Byte(e) => Types::Byte(e),
                DeepTypeResult::Decimal(e) => Types::Decimal(e),
                DeepTypeResult::Bool(e) => Types::Bool(e),
                DeepTypeResult::String(e) => Types::String(e),
                DeepTypeResult::Char(e) => Types::Char(e),
                DeepTypeResult::Collective(e) => Types::Collective(e),
                DeepTypeResult::Operator(e) => Types::Operator(e),
                DeepTypeResult::Cloak(e) => Types::Cloak(e),
                DeepTypeResult::Array(e) => Types::Array(e),
                DeepTypeResult::ClassCall(e) => Types::ClassCall(e),
                DeepTypeResult::Function(e) => Types::Function(e),
                DeepTypeResult::FunctionCall(e) => Types::FunctionCall(e),
                DeepTypeResult::BraceReference(e) => Types::BraceReference(e),
                DeepTypeResult::Void => Types::Void,
                DeepTypeResult::Null => Types::Null,
                DeepTypeResult::Dynamic => Types::Dynamic,
                DeepTypeResult::NotFound => unreachable!(),
                DeepTypeResult::EnumData(e) => Types::EnumData(e),
                DeepTypeResult::Enum(_) => todo!(),
                DeepTypeResult::ClassInstance(_) => todo!(),
                DeepTypeResult::SelfItem(_) => todo!(),
            };

            DeepTypeResult::Operator(ellie_core::definite::types::operator::OperatorType {
                cloaked: e.cloaked,
                first_pos: e.first_pos,
                operator: e.operator,
                second_pos: e.second_pos,
                pos: e.pos,
                second: Box::new(second),
                first: Box::new(first),
            })
        }
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
        Types::SetterCall(_) => unreachable!(),
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
                    DeepTypeResult::Byte(byte_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Byte(byte_type),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Decimal(decimal_type) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Decimal(decimal_type),
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
                    DeepTypeResult::Dynamic => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Dynamic,
                            location: i.location,
                        });
                    }
                    DeepTypeResult::Function(e) => {
                        collective.push(ellie_core::definite::types::array::ArrayEntry {
                            value: Types::Function(e),
                            location: i.location,
                        });
                    }
                    DeepTypeResult::EnumData(_) => todo!(),
                    DeepTypeResult::Enum(_) => todo!(),
                    DeepTypeResult::ClassInstance(_) => todo!(),
                    DeepTypeResult::SelfItem(_) => todo!(),
                }
            }
            DeepTypeResult::Array(ellie_core::definite::types::array::ArrayType {
                collective,
                pos: array.pos,
            })
        }
        Types::ClassCall(class_call) => DeepTypeResult::ClassCall(class_call),
        Types::FunctionCall(e) => DeepTypeResult::FunctionCall(e),
        Types::NullResolver(null_resolver) => {
            let from_type = match resolve_type(
                *null_resolver.target.clone(),
                page_id,
                parser,
                errors,
                Some(null_resolver.target_pos),
            ) {
                Some(e) => e,
                None => return DeepTypeResult::NotFound,
            };
            match from_type {
                definers::DefinerCollecting::ParentGeneric(e) => {
                    if e.rtype == "nullAble" {
                        let resolved_type = generate_type_from_defining(
                            e.generics[0].value.clone(),
                            page_id,
                            parser,
                        );
                        match resolved_type {
                            Some(types) => match types {
                                Types::Integer(e) => DeepTypeResult::Integer(e),
                                Types::Decimal(e) => DeepTypeResult::Decimal(e),
                                Types::Bool(e) => DeepTypeResult::Bool(e),
                                Types::String(e) => DeepTypeResult::String(e),
                                Types::Char(e) => DeepTypeResult::Char(e),
                                Types::Collective(e) => DeepTypeResult::Collective(e),
                                Types::Cloak(e) => DeepTypeResult::Cloak(e),
                                Types::Array(e) => DeepTypeResult::Array(e),
                                Types::Dynamic => DeepTypeResult::Dynamic,
                                Types::ClassCall(e) => DeepTypeResult::ClassCall(e),
                                Types::Function(e) => DeepTypeResult::Function(e),
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
                        //See lib/object.ei
                        let class_class =
                            deep_search(parser, page_id, "object".to_owned(), None, vec![], 0);
                        if class_class.found {
                            if let ProcessedDeepSearchItems::Class(e) = class_class.found_item {
                                DeepTypeResult::ClassCall(
                                    ellie_core::definite::types::class_call::ClassCall {
                                        target: Box::new(Types::VariableType(
                                            ellie_core::definite::types::variable::VariableType {
                                                value: "object".to_owned(),
                                                reference: e.hash,
                                                pos: defs::Cursor::default(),
                                            },
                                        )),
                                        params: vec![],
                                        keyword_pos: defs::Cursor::default(),
                                        target_pos: defs::Cursor::default(),
                                        resolved_generics: vec![],
                                        generic_parameters: vec![],
                                        pos: defs::Cursor::default(),
                                    },
                                )
                            } else {
                                unreachable!("Ellie must ensure that class is a object, and no one can replace it");
                            }
                        } else {
                            let path = parser.find_page(page_id).unwrap().path.clone();
                            errors.push(error::error_list::ERROR_S6.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: "object".to_string(),
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
                    ProcessedDeepSearchItems::FunctionParameter(e) => {
                        match generate_type_from_defining(e.rtype, page_id, parser) {
                            Some(e) => match e {
                                Types::Byte(e) => DeepTypeResult::Byte(e),
                                Types::Integer(e) => DeepTypeResult::Integer(e),
                                Types::Decimal(e) => DeepTypeResult::Decimal(e),
                                Types::Bool(e) => DeepTypeResult::Bool(e),
                                Types::String(e) => DeepTypeResult::String(e),
                                Types::Char(e) => DeepTypeResult::Char(e),
                                Types::Collective(e) => DeepTypeResult::Collective(e),
                                Types::BraceReference(e) => DeepTypeResult::BraceReference(e),
                                Types::Operator(e) => DeepTypeResult::Operator(e),
                                Types::Cloak(e) => DeepTypeResult::Cloak(e),
                                Types::Array(e) => DeepTypeResult::Array(e),
                                Types::Function(e) => DeepTypeResult::Function(e),
                                Types::ClassCall(e) => DeepTypeResult::ClassCall(e),
                                Types::FunctionCall(e) => DeepTypeResult::FunctionCall(e),
                                Types::Void => DeepTypeResult::Void,
                                Types::Null => DeepTypeResult::Null,
                                Types::Dynamic => DeepTypeResult::Dynamic,
                                _ => unreachable!(),
                            },
                            None => panic!("This should never happen"),
                        }
                    }
                    ProcessedDeepSearchItems::Getter(e) => {
                        match generate_type_from_defining(e.return_type, page_id, parser) {
                            Some(e) => match e {
                                Types::Byte(e) => DeepTypeResult::Byte(e),
                                Types::Integer(e) => DeepTypeResult::Integer(e),
                                Types::Decimal(e) => DeepTypeResult::Decimal(e),
                                Types::Bool(e) => DeepTypeResult::Bool(e),
                                Types::String(e) => DeepTypeResult::String(e),
                                Types::Char(e) => DeepTypeResult::Char(e),
                                Types::Collective(e) => DeepTypeResult::Collective(e),
                                Types::BraceReference(e) => DeepTypeResult::BraceReference(e),
                                Types::Operator(e) => DeepTypeResult::Operator(e),
                                Types::Cloak(e) => DeepTypeResult::Cloak(e),
                                Types::Array(e) => DeepTypeResult::Array(e),
                                Types::Function(e) => DeepTypeResult::Function(e),
                                Types::ClassCall(e) => DeepTypeResult::ClassCall(e),
                                Types::FunctionCall(e) => DeepTypeResult::FunctionCall(e),
                                Types::Void => DeepTypeResult::Void,
                                Types::Null => DeepTypeResult::Null,
                                Types::Dynamic => DeepTypeResult::Dynamic,
                                _ => DeepTypeResult::NotFound,
                            },
                            None => DeepTypeResult::NotFound,
                        }
                    }
                    ProcessedDeepSearchItems::Setter(e) => {
                        match generate_type_from_defining(e.rtype, page_id, parser) {
                            Some(e) => match e {
                                Types::Byte(e) => DeepTypeResult::Byte(e),
                                Types::Integer(e) => DeepTypeResult::Integer(e),
                                Types::Decimal(e) => DeepTypeResult::Decimal(e),
                                Types::Bool(e) => DeepTypeResult::Bool(e),
                                Types::String(e) => DeepTypeResult::String(e),
                                Types::Char(e) => DeepTypeResult::Char(e),
                                Types::Collective(e) => DeepTypeResult::Collective(e),
                                Types::BraceReference(e) => DeepTypeResult::BraceReference(e),
                                Types::Operator(e) => DeepTypeResult::Operator(e),
                                Types::Cloak(e) => DeepTypeResult::Cloak(e),
                                Types::Array(e) => DeepTypeResult::Array(e),
                                Types::Function(e) => DeepTypeResult::Function(e),
                                Types::ClassCall(e) => DeepTypeResult::ClassCall(e),
                                Types::FunctionCall(e) => DeepTypeResult::FunctionCall(e),
                                Types::Void => DeepTypeResult::Void,
                                Types::Null => DeepTypeResult::Null,
                                Types::Dynamic => DeepTypeResult::Dynamic,
                                _ => DeepTypeResult::NotFound,
                            },
                            None => DeepTypeResult::NotFound,
                        }
                    }
                    ProcessedDeepSearchItems::Function(e) => {
                        DeepTypeResult::Function(ellie_core::definite::types::function::Function {
                            native: false,
                            parameters: e
                                .parameters
                                .iter()
                                .map(
                                    |x| ellie_core::definite::types::function::FunctionParameter {
                                        name: x.name.clone(),
                                        rtype: Some(x.rtype.clone()),
                                        rtype_pos: x.rtype_pos.clone(),
                                        name_pos: x.name_pos.clone(),
                                    },
                                )
                                .collect::<Vec<_>>(),
                            has_parameter_definings: true,
                            return_type: e.return_type,
                            inside_code: vec![],
                            return_pos: defs::Cursor::default(),
                            arrow_function: false,
                        })
                    }
                    ProcessedDeepSearchItems::NativeFunction(e) => {
                        DeepTypeResult::Function(ellie_core::definite::types::function::Function {
                            native: true,
                            parameters: e
                                .parameters
                                .iter()
                                .map(
                                    |x| ellie_core::definite::types::function::FunctionParameter {
                                        name: x.name.clone(),
                                        rtype: Some(x.rtype.clone()),
                                        rtype_pos: x.rtype_pos.clone(),
                                        name_pos: x.name_pos.clone(),
                                    },
                                )
                                .collect::<Vec<_>>(),
                            has_parameter_definings: true,
                            return_type: e.return_type,
                            inside_code: vec![],
                            return_pos: defs::Cursor::default(),
                            arrow_function: false,
                        })
                    }
                    ProcessedDeepSearchItems::ImportReference(_) => todo!(),
                    ProcessedDeepSearchItems::GenericItem(_) => todo!(),
                    ProcessedDeepSearchItems::None => todo!(),
                    ProcessedDeepSearchItems::Enum(enum_data) => DeepTypeResult::Enum(enum_data),
                    ProcessedDeepSearchItems::SelfItem(self_item) => {
                        DeepTypeResult::SelfItem(self_item)

                        /*
                        DeepTypeResult::ClassCall(
                            ellie_core::definite::types::class_call::ClassCall {
                                target: Box::new(Types::VariableType(
                                    ellie_core::definite::types::variable::VariableType {
                                        value: targeted_class.name.clone(),
                                        reference: targeted_class.hash,
                                        pos: targeted_class.pos.clone(),
                                    },
                                )),
                                params: vec![],
                                keyword_pos: defs::Cursor::default(),
                                target_pos: defs::Cursor::default(),
                                resolved_generics: vec![],
                                generic_parameters: vec![],
                                pos: defs::Cursor::default(),
                            },
                        )
                        */
                    }
                    ProcessedDeepSearchItems::ClassInstance(class_instance) => {
                        DeepTypeResult::ClassInstance(class_instance)
                    }
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
            let targeted = match resolve_type(
                *as_keyword.target.clone(),
                page_id,
                parser,
                errors,
                Some(as_keyword.target_pos),
            ) {
                Some(e) => e,
                None => {
                    return DeepTypeResult::NotFound;
                }
            };

            if errors.len() > 0 {
                DeepTypeResult::NotFound
            } else {
                let rtype = crate::processors::definer_processor::process(
                    ellie_tokenizer::syntax::items::definers::DefinerTypes::Dynamic
                        .from_definite(as_keyword.rtype.clone()),
                    parser,
                    page_id,
                    None,
                );

                match rtype {
                    Ok(rtype) => {
                        fn generate_type_from_defining(
                            rtype: definers::DefinerCollecting,
                        ) -> Option<Types> {
                            match rtype {
                                definers::DefinerCollecting::Function(function) => {
                                    Some(Types::Function(
                                        ellie_core::definite::types::function::Function {
                                            parameters: function.params.iter().map(|x| {
                                                ellie_core::definite::types::function::FunctionParameter {
                                                    name: "anonymous".to_owned(),
                                                    rtype: Some(x.clone()),
                                                    rtype_pos: defs::Cursor::default(),
                                                    name_pos: defs::Cursor::default(),
                                                }
                                            }).collect::<Vec<_>>(),
                                            has_parameter_definings: true,
                                            return_type: *function.returning,
                                            inside_code: vec![],
                                            return_pos: defs::Cursor::default(),
                                            arrow_function: false,
                                            native: false,
                                        }
                                    ))
                                }
                                definers::DefinerCollecting::Generic(generic) => {
                                    if generic.rtype == "int" {
                                        Some(Types::Integer(
                                            ellie_core::definite::types::integer::IntegerType {
                                                value: 0,
                                                pos: defs::Cursor::default(),
                                            },
                                        ))
                                    } else if generic.rtype == "float" {
                                        Some(Types::Decimal(
                                            ellie_core::definite::types::decimal::DecimalType {
                                                value: ellie_core::definite::types::decimal::DecimalTypeEnum::Float(0.0),
                                                pos: defs::Cursor::default(),
                                                is_double: false
                                            },
                                        ))
                                    } else if generic.rtype == "double" {
                                        Some(Types::Decimal(
                                            ellie_core::definite::types::decimal::DecimalType {
                                                value: ellie_core::definite::types::decimal::DecimalTypeEnum::Double(0.0),
                                                pos: defs::Cursor::default(),
                                                is_double: true
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
                                        Some(Types::Bool(
                                            ellie_core::definite::types::bool::BoolType {
                                                value: true,
                                            },
                                        ))
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
                                            Some(t) => Some(Types::Collective(
                                            ellie_core::definite::types::collective::CollectiveType {
                                                entries: vec![
                                                    ellie_core::definite::types::collective::CollectiveEntry {
                                                        key: "?".to_owned(),
                                                        value: t,
                                                        key_pos: defs::Cursor::default(),
                                                        value_pos: defs::Cursor::default(),
                                                    },
                                                ],
                                                pos: defs::Cursor::default(),
                                            },
                                        )),
                                            None => None,
                                        }
                                    } else if parent_generic.rtype == "function" {
                                        Some(Types::Function(
                                            ellie_core::definite::types::function::Function {
                                                parameters: vec![],
                                                has_parameter_definings: false,
                                                return_type: parent_generic.generics[0]
                                                    .value
                                                    .clone(),
                                                inside_code: vec![],
                                                return_pos: defs::Cursor::default(),
                                                arrow_function: false,
                                                native: false,
                                            },
                                        ))
                                    } else {
                                        None
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }

                        let resolved_type = generate_type_from_defining(rtype.clone());

                        match resolved_type {
                            Some(types) => match types {
                                Types::Integer(e) => DeepTypeResult::Integer(e),
                                Types::Decimal(e) => DeepTypeResult::Decimal(e),
                                Types::Bool(e) => DeepTypeResult::Bool(e),
                                Types::String(e) => DeepTypeResult::String(e),
                                Types::Char(e) => DeepTypeResult::Char(e),
                                Types::Collective(e) => DeepTypeResult::Collective(e),
                                Types::Cloak(e) => DeepTypeResult::Cloak(e),
                                Types::Array(e) => DeepTypeResult::Array(e),
                                Types::Function(e) => DeepTypeResult::Function(e),
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
                                            value: rtype.to_string(),
                                        },
                                    ],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    path,
                                    as_keyword.pos,
                                ));
                                DeepTypeResult::NotFound
                            }
                        }
                    }
                    Err(e) => {
                        errors.extend(e);
                        DeepTypeResult::NotFound
                    }
                }
            }
        }
        Types::Bool(bool) => DeepTypeResult::Bool(bool),
        Types::Void => DeepTypeResult::Void,
        Types::Null => DeepTypeResult::Null,
        Types::Dynamic => DeepTypeResult::Dynamic,
        Types::Function(f) => DeepTypeResult::Function(f),
        Types::Byte(byte) => DeepTypeResult::Byte(byte),
        Types::EnumData(e) => DeepTypeResult::EnumData(e),
        Types::ClassInstance(_) => todo!(),
    }
}

pub fn resolve_absolute_definer(
    parser: &mut Parser,
    page_id: usize,
    rtype: definers::DefinerCollecting,
) -> Result<definers::DefinerCollecting, Vec<error::Error>> {
    match rtype {
        definers::DefinerCollecting::Array(e) => {
            let inner_type = match resolve_absolute_definer(parser, page_id, *e.rtype) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

            let array_type = match find_type("array".to_string(), page_id, parser) {
                Some(e) => e,
                None => {
                    return Err(vec![error::error_list::ERROR_S38.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: "array".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        parser.find_page(page_id).unwrap().path.clone(),
                        e.pos,
                    )])
                }
            };

            Ok(definers::DefinerCollecting::ParentGeneric(
                definers::ParentGenericType {
                    rtype: "array".to_string(),
                    parent_pos: array_type.pos,
                    generics: vec![definers::GenericParameter {
                        value: inner_type,
                        pos: e.pos,
                    }],
                    hash: array_type.hash,
                },
            ))
        }
        definers::DefinerCollecting::Generic(e) => {
            match find_type(e.rtype.clone(), page_id, parser) {
                Some(e) => Ok(ellie_core::definite::definers::DefinerCollecting::Generic(
                    e,
                )),
                None => Err(vec![error::error_list::ERROR_S38.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: e.rtype.clone(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    parser.find_page(page_id).unwrap().path.clone(),
                    e.pos,
                )]),
            }
        }
        definers::DefinerCollecting::ParentGeneric(e) => {
            let mut errors = Vec::new();
            let generics = e
                .generics
                .iter()
                .filter_map(
                    |x| match resolve_absolute_definer(parser, page_id, x.value.clone()) {
                        Ok(e) => Some(definers::GenericParameter {
                            value: e,
                            pos: x.pos,
                        }),
                        Err(e) => {
                            errors.extend(e);
                            None
                        }
                    },
                )
                .collect::<Vec<_>>();

            if !errors.is_empty() {
                return Err(errors);
            }

            let parent_type = match find_type(e.rtype.clone(), page_id, parser) {
                Some(e) => e,
                None => {
                    return Err(vec![error::error_list::ERROR_S38.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: e.rtype.clone(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        parser.find_page(page_id).unwrap().path.clone(),
                        e.parent_pos,
                    )])
                }
            };

            Ok(definers::DefinerCollecting::ParentGeneric(
                definers::ParentGenericType {
                    rtype: e.rtype.clone(),
                    parent_pos: parent_type.pos,
                    generics,
                    hash: parent_type.hash,
                },
            ))
        }
        definers::DefinerCollecting::Function(e) => Ok(definers::DefinerCollecting::Function(e)),
        definers::DefinerCollecting::Cloak(e) => {
            match find_type("cloak".to_string(), page_id, parser) {
                Some(e) => Ok(definers::DefinerCollecting::Generic(e)),
                None => Err(vec![error::error_list::ERROR_S38.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: "cloak".to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    parser.find_page(page_id).unwrap().path.clone(),
                    e.pos,
                )]),
            }
        }
        definers::DefinerCollecting::Collective(e) => {
            let inner_type = match resolve_absolute_definer(parser, page_id, *e.value) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

            let collective_type = match find_type("collective".to_string(), page_id, parser) {
                Some(e) => e,
                None => {
                    return Err(vec![error::error_list::ERROR_S38.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: "collective".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        parser.find_page(page_id).unwrap().path.clone(),
                        e.pos,
                    )])
                }
            };

            Ok(definers::DefinerCollecting::ParentGeneric(
                definers::ParentGenericType {
                    rtype: "collective".to_string(),
                    parent_pos: collective_type.pos,
                    generics: vec![definers::GenericParameter {
                        value: inner_type,
                        pos: e.pos,
                    }],
                    hash: collective_type.hash,
                },
            ))
        }
        definers::DefinerCollecting::Nullable(e) => {
            let inner_type = match resolve_absolute_definer(parser, page_id, *e.value) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

            let nullable_type = match find_type("collective".to_string(), page_id, parser) {
                Some(e) => e,
                None => {
                    return Err(vec![error::error_list::ERROR_S38.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: "collective".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        parser.find_page(page_id).unwrap().path.clone(),
                        e.pos,
                    )])
                }
            };

            Ok(definers::DefinerCollecting::ParentGeneric(
                definers::ParentGenericType {
                    rtype: "nullAble".to_string(),
                    parent_pos: nullable_type.pos,
                    generics: vec![definers::GenericParameter {
                        value: inner_type,
                        pos: e.pos,
                    }],
                    hash: nullable_type.hash,
                },
            ))
        }
        definers::DefinerCollecting::Dynamic => {
            match find_type("dyn".to_string(), page_id, parser) {
                Some(e) => Ok(definers::DefinerCollecting::Generic(e)),
                None => Err(vec![error::error_list::ERROR_S38.clone().build_with_path(
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: "collective".to_string(),
                    }],
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                    parser.find_page(page_id).unwrap().path.clone(),
                    defs::Cursor::default(),
                )]),
            }
        }
        definers::DefinerCollecting::EnumField(e) => Ok(definers::DefinerCollecting::EnumField(e)),
        definers::DefinerCollecting::ClassInstance(e) => Ok(definers::DefinerCollecting::Generic(
            definers::GenericType {
                rtype: e.class_name.clone(),
                pos: defs::Cursor::default(),
                hash: e.class_hash,
            },
        )),
    }
}

pub fn resolve_deep_type(
    parser: &mut Parser,
    page_id: usize,
    rtype: Types,
    errors: &mut Vec<error::Error>,
) -> DeepTypeResult {
    iterate_deep_type(parser, page_id, rtype, errors)
}

#[derive(Debug, Clone, EnumAsInner)]
pub enum ProcessedDeepSearchItems {
    Class(ellie_core::definite::items::class::Class),

    Variable(ellie_core::definite::items::variable::Variable),
    Function(ellie_core::definite::items::function::Function),
    Enum(ellie_core::definite::items::enum_type::EnumType),
    NativeFunction(ellie_core::definite::items::native_function::NativeFunction),
    Getter(ellie_core::definite::items::getter::Getter),
    Setter(ellie_core::definite::items::setter::Setter),
    ImportReference(ellie_core::definite::items::import::Import),
    SelfItem(ellie_core::definite::items::self_item::SelfItem),
    ClassInstance(ellie_core::definite::types::class_instance::ClassInstance),
    GenericItem(ellie_core::definite::items::generic::Generic),
    FunctionParameter(ellie_core::definite::items::function_parameter::FunctionParameter),
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
    pub found_page: FoundPage,
}

pub fn deep_search_hash(
    parser: &mut Parser,
    target_page: usize,
    target_hash: usize,
    searched: Vec<usize>,
    _level: usize,
) -> ProcessedDeepSearchResult {
    let mut level = _level;
    let mut found = false;
    let mut found_type = ProcessedDeepSearchItems::None;
    let mut found_pos = None;
    let mut found_page = FoundPage::default();
    let has_mixup = false;
    let mut inner_page = None;
    let mut searched: Vec<usize> = searched;
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
                    let internal_deps = page
                        .dependencies
                        .iter()
                        .filter_map(|x| {
                            if x.public || x.deep_link.is_some() {
                                Some(x.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<Dependency>>();
                    self_dependencies.extend(internal_deps);

                    for item in &page.items {
                        match item.clone() {
                            Collecting::Variable(e) => {
                                if e.hash == target_hash
                                    && (e.public || level == 0 || dep.deep_link.is_some())
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::Variable(e);
                                }
                            }
                            Collecting::Function(e) => {
                                if e.hash == target_hash
                                    && (e.public || level == 0 || dep.deep_link.is_some())
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::Function(e);
                                }
                            }
                            Collecting::Enum(e) => {
                                if e.hash == target_hash
                                    && (e.public || level == 0 || dep.deep_link.is_some())
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::Enum(e);
                                }
                            }
                            Collecting::Import(e) => {
                                if e.reference != ""
                                    && e.hash == target_hash
                                    && (e.public
                                        || level == 0
                                        || dep.deep_link.is_some()
                                        || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::ImportReference(e);
                                }
                            }
                            Collecting::Class(e) => {
                                if e.hash == target_hash
                                    && (e.public
                                        || level == 0
                                        || dep.deep_link.is_some()
                                        || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::Class(e);
                                }
                            }
                            Collecting::Generic(e) => {
                                if e.hash == target_hash
                                    && (level == 0
                                        || dep.deep_link.is_some()
                                        || matches!(inner_page, Some(ref parent_page_hash) if parent_page_hash == &page.hash))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::GenericItem(e);
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
    target_page: usize,
    name: String,
    ignore_hash: Option<usize>,
    searched: Vec<usize>,
    _level: usize,
) -> ProcessedDeepSearchResult {
    let mut level = _level;
    let mut found = false;
    let mut found_type = ProcessedDeepSearchItems::None;
    let mut found_pos = None;
    let mut found_page = FoundPage::default();
    let has_mixup = false;
    let mut inner_page = None;
    let mut searched: Vec<usize> = searched;
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
                    let internal_deps = page
                        .dependencies
                        .iter()
                        .filter_map(|x| {
                            if x.public || x.deep_link.is_some() {
                                Some(x.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<Dependency>>();
                    self_dependencies.extend(internal_deps);

                    for item in page.items.iter() {
                        match item {
                            Collecting::Variable(e) => {
                                if e.name == name
                                    && (e.public || level == 0 || dep.deep_link.is_some())
                                    && (ignore_hash.is_none()
                                        || matches!(ignore_hash, Some(ref t) if &e.hash != t))
                                {
                                    found_pos = Some(e.pos);
                                    found = true;
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::Variable(e.clone());
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
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::Enum(e.clone());
                                }
                            }
                            Collecting::FunctionParameter(e) => {
                                if e.name == name {
                                    found_pos = Some(defs::Cursor {
                                        range_start: e.name_pos.range_start,
                                        range_end: e.rtype_pos.range_end,
                                    });
                                    found = true;
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type =
                                        ProcessedDeepSearchItems::FunctionParameter(e.clone());
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
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::Function(e.clone());
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
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type =
                                        ProcessedDeepSearchItems::NativeFunction(e.clone());
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
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::Getter(e.clone());
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
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::Setter(e.clone());
                                }
                            }
                            Collecting::Import(e) => {
                                if e.reference != ""
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
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type =
                                        ProcessedDeepSearchItems::ImportReference(e.clone());
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
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::Class(e.clone());
                                }
                            }
                            Collecting::ClassInstance(e) => {
                                if name == "self" {
                                    found_pos = None;
                                    found = true;
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::ClassInstance(e.clone());
                                }
                            }
                            Collecting::SelfItem(e) => {
                                if name == "self" {
                                    found_pos = None;
                                    found = true;
                                    found_page = FoundPage::fill_from_processed(&page);
                                    found_type = ProcessedDeepSearchItems::SelfItem(e.clone());
                                }
                            }
                            _ => (),
                        }
                        if found {
                            break;
                        }
                    }
                    if found {
                        break;
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
    target_page: usize,
    parser: &mut Parser,
) -> Option<definers::GenericType> {
    let result = parser.deep_search(target_page, rtype.clone(), None, vec![], 0, None);
    if result.found {
        match result.found_item {
            DeepSearchItems::Class(e) => Some(definers::GenericType {
                rtype,
                pos: e.pos,
                hash: e.hash,
            }),
            DeepSearchItems::None => None,
            DeepSearchItems::Enum(_) => todo!(),
            _ => {
                unreachable!(
                    "Unexpected internal crash, parser should have prevented this, {:?}",
                    result
                );
            }
        }
    } else {
        None
    }
}

pub fn resolve_type(
    target_type: Types,
    target_page: usize,
    parser: &mut Parser,
    errors: &mut Vec<error::Error>,
    pos: Option<defs::Cursor>,
) -> Option<definers::DefinerCollecting> {
    let deep_type = crate::deep_search_extensions::resolve_deep_type(
        parser,
        target_page,
        target_type.clone(),
        errors,
    );

    std::println!("Resolved deep type: {:#?}\n{:#?}", deep_type, target_type);

    match deep_type {
        DeepTypeResult::Integer(_) => {
            let int_type = find_type("int".to_string(), target_page, parser);

            match int_type {
                Some(e) => Some(definers::DefinerCollecting::Generic(e)),
                None => {
                    if let Some(pos) = pos {
                        errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: "int".to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            parser.find_page(target_page).unwrap().path.clone(),
                            pos,
                        ));
                        return None;
                    } else {
                        panic!("Unhandled behaviour, failed to find int type");
                    }
                }
            }
        }
        DeepTypeResult::Byte(_) => {
            let byte_type = find_type("byte".to_string(), target_page, parser);
            match byte_type {
                Some(e) => Some(definers::DefinerCollecting::Generic(e)),
                None => {
                    if let Some(pos) = pos {
                        errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: "byte".to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            parser.find_page(target_page).unwrap().path.clone(),
                            pos,
                        ));
                        return None;
                    } else {
                        panic!("Unhandled behaviour, failed to find byte type");
                    }
                }
            }
        }
        DeepTypeResult::Decimal(decimal_type) => {
            let generic = if decimal_type.is_double {
                "double"
            } else {
                "float"
            };

            let decimal_type = find_type(generic.to_string(), target_page, parser);
            match decimal_type {
                Some(e) => Some(definers::DefinerCollecting::Generic(e)),
                None => {
                    if let Some(pos) = pos {
                        errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: generic.to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            parser.find_page(target_page).unwrap().path.clone(),
                            pos,
                        ));
                        return None;
                    } else {
                        panic!("Unhandled behaviour, failed to find {generic} type");
                    }
                }
            }
        }
        DeepTypeResult::Bool(_) => {
            let bool_type = find_type("bool".to_string(), target_page, parser);
            match bool_type {
                Some(e) => Some(definers::DefinerCollecting::Generic(e)),
                None => {
                    if let Some(pos) = pos {
                        errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: "bool".to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            parser.find_page(target_page).unwrap().path.clone(),
                            pos,
                        ));
                        return None;
                    } else {
                        panic!("Unhandled behaviour, failed to find bool type");
                    }
                }
            }
        }
        DeepTypeResult::String(_) => {
            let string_type = find_type("string".to_string(), target_page, parser);
            match string_type {
                Some(e) => Some(definers::DefinerCollecting::Generic(e)),
                None => {
                    if let Some(pos) = pos {
                        errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: "string".to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            parser.find_page(target_page).unwrap().path.clone(),
                            pos,
                        ));
                        return None;
                    } else {
                        panic!("Unhandled behaviour, failed to find string type");
                    }
                }
            }
        }
        DeepTypeResult::Char(_) => {
            let char_type = find_type("char".to_string(), target_page, parser);
            match char_type {
                Some(e) => Some(definers::DefinerCollecting::Generic(e)),
                None => {
                    if let Some(pos) = pos {
                        errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: "char".to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            parser.find_page(target_page).unwrap().path.clone(),
                            pos,
                        ));
                        return None;
                    } else {
                        panic!("Unhandled behaviour, failed to find char type");
                    }
                }
            }
        }
        DeepTypeResult::Collective(_) => todo!(),
        DeepTypeResult::Operator(operator) => {
            let value_type = match operator.operator.clone() {
                ellie_core::definite::types::operator::Operators::ComparisonType(_) => (
                    find_type("bool".to_string(), target_page, parser),
                    "bool".to_string(),
                ),
                ellie_core::definite::types::operator::Operators::LogicalType(_) => (
                    find_type("bool".to_string(), target_page, parser),
                    "bool".to_string(),
                ),
                ellie_core::definite::types::operator::Operators::ArithmeticType(_) => {
                    let first =
                        resolve_type(*operator.first.clone(), target_page, parser, errors, pos);

                    match first {
                        Some(first) => (
                            match first.clone() {
                                definers::DefinerCollecting::Generic(e) => Some(e),
                                _ => None,
                            },
                            first.to_string(),
                        ),
                        None => (None, String::new()),
                    }
                }
                ellie_core::definite::types::operator::Operators::AssignmentType(_) => {
                    let res =
                        resolve_type(*operator.first.clone(), target_page, parser, errors, pos)
                            .unwrap()
                            .to_string();
                    errors.push(error::error_list::ERROR_S3.clone().build_with_path(
                        vec![
                            error::ErrorBuildField {
                                key: "token1".to_owned(),
                                value: "bool".to_string(),
                            },
                            error::ErrorBuildField {
                                key: "token1".to_owned(),
                                value: res,
                            },
                        ],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        parser.find_page(target_page).unwrap().path.clone(),
                        match pos {
                            Some(e) => e,
                            None => defs::Cursor::default(),
                        },
                    ));
                    return None;
                }
                ellie_core::definite::types::operator::Operators::Null => {
                    unreachable!()
                }
            };

            match value_type.0 {
                Some(value_type) => Some(definers::DefinerCollecting::Generic(value_type)),
                None => {
                    errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: value_type.1,
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        parser.find_page(target_page).unwrap().path.clone(),
                        match pos {
                            Some(e) => e,
                            None => defs::Cursor::default(),
                        },
                    ));
                    None
                }
            }
        }
        DeepTypeResult::Cloak(cloak_type) => {
            #[derive(PartialEq, EnumAsInner, Clone)]
            enum GenericExists {
                Generic(definers::DefinerCollecting),
                Null,
            }
            let mut child_generic = GenericExists::Null;
            for entry in cloak_type.collective {
                match resolve_type(entry.value, target_page, parser, errors, pos) {
                    Some(resolved) => {
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
                    None => {
                        return None;
                    }
                }
            }
            let cloak_type = find_type("cloak".to_string(), target_page, parser);
            match cloak_type {
                Some(cloak_generic) => {
                    let val = child_generic
                        .as_generic()
                        .unwrap_or(&definers::DefinerCollecting::Dynamic)
                        .clone();
                    Some(definers::DefinerCollecting::ParentGeneric(
                        definers::ParentGenericType {
                            rtype: "cloak".to_string(),
                            generics: vec![definers::GenericParameter {
                                value: val,
                                pos: defs::Cursor::default(),
                            }],
                            hash: cloak_generic.hash,
                            parent_pos: defs::Cursor::default(),
                        },
                    ))
                }
                None => {
                    errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: "cloak".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        parser.find_page(target_page).unwrap().path.clone(),
                        match pos {
                            Some(e) => e,
                            None => unreachable!(),
                        },
                    ));
                    None
                }
            }
        }
        DeepTypeResult::Array(array_type) => {
            #[derive(PartialEq, EnumAsInner, Clone)]
            enum GenericExists {
                Generic(definers::DefinerCollecting),
                Null,
            }
            let mut child_generic = GenericExists::Null;
            for entry in array_type.collective {
                match resolve_type(entry.value, target_page, parser, errors, pos) {
                    Some(resolved) => {
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
                    None => {
                        return None;
                    }
                }
            }

            let array_type = find_type("array".to_string(), target_page, parser);
            match array_type {
                Some(array_generic) => {
                    let val = child_generic
                        .as_generic()
                        .unwrap_or(&definers::DefinerCollecting::Dynamic)
                        .clone();
                    Some(definers::DefinerCollecting::ParentGeneric(
                        definers::ParentGenericType {
                            rtype: "array".to_string(),
                            generics: vec![definers::GenericParameter {
                                value: val,
                                pos: defs::Cursor::default(),
                            }],
                            hash: array_generic.hash,
                            parent_pos: defs::Cursor::default(),
                        },
                    ))
                }
                None => {
                    errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: "array".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        parser.find_page(target_page).unwrap().path.clone(),
                        match pos {
                            Some(e) => e,
                            None => unreachable!(),
                        },
                    ));
                    None
                }
            }
        }
        DeepTypeResult::ClassCall(class_call) => match (*class_call.target).clone() {
            Types::Cloak(cloak) => {
                if cloak.collective.len() == 1 {
                    unimplemented!()
                } else {
                    unreachable!()
                }
            }
            Types::VariableType(variable) => {
                let deep_search_result = parser.deep_search(
                    target_page,
                    variable.value.clone(),
                    None,
                    Vec::new(),
                    0,
                    None,
                );
                let targeted_class = find_type(variable.value.clone(), target_page, parser)
                    .unwrap_or_else(|| panic!("Failed to find class {}", variable.value));

                if deep_search_result.found {
                    match deep_search_result.found_item {
                        crate::parser::DeepSearchItems::Class(e) => {
                            /*
                            if e.generic_definings.len() != class_call.generic_parameters.len() {
                                unreachable!()
                            } else
                            */
                            if let Some(_) = e.body.iter().find_map(|x| match x {
                                ellie_tokenizer::processors::items::Processors::Constructor(e) => {
                                    Some(e)
                                }
                                _ => None,
                            }) {
                                if class_call.generic_parameters.is_empty() {
                                    Some(definers::DefinerCollecting::Generic(
                                        definers::GenericType {
                                            rtype: variable.value.clone(),
                                            hash: targeted_class.hash,
                                            pos: defs::Cursor::default(),
                                        },
                                    ))
                                } else {
                                    Some(definers::DefinerCollecting::ParentGeneric(
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
                                    ))
                                }
                            } else {
                                if class_call.generic_parameters.is_empty() {
                                    Some(definers::DefinerCollecting::Generic(
                                        definers::GenericType {
                                            rtype: variable.value.clone(),
                                            hash: targeted_class.hash,
                                            pos: defs::Cursor::default(),
                                        },
                                    ))
                                } else {
                                    Some(definers::DefinerCollecting::ParentGeneric(
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
                                    ))
                                }
                            }
                        }
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
        DeepTypeResult::FunctionCall(e) => Some(e.returning),
        DeepTypeResult::Void => {
            let void_type = find_type("void".to_string(), target_page, parser);
            match void_type {
                Some(e) => Some(definers::DefinerCollecting::Generic(e)),
                None => {
                    if let Some(pos) = pos {
                        errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: "void".to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            parser.find_page(target_page).unwrap().path.clone(),
                            pos,
                        ));
                        return None;
                    } else {
                        panic!("Unhandled behaviour, failed to find void type");
                    }
                }
            }
        }
        DeepTypeResult::Null => {
            let null_type = find_type("null".to_string(), target_page, parser);
            match null_type {
                Some(e) => Some(definers::DefinerCollecting::Generic(e)),
                None => {
                    if let Some(pos) = pos {
                        errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: "null".to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            parser.find_page(target_page).unwrap().path.clone(),
                            pos,
                        ));
                        return None;
                    } else {
                        panic!("Unhandled behaviour, failed to find null type");
                    }
                }
            }
        }
        DeepTypeResult::NotFound => None,
        DeepTypeResult::BraceReference(e) => {
            let nullable_type = find_type("nullAble".to_string(), target_page, parser);
            match nullable_type {
                Some(nullable_generic) => {
                    let nullable_child_generic = match *e.reference.clone() {
                        Types::Array(_) => {
                            let array_type = match resolve_type(
                                *e.reference,
                                target_page,
                                parser,
                                errors,
                                Some(e.reference_pos),
                            ) {
                                Some(e) => e,
                                None => return None,
                            };
                            array_type.as_parent_generic().unwrap().generics[0]
                                .value
                                .clone()
                        }
                        Types::Cloak(_) => {
                            let cloak_type = match resolve_type(
                                *e.reference,
                                target_page,
                                parser,
                                errors,
                                Some(e.reference_pos),
                            ) {
                                Some(e) => e,
                                None => return None,
                            };
                            cloak_type.as_parent_generic().unwrap().generics[0]
                                .value
                                .clone()
                        }
                        _ => {
                            unimplemented!("Custom index queries are not yet supported",)
                        }
                    };
                    Some(definers::DefinerCollecting::ParentGeneric(
                        definers::ParentGenericType {
                            rtype: "nullAble".to_string(),
                            generics: vec![definers::GenericParameter {
                                value: nullable_child_generic,
                                pos: defs::Cursor::default(),
                            }],
                            hash: nullable_generic.hash,
                            parent_pos: defs::Cursor::default(),
                        },
                    ))
                }
                None => {
                    if let Some(pos) = pos {
                        errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: "nullAble".to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            parser.find_page(target_page).unwrap().path.clone(),
                            pos,
                        ));
                        return None;
                    } else {
                        panic!("Unhandled behaviour, failed to find nullAble type");
                    }
                }
            }
        }
        DeepTypeResult::Dynamic => {
            let dyn_type = find_type("dyn".to_string(), target_page, parser);
            match dyn_type {
                Some(dynamic_type) => Some(definers::DefinerCollecting::Generic(dynamic_type)),
                None => {
                    if let Some(pos) = pos {
                        errors.push(error::error_list::ERROR_S38.clone().build_with_path(
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: "dyn".to_string(),
                            }],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            parser.find_page(target_page).unwrap().path.clone(),
                            pos,
                        ));
                        return None;
                    } else {
                        panic!("Unhandled behaviour, failed to find dyn type");
                    }
                }
            }
        }
        DeepTypeResult::Function(e) => Some(definers::DefinerCollecting::Function(
            definers::FunctionType {
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
            },
        )),
        DeepTypeResult::EnumData(e) => {
            let (enum_hash, enum_name) = match *e.reference {
                ellie_core::definite::types::Types::VariableType(e) => (e.reference, e.value),
                _ => unreachable!("Parser should have prevented this"),
            };

            Some(definers::DefinerCollecting::EnumField(
                definers::EnumField {
                    field_name: e.field_name,
                    field_data: match e.value {
                        ellie_core::definite::types::enum_data::Pointer::NoData => {
                            definers::EnumFieldData::NoData
                        }
                        ellie_core::definite::types::enum_data::Pointer::Data(pointer_data) => {
                            match resolve_type(*pointer_data, target_page, parser, errors, pos) {
                                Some(data_type) => {
                                    definers::EnumFieldData::Data(Box::new(data_type))
                                }
                                None => {
                                    return None;
                                }
                            }
                        }
                    },
                    name: enum_name,
                    hash: enum_hash,
                },
            ))
        }
        DeepTypeResult::Enum(e) => Some(definers::DefinerCollecting::Generic(
            definers::GenericType {
                rtype: e.name,
                pos: defs::Cursor::default(),
                hash: e.hash,
            },
        )),
        DeepTypeResult::ClassInstance(instance) => {
            Some(ellie_core::definite::definers::DefinerCollecting::ClassInstance(instance))
        }
        DeepTypeResult::SelfItem(e) => Some(
            ellie_core::definite::definers::DefinerCollecting::Generic(definers::GenericType {
                rtype: "self".to_string(),
                pos: e.pos,
                hash: e.class_hash,
            }),
        ),
    }
}
