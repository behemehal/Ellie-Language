use alloc::{
    format,
    string::{String, ToString},
};

use crate::{config::STACK_MEMORY_SIZE, raw_type::StaticRawType};

//Static memory allocation
#[derive(Clone, Copy)]
pub struct StackMemory {
    pub data: [StaticRawType; STACK_MEMORY_SIZE],
    pub len: usize,
}

impl StackMemory {
    pub fn new() -> StackMemory {
        StackMemory {
            data: [StaticRawType::from_void(); STACK_MEMORY_SIZE],
            len: 0,
        }
    }

    pub fn get(&self, key: &usize) -> Option<StaticRawType> {
        if self.data.len() <= *key {
            None
        } else {
            Some(self.data[*key])
        }
    }

    pub fn set(&mut self, key: &usize, value: StaticRawType) {
        self.data[*key] = value;
    }

    pub fn dea(&mut self, key: &usize) {
        self.data[*key] = StaticRawType::from_void();
    }

    pub fn dump(&self) -> String {
        let mut result = String::new();
        for (key, data) in self.data.iter().enumerate() {
            let value = data;
            let type_id = match value.type_id.id {
                1 => value.to_int().to_string(),
                2 => value.to_float().to_string(),
                3 => value.to_double().to_string(),
                4 => value.to_byte().to_string(),
                5 => (value.data[0] == 1).to_string(),
                6 => {
                    todo!()
                }
                7 => value.to_char().to_string(),
                8 => {
                    continue;
                }
                9 => String::from("arr"),
                10 => String::from("null"),
                11 => String::from("class"),
                12 => String::from("function"),
                13 => String::from("stack_reference"),
                14 => String::from("heap_reference"),
                15 => String::from("static_array"),
                _ => unreachable!("Wrong typeid"),
            };
            result.push_str(&format!(
                "{} : {} = {:?} =! {:?}\n",
                key, value.type_id, type_id, value.data,
            ));
        }
        result
    }
}
