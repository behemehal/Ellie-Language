use ellie_core::definite::types::Types;

pub struct RawType {
    pub size: usize,
    pub data: isize,
}

pub fn is_static_type(type_: &Types) -> bool {
    match type_ {
        Types::Integer(..) => true,
        Types::Byte(..) => true,
        Types::Decimal(..) => true,
        Types::Bool(..) => true,
        Types::String(..) => true,
        Types::Char(..) => true,
        Types::Void => true,
        _ => false,
    }
}

pub fn convert_to_raw_type(types: Types) -> RawType {
    match types {
        Types::Integer(x) => RawType {
            size: std::mem::size_of::<isize>(),
            data: x.value.to_le(),
        },
        Types::Decimal(_) => todo!(),
        Types::String(_) => RawType {
            size: std::mem::size_of::<isize>(),
            data: 1_isize.to_le(),
        },
        Types::Char(_) => todo!(),
        Types::Collective(_) => todo!(),
        Types::Reference(_) => todo!(),
        Types::BraceReference(_) => todo!(),
        Types::Operator(_) => todo!(),
        Types::Cloak(_) => todo!(),
        Types::Array(_) => todo!(),
        Types::Function(_) => todo!(),
        Types::ClassCall(_) => todo!(),
        Types::FunctionCall(_) => todo!(),
        Types::Void => RawType {
            size: std::mem::size_of::<isize>(),
            data: 0_isize.to_le(),
        },
        Types::NullResolver(_) => todo!(),
        Types::Negative(_) => todo!(),
        Types::VariableType(_) => todo!(),
        Types::AsKeyword(_) => todo!(),
        Types::Null => todo!(),
        Types::Dynamic => todo!(),
        Types::Byte(_) => todo!(),
        Types::SetterCall(_) => todo!(),
        Types::EnumData(_) => todo!(),
        Types::ClassInstance(_) => todo!(),
        Types::Bool(_) => todo!(),
    }
}
