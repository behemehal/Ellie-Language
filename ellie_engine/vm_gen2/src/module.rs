use ellie_core::bytecode::RawType;

/// A value passed to or returned from a native function.
pub enum Gen2Value {
    Raw(RawType),
    Str(String),
    Void,
}

impl From<()> for Gen2Value {
    fn from(_: ()) -> Self {
        Gen2Value::Void
    }
}

impl From<String> for Gen2Value {
    fn from(s: String) -> Self {
        Gen2Value::Str(s)
    }
}

impl From<RawType> for Gen2Value {
    fn from(r: RawType) -> Self {
        Gen2Value::Raw(r)
    }
}

pub type NativeFn = Box<dyn Fn(&[RawType], &mut crate::heap::Heap) -> Gen2Value>;

pub struct Gen2Function {
    pub name: String,
    pub hash: usize,
    pub func: NativeFn,
}

pub struct Gen2Module {
    pub name: String,
    pub functions: Vec<Gen2Function>,
}

impl Gen2Module {
    pub fn new(name: impl Into<String>) -> Self {
        Gen2Module {
            name: name.into(),
            functions: Vec::new(),
        }
    }

    pub fn register_fn(
        &mut self,
        name: impl Into<String>,
        hash: usize,
        func: impl Fn(&[RawType], &mut crate::heap::Heap) -> Gen2Value + 'static,
    ) {
        self.functions.push(Gen2Function {
            name: name.into(),
            hash,
            func: Box::new(func),
        });
    }
}

pub struct ModuleManager {
    pub modules: Vec<Gen2Module>,
}

impl ModuleManager {
    pub fn new() -> Self {
        ModuleManager {
            modules: Vec::new(),
        }
    }

    pub fn register_module(&mut self, module: Gen2Module) {
        self.modules.push(module);
    }

    pub fn call(&self, hash: usize, args: &[RawType], heap: &mut crate::heap::Heap) -> Option<Gen2Value> {
        for module in &self.modules {
            for func in &module.functions {
                if func.hash == hash {
                    return Some((func.func)(args, heap));
                }
            }
        }
        None
    }
}
