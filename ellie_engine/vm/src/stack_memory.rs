use alloc::{
    format,
    string::{String, ToString},
};
use ellie_core::raw_type::StaticRawType;

use crate::config::STACK_MEMORY_SIZE;

//Static memory allocation
pub struct StackMemory {
    pub data: [StaticRawType; STACK_MEMORY_SIZE],
    pub len: usize,
}

impl StackMemory {
    pub fn new() -> StackMemory {
        StackMemory {
            //4096 * 8 = 32kb
            data: [StaticRawType::void(); STACK_MEMORY_SIZE],
            len: 0,
        }
    }

    pub fn get(&self, key: &usize) -> Option<StaticRawType> {
        if self.data.len() <= *key {
            return None;
        } else {
            return Some(self.data[*key]);
        }
        //match self.data.get(*key) {
        //    Some(e) => Some(*e),
        //    None => None,
        //}
    }

    pub fn set(&mut self, key: &usize, value: StaticRawType) {
        self.data[*key] = value;
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
                13 => String::from("reference"),
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
