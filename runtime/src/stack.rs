use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
use ellie_core::definite;

pub enum StackElement {
    Type(usize),
    Generic(usize),
}

pub struct Function {
    pub id: usize,
    pub parameters: Vec<(usize, StackElement)>,
    pub return_type: StackElement,
}

pub struct Variable {
    pub id: usize,
    pub rtype: StackElement,
    pub dynamic: bool,
    pub value: Option<usize>, //A heap storage id
}

pub struct Class {
    pub id: usize,
    pub inner_stack_id: usize,
    pub generics: Vec<usize>,
}

#[derive(Default)]
pub struct Stack {
    pub id: usize,
    pub functions: Vec<Function>,
    pub classes: Vec<Class>,
    pub variables: Vec<Variable>,
}

pub enum StackElements {
    Function(Function),
    Class(Class),
    Variable(Variable),
    //Array
    //Collective
}

impl Stack {
    pub fn new(id: usize) -> Self {
        Stack {
            id,
            ..Default::default()
        }
    }

    pub fn element_exists(self, id: usize) -> Option<StackElements> {
        let mut elements: Vec<StackElements> = Vec::new();
        elements.extend(
            self.functions
                .into_iter()
                .map(|x| StackElements::Function(x))
                .collect::<Vec<_>>(),
        );
        elements.extend(
            self.classes
                .into_iter()
                .map(|x| StackElements::Class(x))
                .collect::<Vec<_>>(),
        );
        elements.extend(
            self.variables
                .into_iter()
                .map(|x| StackElements::Variable(x))
                .collect::<Vec<_>>(),
        );
        elements.into_iter().find(|x| match x {
            StackElements::Function(e) => e.id == id,
            StackElements::Class(e) => e.id == id,
            StackElements::Variable(e) => e.id == id,
        })
    }

    pub fn register_function(
        &mut self,
        parameters: Vec<(usize, StackElement)>,
        return_type: StackElement,
    ) -> usize {
        let id = self.functions.len() + self.classes.len() + self.variables.len() + 1;
        self.functions.push(Function {
            id,
            parameters,
            return_type,
        });
        id
    }

    pub fn register_variable(
        &mut self,
        rtype: StackElement,
        value: Option<usize>,
        dynamic: bool,
    ) -> usize {
        let id = self.functions.len() + self.classes.len() + self.variables.len() + 1;
        self.variables.push(Variable {
            id,
            rtype,
            dynamic,
            value,
        });
        id
    }

    pub fn register_class(&mut self, inner_stack_id: usize, generics: Vec<usize>) -> usize {
        let id = self.functions.len() + self.classes.len() + self.variables.len() + 1;
        self.classes.push(Class {
            id,
            inner_stack_id,
            generics,
        });
        id
    }

    pub fn dump(self) -> String {
        let mut lines: Vec<String> =
            Vec::with_capacity(self.classes.len() + self.classes.len() + self.variables.len());
        for i in self.classes {
            lines.push(format!(
                "{:#04x} : {:#04x} : {}",
                i.id,
                i.inner_stack_id,
                i.generics
                    .into_iter()
                    .map(|x| format!("{:#04x}", x))
                    .collect::<Vec<_>>()
                    .join(", "),
            ))
        }

        for i in self.variables {
            lines.push(format!(
                "{} {:#04x} : {}{}",
                if i.dynamic { "dyn " } else { "" },
                i.id,
                match i.rtype {
                    StackElement::Type(e) => format!("t({:#04x})", e),
                    StackElement::Generic(e) => format!("g({:#04x})", e),
                },
                if let Some(e) = i.value {
                    format!(" : {:#04x}", e)
                } else {
                    "".to_owned()
                },
            ))
        }

        for i in self.functions {
            lines.push(format!(
                "{:#04x} : <{}> : {}",
                i.id,
                i.parameters
                    .into_iter()
                    .map(|x| format!(
                        "({:#04x}, {})",
                        x.0,
                        match x.1 {
                            StackElement::Type(e) => format!("t({:#04x})", e),
                            StackElement::Generic(e) => format!("g({:#04x})", e),
                        },
                    ))
                    .collect::<Vec<_>>()
                    .join(", "),
                match i.return_type {
                    StackElement::Type(e) => format!("t({:#04x})", e),
                    StackElement::Generic(e) => format!("g({:#04x})", e),
                },
            ))
        }
        lines.join("\n\r")
    }
}
