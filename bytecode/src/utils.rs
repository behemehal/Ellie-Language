use alloc::vec::Vec;
use ellie_core::definite::types::Types;

pub struct RawType {
    pub size: usize,
    pub data: Vec<u8>,
}

pub fn convert_to_raw_type(types: Types) -> RawType {
    match types {
        Types::Integer(x) => {
            let q = match x.value {
                ellie_core::definite::types::integer::IntegerSize::U8(x) => x as isize,
                ellie_core::definite::types::integer::IntegerSize::U16(x) => x as isize,
                ellie_core::definite::types::integer::IntegerSize::U32(x) => x as isize,
                ellie_core::definite::types::integer::IntegerSize::U64(x) => x as isize,
                ellie_core::definite::types::integer::IntegerSize::U128(x) => x as isize,
                ellie_core::definite::types::integer::IntegerSize::Usize(x) => x as isize,
                ellie_core::definite::types::integer::IntegerSize::I8(x) => x as isize,
                ellie_core::definite::types::integer::IntegerSize::I16(x) => x as isize,
                ellie_core::definite::types::integer::IntegerSize::I32(x) => x as isize,
                ellie_core::definite::types::integer::IntegerSize::I64(x) => x as isize,
                ellie_core::definite::types::integer::IntegerSize::I128(x) => x as isize,
                ellie_core::definite::types::integer::IntegerSize::Isize(x) => x as isize,
            };

            RawType {
                size: std::mem::size_of::<isize>(),
                data: q.to_le_bytes().to_vec(),
            }
        }
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
