use crate::alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
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

    pub fn get_val(&self) -> String {
        let mut q: String = format!("{:?}", self);
        let bracket_open_offset = q.find('(').unwrap_or_else(|| q.len());
        q.replace_range(..bracket_open_offset + 1, "");
        q.replace(")", "")
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

    pub fn from_definite(self, from: definite::types::float::FloatType) -> Self {
        let value = match from.value {
            definite::types::float::FloatSize::F32(e) => FloatSize::F32(e),
            definite::types::float::FloatSize::F64(e) => FloatSize::F64(e),
        };
        let raw = value.get_type().to_string();
        let partitions = raw.split(".").collect::<Vec<_>>();

        FloatTypeCollector {
            data: FloatType {
                value,
                rtype: match from.rtype {
                    definite::types::float::FloatTypes::F32 => FloatTypes::F32,
                    definite::types::float::FloatTypes::F64 => FloatTypes::F64,
                },
                raw: raw.clone(),
            },
            base: partitions[0].to_owned(),
            point: partitions[1].to_owned(),
            at_point: true,
            complete: true,
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
