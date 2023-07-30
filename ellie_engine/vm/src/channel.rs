use crate::utils::{ThreadInfo, VmNativeAnswer, VmNativeCallParameters};
use alloc::{boxed::Box, string::String, vec::Vec};
use ellie_core::defs::NativeCallTrace;

pub enum ModuleElements {
    Function(FunctionElement),
}

impl ModuleElements {
    pub fn get_name(&self) -> &'static str {
        match self {
            ModuleElements::Function(e) => e.name.clone(),
        }
    }

    pub fn get_hash(&self) -> Option<usize> {
        match self {
            ModuleElements::Function(e) => e.hash,
        }
    }

    pub fn set_hash(&mut self, hash: usize) {
        match self {
            ModuleElements::Function(e) => e.hash = Some(hash),
        }
    }
}

pub struct EllieModule {
    pub module_name: String,
    pub elements: Vec<ModuleElements>,
}

pub type FunctionElementCallback =
    Box<dyn FnMut(ThreadInfo, Vec<VmNativeCallParameters>) -> VmNativeAnswer + Send>;

pub struct FunctionElement {
    pub name: &'static str,
    pub hash: Option<usize>,
    /// This is a callback function that will be called when the function is called
    /// (ThreadInfo, params)
    pub callback: FunctionElementCallback,
}

impl FunctionElement {
    pub fn new(name: &'static str, callback: FunctionElementCallback) -> FunctionElement {
        FunctionElement {
            name,
            hash: None,
            callback,
        }
    }
}

impl EllieModule {
    pub fn new(module_name: String) -> Self {
        EllieModule {
            module_name,
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
                    if e.hash.is_some() && e.hash.unwrap() == hash {
                        return Some(element);
                    }
                }
            }
        }
        None
    }

    pub fn get_emiter_by_name(&mut self, name: &str) -> Option<&mut ModuleElements> {
        for element in self.elements.iter_mut() {
            match element {
                ModuleElements::Function(e) => {
                    if e.name == name {
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
                if element.get_hash() == Some(hash) {
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

    pub fn find_module_by_item_name(&mut self, name: &String) -> Option<&mut EllieModule> {
        for module in self.modules.iter_mut() {
            let mut found = false;
            for element in module.elements.iter() {
                if element.get_name() == name {
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

    pub fn get_module(&mut self, module_name: &str) -> Option<&mut EllieModule> {
        for module in self.modules.iter_mut() {
            if module.module_name == module_name {
                return Some(module);
            }
        }
        None
    }

    pub fn get_module_mut(&mut self, module_name: String) -> Option<&mut EllieModule> {
        for module in self.modules.iter_mut() {
            if module.module_name == module_name {
                return Some(module);
            }
        }
        None
    }

    /// Register function to existing module items
    /// This function gets the native call trace from program and and matches with
    /// already registered native module functions.
    /// ## Params
    /// * `trace` - [`NativeCallTrace`]
    /// ## Output
    /// * [`Option<usize>`]
    /// if the function is registered successfully it will return None,
    /// or else it will return the cause of the error
    pub fn add_native_call_trace(&mut self, trace: NativeCallTrace) -> Option<&'static str> {
        if let Some(module) = self.get_module(&trace.module_name) {
            if let Some(emiter) = module.get_emiter_by_name(&trace.function_name) {
                emiter.set_hash(trace.function_hash);
                None
            } else {
                Some("Function not found on module")
            }
        } else {
            Some("Module not found")
        }
    }
}
