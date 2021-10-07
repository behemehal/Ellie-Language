use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
    vec::Vec,
};
use enum_as_inner::EnumAsInner;

#[derive(Debug, Clone, PartialEq)]
pub enum StackElement {
    Type((usize, usize)),
    Generic((usize, usize)),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub id: usize,
    pub inner_page_id: usize,
    pub parameters: Vec<(StackElement, usize)>,
    pub return_type: StackElement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeFunction {
    pub id: usize,
    pub parameters: Vec<(StackElement, usize)>,
    pub return_type: StackElement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub id: usize,
    pub rtype: StackElement,
    pub dynamic: bool,
    pub value: Option<usize>, //A heap storage id
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub id: usize,
    pub inner_page_id: usize,
    pub generics: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Addition {
    pub target_heap: usize,
    pub value: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub id: usize,
    pub type_id: StackElement,
    pub heap_id: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Generic {
    pub id: usize,
    pub header_id: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bridge {
    //Bridge between pages
    pub page_id: usize,
    pub targets: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ret {
    pub target_heap: usize,
}

#[derive(Debug, Clone, EnumAsInner, PartialEq)]
pub enum StackElements {
    Function(Function),
    NativeFunction(NativeFunction),
    Class(Class),
    Variable(Variable),
    Addition(Addition),
    Parameter(Parameter),
    Generic(Generic),
    Bridge(Bridge),
    Ret(Ret),
    None, //Array
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
            StackElements::Parameter(e) => e.id == id,
            StackElements::Generic(e) => e.id == id,
            StackElements::Bridge(e) => e.page_id == id,
            _ => false,
        })
    }

    pub fn register_ret(&mut self, target_heap: usize) -> usize {
        let id = self.elements.len();
        self.elements.push(StackElements::Ret(Ret { target_heap }));
        id
    }

    pub fn register_function(
        &mut self,
        parameters: Vec<(StackElement, usize)>,
        inner_page_id: usize,
        return_type: StackElement,
    ) -> usize {
        let id = self.elements.len();
        self.elements.push(StackElements::Function(Function {
            id,
            parameters,
            inner_page_id,
            return_type,
        }));
        id
    }

    pub fn register_native_function(
        &mut self,
        parameters: Vec<(StackElement, usize)>,
        return_type: StackElement,
    ) -> usize {
        let id = self.elements.len();
        self.elements
            .push(StackElements::NativeFunction(NativeFunction {
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
        let id = self.elements.len();
        self.elements.push(StackElements::Variable(Variable {
            id,
            rtype,
            dynamic,
            value,
        }));
        id
    }

    pub fn register_bridge(&mut self, page: usize, initial_reference: Option<usize>) -> usize {
        let id = self.elements.len();
        self.elements.push(StackElements::Bridge(Bridge {
            page_id: page,
            targets: if initial_reference.is_none() {
                Vec::new()
            } else {
                alloc::vec![initial_reference.unwrap()]
            },
        }));
        id
    }

    pub fn register_bridge_reference(&mut self, bridge_id: usize, reference: usize) -> usize {
        let mut found = false;
        for element in 0..self.elements.len() {
            let element = self.elements.get_mut(element).unwrap();
            if let StackElements::Bridge(bridge) = element {
                if bridge.page_id == bridge_id {
                    if !bridge.targets.contains(&reference) {
                        bridge.targets.push(reference);
                    }
                    found = true;
                    break;
                }
            }
        }

        if !found {
            self.elements.push(StackElements::Bridge(Bridge {
                page_id: bridge_id,
                targets: alloc::vec![reference],
            }));
        }
        0
    }

    pub fn register_parameter(&mut self, rtype: StackElement) -> usize {
        let id = self.elements.len();
        self.elements.push(StackElements::Parameter(Parameter {
            id,
            type_id: rtype,
            heap_id: None,
        }));
        id
    }

    pub fn register_generic(&mut self, header_id: usize) -> usize {
        let id = self.elements.len();
        self.elements
            .push(StackElements::Generic(Generic { id, header_id }));
        id
    }

    pub fn register_class(&mut self, inner_page_id: usize, generics: Vec<usize>) -> usize {
        let id = self.elements.len();
        self.elements.push(StackElements::Class(Class {
            id,
            inner_page_id,
            generics,
        }));
        id
    }

    pub fn dump(self) -> String {
        let mut lines: Vec<String> = Vec::with_capacity(self.elements.len());
        for element in self.elements.clone() {
            match element {
                StackElements::Function(i) => lines.push(format!(
                    "\t\t0 = {:#04x} : <{}> : {} > {}",
                    i.id,
                    i.parameters
                        .into_iter()
                        .map(|x| format!(
                            "({:#04x}, {})",
                            x.1,
                            match x.0 {
                                StackElement::Type(e) => format!(
                                    "t({:#04x}, {})",
                                    e.0,
                                    if self.id == e.1 {
                                        "*".to_string()
                                    } else {
                                        format!("{:#04x}", e.1)
                                    }
                                ),
                                StackElement::Generic(e) => format!(
                                    "g({:#04x}, {})",
                                    e.0,
                                    if self.id == e.1 {
                                        "*".to_string()
                                    } else {
                                        format!("{:#04x}", e.1)
                                    }
                                ),
                            },
                        ))
                        .collect::<Vec<_>>()
                        .join(", "),
                    match i.return_type {
                        StackElement::Type(e) => format!(
                            "t({:#04x}, {})",
                            e.0,
                            if self.id == e.1 {
                                "*".to_string()
                            } else {
                                format!("{:#04x}", e.1)
                            }
                        ),
                        StackElement::Generic(e) => format!(
                            "g({:#04x}, {})",
                            e.0,
                            if self.id == e.1 {
                                "*".to_string()
                            } else {
                                format!("{:#04x}", e.1)
                            }
                        ),
                    },
                    i.inner_page_id
                )),
                StackElements::Class(i) => lines.push(format!(
                    "\t\t1 = {:#04x} : {:#04x} : {}",
                    i.id,
                    i.inner_page_id,
                    if i.generics.len() == 0 {
                        "*".to_owned()
                    } else {
                        i.generics
                            .into_iter()
                            .map(|x| format!("{:#04x}", x))
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                )),
                StackElements::Variable(i) => lines.push(format!(
                    "\t\t2 = {:#04x} : {}{}",
                    i.id,
                    match i.rtype {
                        StackElement::Type(e) => format!(
                            "t({:#04x}, {})",
                            e.0,
                            if self.id == e.1 {
                                "*".to_string()
                            } else {
                                format!("{:#04x}", e.1)
                            }
                        ),
                        StackElement::Generic(e) => format!(
                            "g({:#04x}, {})",
                            e.0,
                            if self.id == e.1 {
                                "*".to_string()
                            } else {
                                format!("{:#04x}", e.1)
                            }
                        ),
                    },
                    if let Some(e) = i.value {
                        format!(" : {:#04x}", e)
                    } else {
                        "".to_owned()
                    },
                )),
                StackElements::Addition(i) => lines.push(format!(
                    "\t\t3 = {:#04x} =+ {:#04x}",
                    i.target_heap, i.value
                )),
                StackElements::Parameter(i) => lines.push(format!(
                    "\t\t4 = {:#04x} : {}{}",
                    i.id,
                    match i.type_id {
                        StackElement::Type(e) => format!(
                            "t({:#04x}, {})",
                            e.0,
                            if self.id == e.1 {
                                "*".to_string()
                            } else {
                                format!("{:#04x}", e.1)
                            }
                        ),
                        StackElement::Generic(e) => format!(
                            "g({:#04x}, {})",
                            e.0,
                            if self.id == e.1 {
                                "*".to_string()
                            } else {
                                format!("{:#04x}", e.1)
                            }
                        ),
                    },
                    if let Some(e) = i.heap_id {
                        format!(" : {:#04x}", e)
                    } else {
                        "".to_owned()
                    },
                )),
                StackElements::Bridge(i) => {
                    if i.targets.is_empty() {
                        lines.push(format!("\t\t5 = {:#04x}>?", i.page_id))
                    } else {
                        let mut targets: String = String::new();
                        for i in i.targets.into_iter().enumerate() {
                            targets += &format!("{}{:#04x}", if i.0 == 0 { "" } else { ", " }, i.1);
                        }
                        lines.push(format!("\t\t5 = {:#04x}>({})", i.page_id, targets.clone()))
                    }
                }
                StackElements::Generic(i) => {
                    lines.push(format!("\t\t6 = {:#04x} : {:#04x}", i.id, i.header_id))
                }
                StackElements::NativeFunction(i) => lines.push(format!(
                    "\t\t7 = {:#04x} : <{}> : {}",
                    i.id,
                    i.parameters
                        .into_iter()
                        .map(|x| format!(
                            "({:#04x}, {})",
                            x.1,
                            match x.0 {
                                StackElement::Type(e) => format!(
                                    "t({:#04x}, {})",
                                    e.0,
                                    if self.id == e.1 {
                                        "*".to_string()
                                    } else {
                                        format!("{:#04x}", e.1)
                                    }
                                ),
                                StackElement::Generic(e) => format!(
                                    "g({:#04x}, {})",
                                    e.0,
                                    if self.id == e.1 {
                                        "*".to_string()
                                    } else {
                                        format!("{:#04x}", e.1)
                                    }
                                ),
                            },
                        ))
                        .collect::<Vec<_>>()
                        .join(", "),
                    match i.return_type {
                        StackElement::Type(e) => format!(
                            "t({:#04x}, {})",
                            e.0,
                            if self.id == e.1 {
                                "*".to_string()
                            } else {
                                format!("{:#04x}", e.1)
                            }
                        ),
                        StackElement::Generic(e) => format!(
                            "g({:#04x}, {})",
                            e.0,
                            if self.id == e.1 {
                                "*".to_string()
                            } else {
                                format!("{:#04x}", e.1)
                            }
                        ),
                    }
                )),
                _ => (),
            }
        }

        if self.elements.is_empty() {
            lines.push("\t\tEMPTY".to_owned());
        }

        lines.join("\n\t")
    }
}
