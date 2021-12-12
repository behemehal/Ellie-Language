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
                    crate::parser::DeepSearchItems::Class(_) => todo!(),
                    crate::parser::DeepSearchItems::Variable(e) => {
                        Ok(types::Types::VariableType(types::variable::VariableType {
                            value: e.name,
                            reference: e.hash,
                            pos: from.get_pos(),
                        }))
                    }
                    crate::parser::DeepSearchItems::Function(_) => todo!(),
                    crate::parser::DeepSearchItems::ImportReference(_) => todo!(),
                    crate::parser::DeepSearchItems::BrokenPageGraph => todo!(),
                    crate::parser::DeepSearchItems::MixUp(_) => todo!(),
                    crate::parser::DeepSearchItems::None => todo!(),
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
        Processors::Negative(_) => todo!(),
        Processors::Array(_) => todo!(),
        Processors::Operator(_) => todo!(),
        Processors::Reference(_) => todo!(),
        Processors::BraceReference(_) => todo!(),
        Processors::FunctionCall(_) => todo!(),
        Processors::ClassCall(_) => todo!(),
        Processors::Cloak(_) => todo!(),
        Processors::Collective(_) => todo!(),
        _ => Ok(from.to_definite()),
    }
}
