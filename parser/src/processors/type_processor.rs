use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, string::String};
use ellie_core::definite::Converter;
use ellie_core::{definite::types, error};
use ellie_tokenizer::processors::types::Processors;

pub fn process(
    from: Processors,
    parser: &mut super::Parser,
    page_id: u64,
    ignore_hash: Option<u64>,
) -> Result<types::Types, Vec<error::Error>> {
    let mut errors = Vec::new();
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
                    crate::parser::DeepSearchItems::Class(e) => {
                        Ok(types::Types::ClassCall(
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
                        ))
                    }
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
            for i in array_type.data.collective {
                
            }
            todo!("array type not yet implemented")
        },
        Processors::Operator(_) => todo!("operator type not yet implemented"),
        Processors::Reference(_) => todo!("reference type not yet implemented"),
        Processors::BraceReference(_) => todo!("brace_reference_type type not yet implemented"),
        Processors::FunctionCall(_) => todo!("functionCall type not yet implemented"),
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
                                                value: 0.to_string(),
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
                                        generic_parameters: class_call.data.generic_parameters.iter().map(|x| ellie_core::definite::types::class_call::ClassCallGenericParameter { value: ellie_core::definite::Converter::to_definite(x.value.clone()), pos: x.pos }).collect::<Vec<_>>(),
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
        _ => Ok(from.to_definite()),
    }
}
