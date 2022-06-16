

use crate::utils::Types;

pub struct Entry {
    pub key: usize,
    pub value: (Types, Vec<u8>),
}

pub struct Heap {
    pub data: Vec<Entry>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap { data: Vec::new() }
    }

    pub fn get(&self, key: &usize) -> Option<&(Types, Vec<u8>)> {
        for entry in self.data.iter() {
            if &entry.key == key {
                return Some(&entry.value);
            }
        }
        None
    }

    pub fn set(&mut self, key: &usize, value: (Types, Vec<u8>)) {
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

    pub fn push(&mut self, value: (Types, Vec<u8>)) {
        self.data.push(Entry {
            key: self.data.len(),
            value,
        });
    }

    pub fn dump(&mut self) -> String {
        let mut result = String::new();
        for entry in &self.data {
            result.push_str(&format!(
                "{:02x} : {} = {:?}\n",
                entry.key,
                entry.value.0.display(),
                entry.value.1
            ));
        }
        result
    }
}
