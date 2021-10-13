use ellie_core::definite::types::float;

#[repr(C)]
pub enum FloatTypes {
    F32,
    F64,
}

#[repr(C)]
pub enum FloatSize {
    F32(f32),
    F64(f64),
}

#[repr(C)]
pub struct FloatType {
    pub value: FloatSize,
    pub rtype: FloatTypes,
}

pub fn build_float_from(target: float::FloatType) -> FloatType {
    FloatType {
        value: match target.value {
            float::FloatSize::F32(e) => FloatSize::F32(e),
            float::FloatSize::F64(e) => FloatSize::F64(e),
        },
        rtype: match target.rtype {
            float::FloatTypes::F32 => FloatTypes::F32,
            float::FloatTypes::F64 => FloatTypes::F64,
        },
    }
}
