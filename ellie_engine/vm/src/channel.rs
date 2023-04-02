use crate::utils::ThreadInfo;
use alloc::{boxed::Box, string::String, vec::Vec};
use ellie_core::{
    defs::{VmNativeAnswer, VmNativeCallParameters},
    raw_type::RawType,
};

pub enum ModuleElements {
    Function(FunctionElement),
}

impl ModuleElements {
    pub fn get_name(&self) -> String {
        match self {
            ModuleElements::Function(e) => e.name.clone(),
        }
    }

    pub fn get_hash(&self) -> usize {
        match self {
            ModuleElements::Function(e) => e.hash,
        }
    }
}

pub struct EllieModule {
    pub module_name: String,
    pub module_hash: usize,
    pub elements: Vec<ModuleElements>,
}

pub struct FunctionElement {
    pub name: String,
    pub hash: usize,
    /// This is a callback function that will be called when the function is called
    /// (ThreadInfo, params)
    pub callback: Box<dyn FnMut(ThreadInfo, Vec<VmNativeCallParameters>) -> VmNativeAnswer>,
}

impl EllieModule {
    pub fn new(module_name: String, module_hash: usize) -> Self {
        EllieModule {
            module_name,
            module_hash,
            elements: Vec::new(),
        }
    }

    pub fn register_element(&mut self, element: ModuleElements) {
        self.elements.push(element);
    }

    pub fn get_emiter(&mut self, hash: usize) -> Option<&mut ModuleElements> {
        for element in self.elements.iter_mut() {
            match element {
                ModuleElements::Function(e) => {
                    if e.hash == hash {
                        return Some(element);
                    }
                }
            }
        }
        None
    }
}

pub struct ModuleManager {
    pub modules: Vec<EllieModule>,
}

impl ModuleManager {
    pub fn new() -> Self {
        ModuleManager {
            modules: Vec::new(),
        }
    }

    pub fn find_module_by_item_hash(&mut self, hash: usize) -> Option<&mut EllieModule> {
        for module in self.modules.iter_mut() {
            let mut found = false;
            for element in module.elements.iter() {
                if element.get_hash() == hash {
                    found = true;
                    break;
                }
            }
            if found {
                return Some(module);
            }
        }
        None
    }

    pub fn register_module(&mut self, module: EllieModule) {
        self.modules.push(module);
    }

    pub fn get_module(&mut self, module_hash: usize) -> Option<&mut EllieModule> {
        for module in self.modules.iter_mut() {
            if module.module_hash == module_hash {
                return Some(module);
            }
        }
        None
    }

    pub fn get_module_mut(&mut self, module_hash: usize) -> Option<&mut EllieModule> {
        for module in self.modules.iter_mut() {
            if module.module_hash == module_hash {
                return Some(module);
            }
        }
        None
    }
}
