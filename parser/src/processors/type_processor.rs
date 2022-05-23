use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, string::String};
use core::panic;
use ellie_core::definite::definers::DefinerCollecting;
use ellie_core::definite::types::Types;
use ellie_core::{
    definite::{items::Collecting, types, Converter},
    error,
};
use ellie_tokenizer::processors::types::Processors;

use crate::deep_search_extensions::{deep_search, deep_search_hash, find_type, resolve_type};

pub fn process(
    from: Processors,
    parser: &mut super::Parser,
    page_id: u64,
    ignore_hash: Option<u64>,
    include_setter: bool,
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
                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                    crate::parser::DeepSearchItems::Class(_) => {
                        //ERROR_S15
                        let path = parser.find_page(page_id).unwrap().path.clone();
                        errors.push(error::error_list::ERROR_S15.clone().build_with_path(
                            vec![],
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                            path.clone(),
                            variable.data.pos,
                        ));
                        return Err(errors);
                    }
                    crate::parser::DeepSearchItems::Variable(e) => {
                        Ok(types::Types::VariableType(types::variable::VariableType {
                            value: e.name,
                            reference: e.hash,
                            pos: from.get_pos(),
                        }))
                    }
                    crate::parser::DeepSearchItems::Getter(e) => {
                        match crate::deep_search_extensions::generate_type_from_defining(
                            e.return_type.definer_type.to_definite(),
                            page_id,
                            parser,
                        ) {
                            Some(e) => Ok(e),
                            None => Err(errors),
                        }
                    }
                    crate::parser::DeepSearchItems::Setter(e) => {
                        if include_setter {
                            Ok(Types::SetterCall(
                                e.parameters
                                    .first()
                                    .unwrap()
                                    .rtype
                                    .definer_type
                                    .clone()
                                    .to_definite(),
                            ))
                        } else {
                            errors.push(error::error_list::ERROR_S23.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: e.name,
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                parser.find_page(page_id).unwrap().path.clone(),
                                variable.data.pos,
                            ));
                            Err(errors)
                        }
                    }
                    crate::parser::DeepSearchItems::Function(function) => {
                        match crate::processors::definer_processor::process(
                            function.return_type.definer_type,
                            parser,
                            page_id,
                            ignore_hash,
                        ) {
                            Ok(resolved_definer) => {
                                match find_type("function".to_owned(), page_id, parser) {
                                    Some(e) => {
                                        Ok(types::Types::FunctionCall(types::function_call::FunctionCall {
                                            pos: from.get_pos(),
                                            target: Box::new(
                                                ellie_core::definite::types::Types::VariableType(
                                                    ellie_core::definite::types::variable::VariableType {
                                                        value: "function".to_owned(),
                                                        reference: e.hash,
                                                        pos: ellie_core::defs::Cursor::default(),
                                                    },
                                                ),
                                            ),
                                            params: vec![],
                                            target_pos: ellie_core::defs::Cursor::default(),
                                            generic_parameters: vec![],
                                            returning: resolved_definer,
                                        }))

                                        /*
                                            Ok(types::Types::ClassCall(types::class_call::ClassCall {
                                            pos: from.get_pos(),
                                            target: Box::new(ellie_core::definite::types::Types::VariableType(
                                                ellie_core::definite::types::variable::VariableType {
                                                    value: "function".to_owned(),
                                                    reference: e.hash,
                                                    pos: ellie_core::defs::Cursor::default(),
                                                },
                                            )),
                                            params: vec![],
                                            keyword_pos: ellie_core::defs::Cursor::default(),
                                            target_pos: ellie_core::defs::Cursor::default(),
                                            generic_parameters: vec![],
                                            resolved_generics: vec![],
                                        }))
                                        */
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
                                                from.get_pos(),
                                            ),
                                        );
                                        Err(errors)
                                    }
                                }
                            }
                            Err(e) => {
                                errors.extend(e);
                                Err(errors)
                            }
                        }
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
                    alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                let response = process(i.value, parser, page_id, ignore_hash, false);
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
        Processors::Operator(operator) => {
            let processed_first_value = process(
                *operator.data.first.clone(),
                parser,
                page_id,
                ignore_hash,
                false,
            );

            let processed_second_value = process(
                *operator.data.second.clone(),
                parser,
                page_id,
                ignore_hash,
                false,
            );

            if processed_first_value.is_err() || processed_second_value.is_err() {
                if processed_first_value.is_err() {
                    errors.append(&mut processed_first_value.unwrap_err());
                }
                if processed_second_value.is_err() {
                    errors.append(&mut processed_second_value.unwrap_err());
                }
                return Err(errors);
            }

            let _first_value = match resolve_type(
                operator.data.first.to_definite(),
                page_id,
                parser,
                &mut errors,
                Some(operator.data.first_pos),
            ) {
                Some(e) => e,
                None => return Err(errors),
            };
            let _second_value = match resolve_type(
                operator.data.second.to_definite(),
                page_id,
                parser,
                &mut errors,
                Some(operator.data.second_pos),
            ) {
                Some(e) => e,
                None => return Err(errors),
            };

            Ok(types::Types::Operator(types::operator::OperatorType {
                cloaked: false,
                first: Box::new(processed_first_value.unwrap()),
                first_pos: operator.data.first_pos,
                second_pos: operator.data.second_pos,
                second: Box::new(processed_second_value.unwrap()),
                operator: operator.data.operator.to_definite(),
                pos: operator.data.pos,
            }))
        }
        Processors::Reference(reference) => {
            let processed_reference = process(
                *reference.data.reference.clone(),
                parser,
                page_id,
                ignore_hash,
                false,
            );
            match processed_reference {
                Ok(found_reference) => {
                    #[derive(Debug, Clone, PartialEq)]
                    enum AttributeType {
                        Property,
                        Method,
                        Setter,
                        Getter,
                    }

                    #[derive(Debug, Clone)]
                    struct Attribute {
                        _rtype: AttributeType,
                        name: String,
                        value: DefinerCollecting,
                    }

                    #[allow(dead_code)]
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
                                            value: 0,
                                            pos: ellie_core::defs::Cursor::default(),
                                        },
                                    ))
                                } else if generic.rtype == "float" {
                                    Some(types::Types::Float(
                                        ellie_core::definite::types::float::FloatType {
                                            value: 0.0,
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
                            DefinerCollecting::Function(function) => {
                                Some(types::Types::FunctionCall(
                                    ellie_core::definite::types::function_call::FunctionCall {
                                        target: Box::new(types::Types::Null),
                                        returning: *function.returning,
                                        target_pos: ellie_core::defs::Cursor::default(),
                                        params: vec![],
                                        generic_parameters: vec![],
                                        pos: ellie_core::defs::Cursor::default(),
                                    },
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
                        is_setter: bool,
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
                                                                let resolved_type = if e.has_type { e.rtype } else { match resolve_type(e.value, class_inner_page.hash, parser, &mut errors, Some(e.value_pos)) {
                                                                    Some(e) => e,
                                                                    None => return None,
                                                                } };
                                                                Some(Attribute {
                                                                    _rtype: AttributeType::Property,
                                                                    name: e.name.clone(),
                                                                    value: resolved_type
                                                                })
                                                            },
                                                            Collecting::Function(e) => {
                                                                Some(Attribute {
                                                                    _rtype: AttributeType::Method,
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
                                                                    _rtype: AttributeType::Method,
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
                                                            Collecting::Getter(e) => {
                                                                Some(Attribute {
                                                                    _rtype: AttributeType::Getter,
                                                                    name: e.name.clone(),
                                                                    value: e.return_type
                                                                })
                                                            }
                                                            Collecting::Setter(e) => {
                                                                if is_setter {
                                                                    Some(Attribute {
                                                                        _rtype: AttributeType::Setter,
                                                                        name: e.name.clone(),
                                                                        value: e.rtype
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
                                                },
                                                None => {
                                                    unreachable!()
                                                }
                                            }
                                        },
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::Variable(_) => todo!(),
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::Function(_) => todo!(),
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::Getter(_) => todo!(),
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::Setter(_) => todo!(),
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::ImportReference(_) => todo!(),
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::GenericItem(_) => todo!(),
                                        crate::deep_search_extensions::ProcessedDeepSearchItems::None => todo!(),
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
                            ellie_core::definite::definers::DefinerCollecting::ParentGeneric(
                                rtype,
                            ) => {
                                let rtype = find_type(rtype.rtype, page_id, parser);

                                match resolve_chain(
                                    DefinerCollecting::Generic(rtype.unwrap()),
                                    ellie_core::defs::Cursor::default(),
                                    page_id,
                                    parser,
                                    is_setter,
                                ) {
                                    Ok(e) => Ok(e),
                                    Err(_) => todo!(),
                                }
                            }
                            ellie_core::definite::definers::DefinerCollecting::Function(_) => {
                                let rtype = find_type("function".to_owned(), page_id, parser);
                                match resolve_chain(
                                    DefinerCollecting::Generic(rtype.unwrap()),
                                    ellie_core::defs::Cursor::default(),
                                    page_id,
                                    parser,
                                    is_setter,
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

                    let reference_type = match resolve_type(
                        found_reference.clone(),
                        page_id,
                        parser,
                        &mut errors,
                        Some(reference.data.reference_pos),
                    ) {
                        Some(e) => e,
                        None => return Err(errors),
                    };
                    //#[derive(Debug, EnumAsInner)]
                    //enum LastEntry {
                    //    Type(types::Types),
                    //    Null,
                    //}
                    //let mut resolved_types = LastEntry::Null;

                    let mut last_chain_attributes = (
                        reference_type.clone(),
                        resolve_chain(
                            reference_type,
                            reference.data.reference_pos,
                            page_id,
                            parser,
                            include_setter,
                        ),
                    );

                    for chain in reference.data.chain.clone() {
                        match last_chain_attributes.1.clone() {
                            Ok(e) => {
                                let attribute = e.iter().find(|a| a.name == chain.value);
                                match attribute {
                                    Some(a) => {
                                        //resolved_types = LastEntry::Type(
                                        //    generate_type_from_defining(
                                        //        a.value.clone(),
                                        //        page_id,
                                        //        parser,
                                        //    )
                                        //    .unwrap(),
                                        //);
                                        last_chain_attributes = (
                                            a.value.clone(),
                                            resolve_chain(
                                                a.value.clone(),
                                                chain.pos,
                                                page_id,
                                                parser,
                                                include_setter,
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
                                                parser.find_page(page_id).unwrap().path.clone(),
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
                        //panic!("{:#?}", resolved_types);
                        Ok(types::Types::Reference(types::reference::ReferenceType {
                            reference: Box::new(found_reference),
                            reference_pos: reference.data.reference_pos,
                            chain: reference
                                .data
                                .chain
                                .iter()
                                .map(|chain| types::reference::Chain {
                                    pos: chain.pos,
                                    value: chain.value.clone(),
                                })
                                .collect::<Vec<_>>(),
                            pos: reference.data.pos,
                        }))
                    } else {
                        Err(errors)
                    }
                }
                Err(e) => Err(e),
            }
        }
        Processors::BraceReference(brace_reference) => {
            let index = process(
                *brace_reference.data.value,
                parser,
                page_id,
                ignore_hash,
                false,
            );
            match index {
                Ok(index) => {
                    //if matches!(index, types::Types::Integer(x) if matches!()) {}

                    let index_type = match resolve_type(
                        index.clone(),
                        page_id,
                        parser,
                        &mut errors,
                        Some(brace_reference.data.brace_pos),
                    ) {
                        Some(e) => e,
                        None => {
                            return Err(errors);
                        }
                    };
                    let reference = process(
                        *brace_reference.data.reference,
                        parser,
                        page_id,
                        ignore_hash,
                        false,
                    );
                    match reference {
                        Ok(found_reference) => {
                            let reference_type = resolve_type(
                                found_reference.clone(),
                                page_id,
                                parser,
                                &mut errors,
                                Some(brace_reference.data.reference_pos),
                            );
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
                                Some(reference_type) => {
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
                                                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                                                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                                parser.find_page(page_id).unwrap().path.clone(),
                                                brace_reference.data.reference_pos
                                            ));
                                            Err(errors)
                                        }
                                    }
                                },
                                None => return Err(errors)
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        }
        Processors::FunctionCall(function_call) => {
            // let mut errors = Vec::new();
            //let index = process(
            //    function_call.data.target.clone()
            //    page_id,
            //    parser,
            //    &mut errors,
            //    Some(function_call.data.target_pos),
            //);
            //let target_resolution = resolve_deep_type(parser, page_id, function_call.data.target.clone().to_definite(), &mut errors);
            //panic!("Target resolution: {:#?} - {:#?}", target_resolution, function_call.data.target.to_definite());

            let target = process(
                *function_call.data.target.clone(),
                parser,
                page_id,
                ignore_hash,
                false,
            );
            match target {
                Ok(_) => {
                    let resolved = resolve_type(
                        function_call.data.target.clone().to_definite(),
                        page_id,
                        parser,
                        &mut errors,
                        Some(function_call.data.target_pos),
                    );
                    match resolved {
                        Some(index) => match &index {
                            DefinerCollecting::Function(function) => {
                                let used_params = function_call
                                    .data
                                    .parameters
                                    .iter()
                                    .filter_map(|param| {
                                        match process(
                                            param.value.clone(),
                                            parser,
                                            page_id,
                                            ignore_hash,
                                            false,
                                        ) {
                                            Ok(resolved) => {
                                                let found = resolve_type(
                                                    param.value.clone().to_definite(),
                                                    page_id,
                                                    parser,
                                                    &mut errors,
                                                    Some(function_call.data.target_pos),
                                                );
                                                if errors.is_empty() {
                                                    Some((resolved, found.unwrap(), param.pos))
                                                } else {
                                                    None
                                                }
                                            }
                                            Err(e) => {
                                                errors.extend(e);
                                                None
                                            }
                                        }
                                    })
                                    .collect::<Vec<_>>();

                                if function.params.len() != used_params.len() && errors.len() == 0 {
                                    errors.push(
                                        error::error_list::ERROR_S7.clone().build_with_path(
                                            vec![
                                                error::ErrorBuildField::new(
                                                    "name",
                                                    &(index.clone().to_string()),
                                                ),
                                                error::ErrorBuildField::new(
                                                    "token",
                                                    &function.params.len().to_string(),
                                                ),
                                                error::ErrorBuildField::new(
                                                    "token2",
                                                    &used_params.len().to_string(),
                                                ),
                                            ],
                                            alloc::format!(
                                                "{}:{}:{}",
                                                file!().to_owned(),
                                                line!(),
                                                column!()
                                            ),
                                            parser.find_page(page_id).unwrap().path.clone(),
                                            function_call.data.target_pos,
                                        ),
                                    );
                                    return Err(errors);
                                }

                                if errors.len() == 0 {
                                    for (index, param) in function.params.iter().enumerate() {
                                        let used = used_params[index].1.clone();
                                        let pos = used_params[index].2.clone();
                                        if !param.same_as(used.clone()) {
                                            errors.push(
                                                error::error_list::ERROR_S3
                                                    .clone()
                                                    .build_with_path(
                                                        vec![
                                                            error::ErrorBuildField::new(
                                                                "token1",
                                                                &param.to_string(),
                                                            ),
                                                            error::ErrorBuildField {
                                                                key: "token2".to_string(),
                                                                value: used.to_string(),
                                                            },
                                                        ],
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
                                                        pos,
                                                    ),
                                            );
                                        }
                                    }
                                }

                                if errors.is_empty() {
                                    match process(
                                        *function_call.data.target,
                                        parser,
                                        page_id,
                                        None,
                                        false,
                                    ) {
                                        Ok(resolved) => {
                                            Ok(ellie_core::definite::types::Types::FunctionCall(
                                                ellie_core::definite::types::function_call::FunctionCall {
                                                    target: Box::new(resolved),
                                                    target_pos: ellie_core::defs::Cursor::default(),
                                                    returning: *function.returning.clone(),
                                                    params: function.params.iter().map(|_| {
                                                        ellie_core::definite::types::function_call::FunctionCallParameter {
                                                            value: types::Types::Dynamic,
                                                            pos: ellie_core::defs::Cursor::default()
                                                        }
                                                    }).collect::<Vec<_>>(),
                                                    pos: ellie_core::defs::Cursor::default(),
                                                    generic_parameters: vec![],
                                                },
                                            ))
                                        },
                                        Err(e) => {
                                            errors.extend(e);
                                            Err(errors)
                                        },
                                    }
                                } else {
                                    Err(errors)
                                }
                            }
                            _ => {
                                errors.push(error::error_list::ERROR_S25.clone().build_with_path(
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: index.to_string(),
                                    }],
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
                                    parser.find_page(page_id).unwrap().path.clone(),
                                    function_call.data.target_pos,
                                ));
                                Err(errors)
                            }
                        },
                        None => Err(errors),
                    }
                }
                Err(e) => {
                    errors.extend(e);
                    Err(errors)
                }
            }
            /*
            match index.clone() {
                Some(index) => {
                    match index {
                        DefinerCollecting::Function(function) => {
                            Ok(ellie_core::definite::types::Types::FunctionCall(
                                ellie_core::definite::types::function_call::FunctionCall {
                                    target: Box::new(function_call.data.target.to_definite()),
                                    target_pos: ellie_core::defs::Cursor::default(),
                                    returning: *function.returning,
                                    params: function.params.iter().map(|_| {
                                        ellie_core::definite::types::function_call::FunctionCallParameter {
                                            value: types::Types::Dynamic,
                                            pos: ellie_core::defs::Cursor::default()
                                        }
                                    }).collect::<Vec<_>>(),
                                    pos: ellie_core::defs::Cursor::default(),
                                    generic_parameters: vec![],
                                },
                            ))
                        },
                        _ => {
                            errors.push(error::error_list::ERROR_S25.clone().build_with_path(
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: index.to_string(),
                                }],
                                alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                                parser.find_page(page_id).unwrap().path.clone(),
                                function_call.data.target_pos,
                            ));
                            Err(errors)
                        }
                    }
                },
                None => {
                    Err(errors)
                },
            }
            */

            /*
            match index.clone() {
                types::Types::Function(d) => {
                    Ok(ellie_core::definite::types::Types::FunctionCall(
                        ellie_core::definite::types::function_call::FunctionCall {
                            target: Box::new(types::Types::Dynamic),
                            target_pos: ellie_core::defs::Cursor::default(),
                            returning: d.return_type,
                            params: d.parameters.iter().map(|param| {
                                ellie_core::definite::types::function_call::FunctionCallParameter {
                                    value: types::Types::Dynamic,
                                    pos: param.pos
                                }
                            }).collect::<Vec<_>>(),
                            pos: ellie_core::defs::Cursor::default(),
                        },
                    ))
                }
                _ => {
                    panic!("{:#?}", index);
                    let reference_type =
                        resolve_type(index.clone(), page_id, parser, &mut errors);
                    errors.push(error::error_list::ERROR_S25.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: reference_type.to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        parser.find_page(page_id).unwrap().path.clone(),
                        function_call.data.target_pos,
                    ));
                    Err(errors)
                }
            }
            */
        }
        Processors::ClassCall(class_call) => {
            let mut resolved_generics = class_call.data.resolved_generics.clone();
            let resolved_generics_defined = class_call.data.resolved_generics.len() != 0;

            match (*class_call.data.target).clone() {
                Processors::Integer(_) => {
                    errors.push(error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "int".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
                        parser.find_page(page_id).unwrap().path.clone(),
                        class_call.data.keyword_pos,
                    ));
                    Err(errors)
                }
                Processors::Byte(_) => {
                    errors.push(error::error_list::ERROR_S11.clone().build_with_path(
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: "int".to_string(),
                        }],
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                                    .filter_map(|g| {
                                        match crate::processors::definer_processor::process(
                                            g.value.clone(),
                                            parser,
                                            page_id,
                                            ignore_hash,
                                        ) {
                                            Ok(e) => {
                                                if !resolved_generics_defined {
                                                    resolved_generics.push(e);
                                                }
                                                None
                                            }
                                            Err(err) => {
                                                errors.extend(err);
                                                Some(g)
                                            }
                                        }
                                    })
                                    .collect::<Vec<_>>();
                                if e.generic_definings.len()
                                    != class_call.data.generic_parameters.len()
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
                                            alloc::format!(
                                                "{}:{}:{}",
                                                file!().to_owned(),
                                                line!(),
                                                column!()
                                            ),
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
                                                    value: g
                                                        .value
                                                        .clone()
                                                        .to_definite()
                                                        .to_string(),
                                                }],
                                                alloc::format!(
                                                    "{}:{}:{}",
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!()
                                                ),
                                                parser.find_page(page_id).unwrap().path.clone(),
                                                g.pos,
                                            ),
                                        );
                                    }
                                    Err(errors)
                                } else {
                                    let constructor = e.body.iter().find_map(|x| {
                                        match x {
                                    ellie_tokenizer::processors::items::Processors::Constructor(
                                        e,
                                    ) => Some(e),
                                    _ => None,
                                }
                                    });

                                    if constructor.is_some() {
                                        if constructor.unwrap().parameters.len()
                                            != class_call.data.parameters.len()
                                        {
                                            errors.push(
                                                error::error_list::ERROR_S7
                                                    .clone()
                                                    .build_with_path(
                                                        vec![
                                                            error::ErrorBuildField {
                                                                key: "name".to_string(),
                                                                value: e.name.clone(),
                                                            },
                                                            error::ErrorBuildField {
                                                                key: "token".to_string(),
                                                                value: constructor
                                                                    .unwrap()
                                                                    .parameters
                                                                    .len()
                                                                    .to_string(),
                                                            },
                                                            error::ErrorBuildField {
                                                                key: "token2".to_string(),
                                                                value: class_call
                                                                    .data
                                                                    .parameters
                                                                    .len()
                                                                    .to_string(),
                                                            },
                                                        ],
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
                                                        class_call.data.target_pos,
                                                    ),
                                            );
                                            return Err(errors);
                                        } else {
                                            let class_page = deep_search_hash(
                                                parser,
                                                page_id,
                                                e.hash,
                                                Vec::new(),
                                                0,
                                            );
                                            let belonging_class = class_page
                                                .found_item
                                                .as_class()
                                                .unwrap_or_else(|| {
                                                    unreachable!(
                                                        "Parser should have prevented this"
                                                    )
                                                });
                                            let constructor_elements: Vec<_> = constructor
                                            .unwrap()
                                            .parameters
                                            .iter()
                                            .enumerate()
                                            .filter_map(|(index, x)| {
                                                let attribute_search =  deep_search(
                                                    parser,
                                                    belonging_class.inner_page_id,
                                                    x.name.clone(),
                                                    None,
                                                    vec![],
                                                    0,
                                                );

                                                if attribute_search.found {
                                                    match attribute_search.found_item {
                                                        crate::deep_search_extensions::ProcessedDeepSearchItems::Variable(variable) => {
                                                            if variable.has_type {
                                                                if belonging_class.generic_definings.len() - 1  >= index && matches!(variable.rtype.clone(), DefinerCollecting::Generic(e) if e.hash == belonging_class.generic_definings[index].hash) {
                                                                    Some(
                                                                        resolved_generics[index].clone()
                                                                    )
                                                                } else {
                                                                    Some(variable.rtype)
                                                                }
                                                            } else {
                                                                let mut errors = Vec::new();
                                                                let resolved = resolve_type(variable.value, belonging_class.inner_page_id, parser, &mut errors, Some(variable.value_pos));
                                                                if !errors.is_empty() {
                                                                    panic!("Parser should have prevented this{:?}", errors);
                                                                }
                                                                resolved
                                                            }
                                                    },
                                                        crate::deep_search_extensions::ProcessedDeepSearchItems::Function(_) => todo!(),
                                                        e => unreachable!("Parser should have prevented this: {:?}", e),
                                                    }
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect();

                                            //let resolved_generics =

                                            //Ignore if length is not a match
                                            if constructor.unwrap().parameters.len()
                                                == constructor_elements.len()
                                            {
                                                for (index, element) in
                                                    constructor_elements.iter().enumerate()
                                                {
                                                    let matching_param =
                                                        &class_call.data.parameters[index];
                                                    match process(
                                                        matching_param.value.clone(),
                                                        parser,
                                                        page_id,
                                                        ignore_hash,
                                                        false,
                                                    ) {
                                                        Ok(resolved_type) => {
                                                            let comperable = parser
                                                                .compare_defining_with_type(
                                                                    element.clone(),
                                                                    resolved_type.clone(),
                                                                    belonging_class.inner_page_id,
                                                                );
                                                            match comperable {
                                                                Ok((compare, defined, given)) => {
                                                                    if !compare {
                                                                        let err = error::error_list::ERROR_S3
                                                                        .clone()
                                                                        .build_with_path(
                                                                            vec![
                                                                                error::ErrorBuildField {
                                                                                    key: "token1".to_owned(),
                                                                                    value: defined,
                                                                                },
                                                                                error::ErrorBuildField {
                                                                                    key: "token2".to_owned(),
                                                                                    value: given,
                                                                                },
                                                                            ],
                                                                            alloc::format!(
                                                                                "{}:{}:{}",
                                                                                file!().to_owned(),
                                                                                line!(),
                                                                                column!()
                                                                            ),
                                                                            parser.find_page(page_id).unwrap().path.clone(),
                                                                            matching_param.pos,
                                                                        );
                                                                        errors.push(err);
                                                                        return Err(errors);
                                                                    }
                                                                }
                                                                Err(err) => errors.extend(err),
                                                            }
                                                        }
                                                        Err(err) => errors.extend(err),
                                                    }
                                                    //let matching_param_type = resolve_type(, belonging_class.inner_page_id, parser, &mut errors);
                                                    //panic!("{:?}: {:?} - {:?}", index, matching_param, element);
                                                }
                                            }
                                        }
                                    }
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
                                        resolved_generics,
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
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
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
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
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
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
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
                                    alloc::format!(
                                        "{}:{}:{}",
                                        file!().to_owned(),
                                        line!(),
                                        column!()
                                    ),
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
                            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
                        alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
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
            }
        }
        Processors::Cloak(_) => todo!("cloak type not yet implemented"),
        Processors::Collective(_) => todo!("collective type not yet implemented"),
        Processors::AsKeyword(as_keyword) => {
            match process(
                *as_keyword.data.target,
                parser,
                page_id,
                ignore_hash.clone(),
                false,
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
            match process(
                *null_resolver.target,
                parser,
                page_id,
                ignore_hash.clone(),
                false,
            ) {
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
