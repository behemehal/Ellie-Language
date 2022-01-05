use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, string::String};
use ellie_core::{definite::types, error};
use ellie_tokenizer::processors::types::Processors;

pub fn process(
    from: Processors,
    parser: &mut super::Parser,
    page_id: u64,
    ignore_hash: Option<String>,
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
                    crate::parser::DeepSearchItems::Class(_) => todo!("class type not yet implemented"),
                    crate::parser::DeepSearchItems::Variable(e) => {
                        Ok(types::Types::VariableType(types::variable::VariableType {
                            value: e.name,
                            reference: e.hash,
                            pos: from.get_pos(),
                        }))
                    }
                    crate::parser::DeepSearchItems::Function(_) => todo!("function type not yet implemented"),
                    crate::parser::DeepSearchItems::ImportReference(_) => todo!("import reference type not yet implemented"),
                    crate::parser::DeepSearchItems::BrokenPageGraph => todo!(),
                    crate::parser::DeepSearchItems::MixUp(_) => todo!(),
                    crate::parser::DeepSearchItems::None => todo!(),
                    crate::parser::DeepSearchItems::SelfItem(_) => todo!(),
                    crate::parser::DeepSearchItems::GenericItem(_) => todo!(),
                    crate::parser::DeepSearchItems::FunctionParameter(e) => {
                        Ok(types::Types::VariableType(types::variable::VariableType {
                            value: e.name,
                            reference: "<virtual>".to_string(),
                            pos: from.get_pos(),
                        }))
                    }
                    crate::parser::DeepSearchItems::ConstructorParameter(e) => {
                        Ok(types::Types::VariableType(types::variable::VariableType {
                            value: e.name,
                            reference: "<virtual>".to_string(),
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
                    "ptyp_0x14".to_owned(),
                    parser.find_page(page_id).unwrap().path.clone(),
                    from.get_pos(),
                ));
                Err(errors)
            }
        }
        Processors::Negative(_) => todo!("negative type not yet implemented"),
        Processors::Array(_) => todo!("array type not yet implemented"),
        Processors::Operator(_) => todo!("operator type not yet implemented"),
        Processors::Reference(_) => todo!("reference type not yet implemented"),
        Processors::BraceReference(_) => todo!("brace_reference_type type not yet implemented"),
        Processors::FunctionCall(_) => todo!("functionCall type not yet implemented"),
        Processors::ClassCall(_) => todo!("classCall type not yet implemented"),
        Processors::Cloak(_) => todo!("cloak type not yet implemented"),
        Processors::Collective(_) => todo!("collective type not yet implemented"),
        Processors::AsKeyword(as_keyword) => {
            match process(*as_keyword.data.target, parser, page_id, ignore_hash.clone()) {
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
