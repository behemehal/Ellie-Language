use alloc::{collections::BTreeMap, format};
use ellie_core::raw_type::RawType;

#[derive(Clone, Debug)]
pub struct Entry {
    pub key: usize,
    pub value: RawType,
}

#[derive(Clone)]
pub struct Heap {
    pub data: BTreeMap<usize, RawType>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap {
            data: BTreeMap::new(),
        }
    }

    pub fn get(&self, key: &usize) -> Option<&RawType> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &usize) -> Option<&mut RawType> {
        self.data.get_mut(key)
    }

    pub fn set(&mut self, key: &usize, value: RawType) {
        self.data.insert(*key, value);
    }

    pub fn push(&mut self, value: RawType) {
        let key = self.data.len();
        self.data.insert(key, value);
    }

    pub fn dump(&self) -> String {
        let mut result = String::new();
        for (key, value) in &self.data {
            result.push_str(&format!(
                "{:02x} : {} = {:?} =! {:?}\n",
                key,
                value.type_id,
                match value.type_id.id {
                    1 => {
                        isize::from_le_bytes(value.data.clone().try_into().unwrap()).to_string()
                    }
                    2 => {
                        f64::from_le_bytes(value.data.clone().try_into().unwrap()).to_string()
                    }
                    3 => {
                        f32::from_le_bytes(value.data.clone().try_into().unwrap()).to_string()
                    }
                    4 => {
                        u8::from_le_bytes(value.data.clone().try_into().unwrap()).to_string()
                    }
                    5 => {
                        (value.data[0] == 1).to_string()
                    }
                    6 => {
                        String::from_utf8(value.data.clone()).unwrap()
                    }
                    7 => {
                        todo!("Todo")
                    }
                    8 => String::from("void"),
                    9 => String::from("arr"),
                    10 => String::from("null"),
                    11 => String::from("class"),
                    _ => unreachable!("Wrong typeid"),
                },
                value.data,
            ));
        }
        result
    }
}
