use alloc::borrow::ToOwned;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{definite::types, error};
use ellie_tokenizer::processors::types::Processors;

pub fn process(
    from: Processors,
    parser: &mut super::Parser,
    page_id: u64,
) -> Result<types::Types, Vec<error::Error>> {
    let mut errors = Vec::new();
    match from.clone() {
        Processors::Variable(variable) => {
            match parser.deep_search(page_id, variable.data.value.clone(), None, Vec::new(), 0) {
                Some(e) => match e {
                    crate::parser::DeepSearchResult::Class(_) => todo!(),
                    crate::parser::DeepSearchResult::Variable(e) => {
                        Ok(types::Types::VariableType(types::variable::VariableType {
                            value: e.name,
                            reference: e.hash,
                            pos: from.get_pos(),
                        }))
                    }
                    crate::parser::DeepSearchResult::Function(_) => todo!(),
                    crate::parser::DeepSearchResult::ImportReference(_) => todo!(),
                    crate::parser::DeepSearchResult::BrokenPageGraph => todo!(),
                    crate::parser::DeepSearchResult::MixUp(_) => todo!(),
                    crate::parser::DeepSearchResult::None => todo!(),
                },
                None => {
                    errors.push(error::errorList::error_s6.clone().build(
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: variable.data.value,
                        }],
                        "ptyp_0x14".to_owned(),
                        from.get_pos(),
                    ));
                    Err(errors)
                }
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
