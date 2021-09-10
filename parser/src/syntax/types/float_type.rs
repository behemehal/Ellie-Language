use crate::alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::{String, ToString};
use core::any::Any;
use core::any::TypeId;
use ellie_core::definite;
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FloatTypes {
    F32,
    F64,
}

impl Default for FloatTypes {
    fn default() -> Self {
        FloatTypes::F32
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, EnumAsInner, Deserialize)]
pub enum FloatSize {
    F32(f32),
    F64(f64),
}

impl FloatSize {
    pub fn get_type(&self) -> String {
        let mut q: String = format!("{:?}", self);
        let bracket_offset = q.find('(').unwrap_or_else(|| q.len());
        q.replace_range(bracket_offset.., "");
        q
    }
}

impl Default for FloatSize {
    fn default() -> Self {
        FloatSize::F64(0.0)
    }
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct FloatType {
    pub value: FloatSize,
    pub rtype: FloatTypes,
    pub raw: String,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct FloatTypeCollector {
    pub data: FloatType,
    pub base: String,
    pub point: String,
    pub at_point: bool,
    pub complete: bool,
}

impl FloatTypeCollector {
    pub fn to_definite(self) -> definite::types::float::FloatType {
        definite::types::float::FloatType {
            value: match self.data.value {
                FloatSize::F32(e) => definite::types::float::FloatSize::F32(e),
                FloatSize::F64(e) => definite::types::float::FloatSize::F64(e),
            },
            rtype: match self.data.rtype {
                FloatTypes::F32 => definite::types::float::FloatTypes::F32,
                FloatTypes::F64 => definite::types::float::FloatTypes::F64,
            },
        }
    }

    pub fn collect(&self) -> String {
        (self.base.to_string() + &(".".to_owned())) + &self.point
    }

    pub fn build<T: Any>(raw: T) -> FloatType {
        if TypeId::of::<T>() == TypeId::of::<f32>() {
            FloatType {
                value: FloatSize::F32(*(&raw as &dyn Any).downcast_ref::<f32>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<f64>() {
            FloatType {
                value: FloatSize::F64(*(&raw as &dyn Any).downcast_ref::<f64>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else {
            FloatType {
                value: FloatSize::F32(*(&raw as &dyn Any).downcast_ref::<f32>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        }
    }
}
