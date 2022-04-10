use alloc::vec::Vec;
use ellie_core::definite::types::Types;

pub struct RawType {
    pub size: usize,
    pub data: Vec<u8>
}

pub fn convert_to_raw_type(types: Types) -> RawType {
    match types {
        Types::Integer(_) => todo!(),
        Types::Float(_) => todo!(),
        Types::Bool(_) => todo!(),
        Types::String(_) => todo!(),
        Types::Char(_) => todo!(),
        Types::Collective(_) => todo!(),
        Types::Reference(_) => todo!(),
        Types::BraceReference(_) => todo!(),
        Types::Operator(_) => todo!(),
        Types::Cloak(_) => todo!(),
        Types::Array(_) => todo!(),
        Types::Vector(_) => todo!(),
        Types::Function(_) => todo!(),
        Types::ClassCall(_) => todo!(),
        Types::FunctionCall(_) => todo!(),
        Types::Void => todo!(),
        Types::NullResolver(_) => todo!(),
        Types::Negative(_) => todo!(),
        Types::VariableType(_) => todo!(),
        Types::AsKeyword(_) => todo!(),
        Types::Null => todo!(),
        Types::Dynamic => todo!(),
    }
}