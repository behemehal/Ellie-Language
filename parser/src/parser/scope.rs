use crate::alloc::boxed::Box;
use crate::alloc::string::{String, ToString};
use crate::parser;
use crate::syntax::{class, condition, constructor, function, import, ret, variable};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    scope_name: String,
    scope_data: parser::Parser,
    inner_scope: Option<Box<Scope>>,
    scope_type: ScopeTypes,
}

impl Default for Scope {
    fn default() -> Self {
        Scope {
            scope_name: String::new(),
            scope_type: ScopeTypes::default(),
            scope_data: parser::Parser::default(),
            inner_scope: None
        }
    }
}

impl Scope {
    //pub fn require_variable(name: String) -> variable::VariableCollector {
    //
    //}

    pub fn resolve_item(name: String) {
        
    }

    pub fn mount_scope(&mut self, scope: Scope) {
        self.inner_scope = Some(Box::new(scope));
    }
}
