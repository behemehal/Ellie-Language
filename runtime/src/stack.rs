use alloc::{borrow::ToOwned, format, string::String, vec::Vec};

#[derive(Debug, Clone)]
pub enum StackElement {
    Type(usize),
    Generic(usize),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub id: usize,
    pub parameters: Vec<(usize, StackElement)>,
    pub return_type: StackElement,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub id: usize,
    pub rtype: StackElement,
    pub dynamic: bool,
    pub value: Option<usize>, //A heap storage id
}

#[derive(Debug, Clone)]
pub struct Class {
    pub id: usize,
    pub inner_page_id: usize,
    pub generics: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Addition {
    pub target_heap: usize,
    pub value: usize,
}

#[derive(Debug, Clone)]
pub enum StackElements {
    Function(Function),
    Class(Class),
    Variable(Variable),
    Addition(Addition),
    //Array
    //Collective
}

#[derive(Default, Debug, Clone)]
pub struct Stack {
    pub id: usize,
    pub elements: Vec<StackElements>,
}

impl Stack {
    pub fn new(id: usize) -> Self {
        Stack {
            id,
            ..Default::default()
        }
    }

    pub fn element_exists(self, id: usize) -> Option<StackElements> {
        self.elements.into_iter().find(|x| match x {
            StackElements::Function(e) => e.id == id,
            StackElements::Class(e) => e.id == id,
            StackElements::Variable(e) => e.id == id,
            _ => false,
        })
    }

    pub fn register_function(
        &mut self,
        parameters: Vec<(usize, StackElement)>,
        return_type: StackElement,
    ) -> usize {
        let id = self.elements.len() + 1;
        self.elements.push(StackElements::Function(Function {
            id,
            parameters,
            return_type,
        }));
        id
    }

    pub fn register_variable(
        &mut self,
        rtype: StackElement,
        value: Option<usize>,
        dynamic: bool,
    ) -> usize {
        let id = self.elements.len() + 1;
        self.elements.push(StackElements::Variable(Variable {
            id,
            rtype,
            dynamic,
            value,
        }));
        id
    }

    pub fn register_class(&mut self, inner_page_id: usize, generics: Vec<usize>) -> usize {
        let id = self.elements.len() + 1;
        self.elements.push(StackElements::Class(Class {
            id,
            inner_page_id,
            generics,
        }));
        id
    }

    pub fn dump(self) -> String {
        let mut lines: Vec<String> = Vec::with_capacity(self.elements.len());
        for element in self.elements {
            match element {
                StackElements::Function(i) => lines.push(format!(
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
                )),
                StackElements::Class(i) => lines.push(format!(
                    "{:#04x} : {:#04x} : {}",
                    i.id,
                    i.inner_page_id,
                    i.generics
                        .into_iter()
                        .map(|x| format!("{:#04x}", x))
                        .collect::<Vec<_>>()
                        .join(", "),
                )),
                StackElements::Variable(i) => lines.push(format!(
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
                )),
                StackElements::Addition(i) => {
                    lines.push(format!("@{:#04x} =+ {:#04x}", i.target_heap, i.value))
                }
            }
        }
        lines.join("\n\r")
    }
}
