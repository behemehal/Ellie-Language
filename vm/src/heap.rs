use ellie_core::raw_type::RawType;

use crate::utils::Colors;

#[derive(Clone)]
pub struct Entry {
    pub key: usize,
    pub value: RawType,
}

#[derive(Clone)]
pub struct Heap {
    pub data: Vec<Entry>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap { data: Vec::new() }
    }

    pub fn get(&self, key: &usize) -> Option<&RawType> {
        for entry in self.data.iter() {
            if &entry.key == key {
                return Some(&entry.value);
            }
        }
        None
    }

    pub fn set(&mut self, key: &usize, value: RawType) {
        for entry in self.data.iter_mut() {
            if &entry.key == key {
                entry.value = value;
                return;
            }
        }
        self.data.push(Entry {
            key: key.clone(),
            value,
        });
    }

    pub fn push(&mut self, value: RawType) {
        self.data.push(Entry {
            key: self.data.len(),
            value,
        });
    }

    pub fn dump(&self) -> String {
        let mut result = String::new();
        for entry in &self.data {
            result.push_str(&format!(
                "{:02x} : {} = {}{:?}{} =! {}{:?}{}\n",
                entry.key,
                entry.value.type_id,
                Colors::Cyan,
                match entry.value.type_id.id {
                    1 => {
                        isize::from_le_bytes(entry.value.data.clone().try_into().unwrap())
                            .to_string()
                    }
                    2 => {
                        f64::from_le_bytes(entry.value.data.clone().try_into().unwrap()).to_string()
                    }
                    3 => {
                        f32::from_le_bytes(entry.value.data.clone().try_into().unwrap()).to_string()
                    }
                    4 => {
                        u8::from_le_bytes(entry.value.data.clone().try_into().unwrap()).to_string()
                    }
                    5 => {
                        (entry.value.data[0] == 1).to_string()
                    }
                    6 => {
                        String::from_utf8(entry.value.data.clone()).unwrap()
                    }
                    7 => {
                        todo!("Todo")
                    }
                    8 => String::from("void"),
                    9 => String::from("arr"),
                    10 => String::from("null"),
                    _ => unreachable!("Wrong typeid"),
                },
                Colors::Reset,
                Colors::Red,
                entry.value.data,
                Colors::Reset,
            ));
        }
        result
    }
}
