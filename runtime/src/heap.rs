use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};

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
pub struct Collective {
    pub keys: Vec<usize>,   //HeapID
    pub values: Vec<usize>, //HeapTarget of value
}

#[derive(Debug, Clone)]
pub enum HeapTypes {
    Integer(HeapIntegerSize),
    Float(HeapFloatSize),
    Bool(u8),
    String(Vec<u8>),
    Char(u32),
    Collective(Collective),
    Array(Vec<usize>),
    Cloak(Vec<usize>),
    Void,
    Null, /*
          Reference,
          Operator,
          ArrowFunction,
          ConstructedClass,
          FunctionCall,
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
            fn stringify(rtype: HeapTypes) -> String {
                match rtype {
                    HeapTypes::Integer(e) => {
                        match e {
                            HeapIntegerSize::U8(e) => format!("int->U8({:#04x})", e),
                            HeapIntegerSize::U16(e) => format!("int->U16({:#04x})", e),
                            HeapIntegerSize::U32(e) => format!("int->U32({:#04x})", e),
                            HeapIntegerSize::U64(e) => format!("int->U64({:#04x})", e),
                            HeapIntegerSize::U128(e) => format!("int->U128({:#04x})", e),
                            HeapIntegerSize::Usize(e) => format!("int->Usize({:#04x})", e),
                            HeapIntegerSize::I8(e) => format!("int->I8({:#04x})", e),
                            HeapIntegerSize::I16(e) => format!("int->I16({:#04x})", e),
                            HeapIntegerSize::I32(e) => format!("int->I32({:#04x})", e),
                            HeapIntegerSize::I64(e) => format!("int->I64({:#04x})", e),
                            HeapIntegerSize::I128(e) => format!("int->I128({:#04x})", e),
                            HeapIntegerSize::Isize(e) => format!("int->Isize({:#04x})", e),
                        }
                    },
                    HeapTypes::Float(e) => {
                        match e {
                            HeapFloatSize::F32(e) => format!("float->F32({:?})", e),
                            HeapFloatSize::F64(e) => format!("float->F64({:?})", e),
                        }
                    },
                    HeapTypes::Bool(e) => format!("bool({:#04x})", e),
                    HeapTypes::String(e) => format!("str({:?})", e),
                    HeapTypes::Char(e) => format!("char({:#04x})", e),
                    HeapTypes::Collective(e) => {
                        let mut formatted = "collective{".to_string();
                        for i in 0..e.values.len() {
                            formatted += &format!(
                                "[{:#04x} -> {:#04x}]{}",
                                e.keys[i],
                                e.values[i],
                                if i == e.values.len() - 1 { "" } else { ", " }
                            );
                        }
                        formatted += "}";
                        formatted
                    }
                    HeapTypes::Array(e) => {
                        let mut formatted = "array[".to_string();
                        for i in 0..e.len() {
                            formatted += &format!(
                                "{:#04x}{}",
                                e[i],
                                if i == e.len() - 1 { "" } else { ", " }
                            );
                        }
                        formatted += "]";
                        formatted
                    }
                    HeapTypes::Cloak(e) => {
                        let mut formatted = "cloak(".to_string();
                        for i in 0..e.len() {
                            formatted += &format!(
                                "{:#04x}{}",
                                e[i],
                                if i == e.len() - 1 { "" } else { ", " }
                            );
                        }
                        formatted += ")";
                        formatted
                    }
                    HeapTypes::Void => todo!(),
                    HeapTypes::Null => "null".to_owned(),
                }
            }
            lines.push(format!(
                "\t{:#04x} : {}",
                i,
                stringify(values[i].clone())
            ));
        }

        if values.is_empty() {
            lines.push("\t\tEMPTY".to_owned());
        }

        lines.join("\n\t")
    }
}
