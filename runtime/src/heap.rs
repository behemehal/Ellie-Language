use alloc::{collections::BTreeMap, format, string::String, vec::Vec};

/*
pub enum HeapWarning {
    MaxHeapScopeLength,
    MaxHeapScopeSize,
    MaxHeapSize,
}
*/

#[derive(Debug, Clone)]
pub enum HeapIntegerSize {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
}

#[derive(Debug, Clone)]
pub enum HeapFloatSize {
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone)]
pub enum HeapTypes {
    Integer(HeapIntegerSize),
    Float(HeapFloatSize),
    Bool(bool),
    String(&'static str),
    Null, /*
          Char(char),
          Collective,
          Reference,
          Operator,
          ArrowFunction,
          ConstructedClass,
          FunctionCall,
          Void,
          NullResolver,
          Negative,
          VariableType,
          Null
          */
}

pub struct HeapElement {
    pub rtype: BTreeMap<usize, HeapTypes>,
}

#[derive(Default, Debug, Clone)]
pub struct Heap {
    pub id: usize,
    pub values: BTreeMap<usize, HeapTypes>,
}

impl Heap {
    pub fn new() -> Self {
        Heap::default()
    }

    /*
        Insert data to heap
        returns data location
    */
    pub fn insert(&mut self, data: HeapTypes) -> usize {
        self.values.insert(self.values.len() + 1, data);
        self.values.len()
    }

    pub fn dump(self) -> String {
        let mut values: Vec<HeapTypes> = Vec::new();
        for i in self.values {
            values.push(i.1);
        }

        let mut lines: Vec<String> = Vec::with_capacity(values.len());

        for i in 0..values.len() {
            lines.push(format!("\t\t\t{:#04x} : {:?}", i, values[i]));
        }
        lines.join("\n\t")
    }
}
