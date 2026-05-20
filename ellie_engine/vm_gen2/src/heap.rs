use ellie_core::bytecode::{RawType, TypeId};

pub enum HeapObject {
    Str(String),
    ClassInst(Vec<RawType>),
}

pub struct Heap {
    pub objects: Vec<HeapObject>,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            objects: Vec::new(),
        }
    }

    fn zero_raw() -> RawType {
        RawType {
            type_id: TypeId::Int,
            size: 0,
            data: [0; 8],
        }
    }

    pub fn alloc_string(&mut self, s: String) -> usize {
        self.objects.push(HeapObject::Str(s));
        self.objects.len() - 1
    }

    pub fn alloc_class_inst(&mut self, n_fields: usize) -> usize {
        self.objects.push(HeapObject::ClassInst(vec![Self::zero_raw(); n_fields]));
        self.objects.len() - 1
    }

    pub fn get_string(&self, idx: usize) -> Option<&str> {
        match self.objects.get(idx) {
            Some(HeapObject::Str(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn get_string_mut(&mut self, idx: usize) -> Option<&mut String> {
        match self.objects.get_mut(idx) {
            Some(HeapObject::Str(s)) => Some(s),
            _ => None,
        }
    }

    pub fn get_field(&self, idx: usize, field_idx: usize) -> Option<RawType> {
        match self.objects.get(idx) {
            Some(HeapObject::ClassInst(fields)) => fields.get(field_idx).copied(),
            _ => None,
        }
    }

    pub fn set_field(&mut self, idx: usize, field_idx: usize, val: RawType) {
        if let Some(HeapObject::ClassInst(fields)) = self.objects.get_mut(idx) {
            if field_idx < fields.len() {
                fields[field_idx] = val;
            }
        }
    }
}
