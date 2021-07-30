use crate::alloc::string::{String, ToString};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ScopeTypes {
    File,
    System,
    Inner,
}

impl Default for ScopeTypes {
    fn default() -> Self {
        ScopeTypes::Inner
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Scope {
    pub scope_name: String,
}

impl Default for Scope {
    fn default() -> Self {
        Scope {
            scope_name: "core".to_string(),
        }
    }
}

impl Scope {
    //pub fn require_variable(name: String) -> variable::VariableCollector {
    //
    //}

    //pub fn mount_scope(&mut self, scope_data: parser::Parser, scope_type: ScopeTypes, scope_name: String) -> Scope {
    //    Scope {
    //        scope_name: self.scope_name.clone() + "/" + &scope_name,
    //        scope_type,
    //        scope_data,
    //        upper_scope: Some(Box::new(self.clone()))
    //    }
    //}
}
