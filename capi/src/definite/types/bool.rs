use ellie_core::definite::types::bool;

#[repr(C)]
pub struct BoolType {
    pub value: bool,
}

pub fn build_bool_from(target: bool::BoolType) -> BoolType {
    BoolType {
        value: target.value,
    }
}
