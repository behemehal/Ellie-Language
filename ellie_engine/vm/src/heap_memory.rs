use crate::raw_type::{MutatableRawType, RawType};
use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};

#[derive(Clone)]
pub struct HeapMemory {
    pub data: BTreeMap<usize, Vec<u8>>,
}

impl HeapMemory {
    pub fn new() -> HeapMemory {
        HeapMemory {
            data: BTreeMap::new(),
        }
    }

    pub fn get_mut(&mut self, key: &usize) -> Option<MutatableRawType> {
        self.data.get_mut(key).map(|data| MutatableRawType { data })
    }

    pub fn get(&self, key: &usize) -> Option<RawType> {
        self.data.get(key).map(|data| RawType::from_bytes(data))
    }

    pub fn get_def(&self, key: &usize) -> Option<RawType> {
        match self.get(key) {
            Some(e) => {
                if e.type_id.id == 13 {
                    self.get_def(&(e.to_int() as usize))
                } else {
                    Some(e)
                }
            }
            None => None,
        }
    }

    pub fn set(&mut self, key: &usize, value: RawType) {
        self.data.insert(*key, value.to_bytes());
    }

    pub fn dea(&mut self, key: &usize) {
        self.data.remove(key);
    }

    pub fn dump(&self) -> String {
        let mut result = String::new();
        for key in &self.data {
            let value = self.get(key.0).unwrap();
            result.push_str(&format!(
                "{} : {} = {:?} =! {:?}\n",
                key.0,
                value.type_id,
                match value.type_id.id {
                    1 => {
                        isize::from_le_bytes(value.data.clone().try_into().unwrap()).to_string()
                    }
                    2 => {
                        f32::from_le_bytes(value.data.clone().try_into().unwrap()).to_string()
                    }
                    3 => {
                        f64::from_le_bytes(value.data.clone().try_into().unwrap()).to_string()
                    }
                    4 => {
                        u8::from_le_bytes(value.data.clone().try_into().unwrap()).to_string()
                    }
                    5 => {
                        (value.data[0] == 1).to_string()
                    }
                    6 => {
                        let mut new_string = String::new();
                        for i in value.data.chunks(4) {
                            let char = u32::from_le_bytes(i.try_into().unwrap());
                            new_string.push(char::from_u32(char).unwrap());
                        }
                        new_string
                    }
                    7 => value.to_char().to_string(),
                    8 => String::from("void"),
                    9 => String::from("arr"),
                    10 => String::from("null"),
                    11 => String::from("class"),
                    12 => String::from("function"),
                    13 => String::from("stack_reference"),
                    14 => String::from("heap_reference"),
                    _ => unreachable!("Wrong typeid"),
                },
                value.data,
            ));
        }
        result
    }
}
