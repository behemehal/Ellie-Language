use core::panic;

use enum_as_inner::EnumAsInner;

use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite;

use crate::heap;
use crate::stack;

#[derive(Debug, Clone)]
pub enum NotifierMessageType {
    StackIdFault,
    HeapIdFault,
    CommandFault,
    StackPaused,
    StackResume,
    StackCrash,
    StackComplete,
    ThreadStopped,
    StackStep,
}

#[derive(Debug, Clone)]
pub struct NotifierMessage {
    pub stack_id: u64,
    pub stack_step: usize,
    pub message_type: NotifierMessageType,
}

pub enum ThreadMessage {
    HANG,
    CONTINUE,
    HALT,
}

pub trait NotifierFn {
    fn call(&mut self, args: NotifierMessage);
}

impl<T> NotifierFn for T
where
    T: FnMut(NotifierMessage) + Sized,
{
    fn call(&mut self, args: NotifierMessage) {
        self(args)
    }
}

pub trait FrameFn {
    fn call(&mut self, args: (u64, usize)) -> ThreadMessage;
}

impl<T> FrameFn for T
where
    T: FnMut((u64, usize)) -> ThreadMessage + Sized,
{
    fn call(&mut self, args: (u64, usize)) -> ThreadMessage {
        self(args)
    }
}

pub struct ThreadController {
    pub notifier: Box<dyn NotifierFn>,
    pub require_frame: Box<dyn FrameFn>,
    pub wait_next_frame: bool,
}

impl ThreadController {
    pub fn new() -> Self {
        ThreadController {
            notifier: Box::new(|_| {}),
            require_frame: Box::new(|_| ThreadMessage::CONTINUE),
            wait_next_frame: true,
        }
    }
}

#[derive(Debug, Clone, EnumAsInner)]
pub enum HeapType {
    SolidHeap(heap::Heap),
    SharedHeap((usize, u64)), //THREAD_ID, PAGE_ID
}

#[derive(Debug, Clone)]
pub struct Page {
    pub page_id: u64,
    pub headers: BTreeMap<usize, String>,
    pub stack: stack::Stack,
    pub step: usize,
}

impl Page {
    pub fn header_exists(&self, target_name: &str) -> Option<usize> {
        let mut found = false;
        let mut found_id = 0;
        for (id, name) in self.headers.clone() {
            if name == target_name {
                found = true;
                found_id = id;
                break;
            }
        }
        if found {
            Some(found_id)
        } else {
            None
        }
    }
}

pub struct Thread {
    pub id: usize,
    pub pages: BTreeMap<u64, Page>,
    pub controller: ThreadController,
    pub heap: HeapType, //HEAP OR SHARED HEAP
    pub tasks: Vec<u64>,
    pub hang: bool,
}

impl Thread {
    pub fn new(id: usize, thread_controller: ThreadController) -> Thread {
        let mut pages: BTreeMap<u64, Page> = BTreeMap::new();
        //Ring zero
        pages.insert(
            1,
            Page {
                page_id: 1,
                headers: BTreeMap::new(),
                stack: stack::Stack::new(1),
                step: 0,
            },
        );

        Thread {
            id,
            pages,
            heap: HeapType::SolidHeap(heap::Heap::new()),
            tasks: vec![0],
            hang: true,
            controller: thread_controller,
        }
    }

    pub fn get_page(&mut self, targeted_page: u64) -> Option<&mut Page> {
        self.pages.get_mut(&targeted_page)
    }

    pub fn add_data_to_heap(&mut self, data: definite::types::Types, active_page: u64) -> usize {
        pub fn resolve_reference(
            page: Page,
            variable_type: definite::types::variable::VariableType,
        ) -> usize {
            let mut found = false;
            let mut found_element_id: usize = 0;
            for (element_id, variable_name) in page.headers.clone() {
                if variable_type.value == variable_name {
                    found_element_id = element_id;
                    found = true;
                    break;
                }
            }

            if found {
                let mut element_found = false;
                let mut found_heap_id: usize = 0;
                for element in page.stack.elements.clone() {
                    match element {
                        stack::StackElements::Function(e) => {
                            if e.id == found_element_id {
                                panic!("Reference of 'functions' not yet supported")
                            }
                        }
                        stack::StackElements::NativeFunction(e) => {
                            if e.id == found_element_id {
                                panic!("Reference of 'native functions' not yet supported");
                            }
                        }
                        stack::StackElements::Class(e) => {
                            if e.id == found_element_id {
                                panic!("Reference of 'classes' not yet supported");
                            }
                        }
                        stack::StackElements::Variable(e) => {
                            if e.id == found_element_id {
                                if let Some(heap_id) = e.value {
                                    found_heap_id = heap_id;
                                    element_found = true;
                                    break;
                                } else {
                                    panic!("Referenced variable does not have heap");
                                }
                            }
                        }
                        _ => (),
                    }
                }
                if element_found {
                    found_heap_id
                } else {
                    panic!("Unexpected runtime error, cannot find referenced heap in scope")
                }
            } else {
                panic!("Unexpected runtime error, cannot find referenced element in scope")
            }
        }
        match data {
            definite::types::Types::Integer(e) => {
                self.heap
                    .as_solid_heap_mut()
                    .unwrap()
                    .insert(heap::HeapTypes::Integer(match e.value {
                        definite::types::integer::IntegerSize::U8(e) => {
                            heap::HeapIntegerSize::U8(e)
                        }
                        definite::types::integer::IntegerSize::U16(e) => {
                            heap::HeapIntegerSize::U16(e)
                        }
                        definite::types::integer::IntegerSize::U32(e) => {
                            heap::HeapIntegerSize::U32(e)
                        }
                        definite::types::integer::IntegerSize::U64(e) => {
                            heap::HeapIntegerSize::U64(e)
                        }
                        definite::types::integer::IntegerSize::U128(e) => {
                            heap::HeapIntegerSize::U128(e)
                        }
                        definite::types::integer::IntegerSize::Usize(e) => {
                            heap::HeapIntegerSize::Usize(e)
                        }
                        definite::types::integer::IntegerSize::I8(e) => {
                            heap::HeapIntegerSize::I8(e)
                        }
                        definite::types::integer::IntegerSize::I16(e) => {
                            heap::HeapIntegerSize::I16(e)
                        }
                        definite::types::integer::IntegerSize::I32(e) => {
                            heap::HeapIntegerSize::I32(e)
                        }
                        definite::types::integer::IntegerSize::I64(e) => {
                            heap::HeapIntegerSize::I64(e)
                        }
                        definite::types::integer::IntegerSize::I128(e) => {
                            heap::HeapIntegerSize::I128(e)
                        }
                        definite::types::integer::IntegerSize::Isize(e) => {
                            heap::HeapIntegerSize::Isize(e)
                        }
                    }))
            }
            definite::types::Types::Float(e) => {
                self.heap
                    .as_solid_heap_mut()
                    .unwrap()
                    .insert(heap::HeapTypes::Float(match e.value {
                        definite::types::float::FloatSize::F32(e) => heap::HeapFloatSize::F32(e),
                        definite::types::float::FloatSize::F64(e) => heap::HeapFloatSize::F64(e),
                    }))
            }
            definite::types::Types::Bool(bool_type) => self
                .heap
                .as_solid_heap_mut()
                .unwrap()
                .insert(heap::HeapTypes::Bool(if bool_type.value { 1 } else { 0 })),
            definite::types::Types::String(string_type) => self
                .heap
                .as_solid_heap_mut()
                .unwrap()
                .insert(heap::HeapTypes::String(
                    string_type.value.clone().as_bytes().to_vec(),
                )),
            definite::types::Types::Char(char_type) => self
                .heap
                .as_solid_heap_mut()
                .unwrap()
                .insert(heap::HeapTypes::Char(char_type.value as u32)),
            definite::types::Types::Collective(collective_type) => {
                let mut keys: Vec<usize> = Vec::new();
                let mut values: Vec<usize> = Vec::new();

                for entry in collective_type.entries {
                    let key_id = self.add_data_to_heap(*entry.key.clone(), active_page) - 1;
                    let value_id = self.add_data_to_heap(*entry.key, active_page) - 1;
                    keys.push(key_id);
                    values.push(value_id);
                }

                self.heap
                    .as_solid_heap_mut()
                    .unwrap()
                    .insert(heap::HeapTypes::Collective(heap::Collective {
                        keys,
                        values,
                    }))
            }
            definite::types::Types::Reference(_) => todo!(),
            definite::types::Types::Operator(_) => todo!(),
            definite::types::Types::Cloak(cloak_type) => {
                let mut entries: Vec<usize> = Vec::new();
                for entry in cloak_type.collective {
                    entries.push(self.add_data_to_heap(*entry.value, active_page));
                }
                self.heap
                    .as_solid_heap_mut()
                    .unwrap()
                    .insert(heap::HeapTypes::Cloak(entries))
            }
            definite::types::Types::Array(array_type) => {
                let mut entries: Vec<usize> = Vec::new();
                for entry in array_type.collective {
                    entries.push(self.add_data_to_heap(*entry.value, active_page) - 1);
                }
                self.heap
                    .as_solid_heap_mut()
                    .unwrap()
                    .insert(heap::HeapTypes::Array(entries))
            }
            definite::types::Types::ArrowFunction(_) => todo!(),
            definite::types::Types::ConstructedClass(_) => todo!(),
            definite::types::Types::FunctionCall(_) => todo!(),
            definite::types::Types::Void => self
                .heap
                .as_solid_heap_mut()
                .unwrap()
                .insert(heap::HeapTypes::Void),
            definite::types::Types::NullResolver(_) => todo!(),
            definite::types::Types::Negative(negative) => {
                pub fn resolve_negative(
                    targeted_page: Option<(Page, heap::Heap)>,
                    value: definite::types::Types,
                ) -> u8 {
                    match value {
                        definite::types::Types::Integer(integer_type) => {
                            if match integer_type.value {
                                definite::types::integer::IntegerSize::U8(e) => e == 0,
                                definite::types::integer::IntegerSize::U16(e) => e == 0,
                                definite::types::integer::IntegerSize::U32(e) => e == 0,
                                definite::types::integer::IntegerSize::U64(e) => e == 0,
                                definite::types::integer::IntegerSize::U128(e) => e == 0,
                                definite::types::integer::IntegerSize::Usize(e) => e == 0,
                                definite::types::integer::IntegerSize::I8(e) => e == 0,
                                definite::types::integer::IntegerSize::I16(e) => e == 0,
                                definite::types::integer::IntegerSize::I32(e) => e == 0,
                                definite::types::integer::IntegerSize::I64(e) => e == 0,
                                definite::types::integer::IntegerSize::I128(e) => e == 0,
                                definite::types::integer::IntegerSize::Isize(e) => e == 0,
                            } {
                                1
                            } else {
                                0
                            }
                        }
                        definite::types::Types::Float(float_type) => {
                            if match float_type.value {
                                definite::types::float::FloatSize::F32(e) => e == 0.0,
                                definite::types::float::FloatSize::F64(e) => e == 0.0,
                            } {
                                1
                            } else {
                                0
                            }
                        }
                        definite::types::Types::Bool(bool_type) => {
                            if bool_type.value {
                                0
                            } else {
                                1
                            }
                        }
                        definite::types::Types::String(string_type) => {
                            if string_type.value == "" {
                                1
                            } else {
                                0
                            }
                        }
                        definite::types::Types::Char(char_type) => {
                            if char_type.value == '\0' {
                                1
                            } else {
                                0
                            }
                        }
                        definite::types::Types::Reference(_) => todo!(),
                        definite::types::Types::Operator(_) => todo!(),
                        definite::types::Types::ConstructedClass(_) => todo!(),
                        definite::types::Types::FunctionCall(_) => todo!(),
                        definite::types::Types::Void => 1,
                        definite::types::Types::NullResolver(_) => todo!(),
                        definite::types::Types::Negative(negative_type) => {
                            resolve_negative(targeted_page, *negative_type.value)
                        }
                        definite::types::Types::VariableType(e) => match targeted_page {
                            Some((page, heap)) => {
                                let found_heap = resolve_reference(page.clone(), e);
                                match heap.values.get(&found_heap) {
                                        Some(found_heap_data) => {
                                            if match found_heap_data {
                                                heap::HeapTypes::Integer(integer_size) => match *integer_size {
                                                    heap::HeapIntegerSize::U8(e) => e == 0,
                                                    heap::HeapIntegerSize::U16(e) => e == 0,
                                                    heap::HeapIntegerSize::U32(e) => e == 0,
                                                    heap::HeapIntegerSize::U64(e) => e == 0,
                                                    heap::HeapIntegerSize::U128(e) => e == 0,
                                                    heap::HeapIntegerSize::Usize(e) => e == 0,
                                                    heap::HeapIntegerSize::I8(e) => e == 0,
                                                    heap::HeapIntegerSize::I16(e) => e == 0,
                                                    heap::HeapIntegerSize::I32(e) => e == 0,
                                                    heap::HeapIntegerSize::I64(e) => e == 0,
                                                    heap::HeapIntegerSize::I128(e) => e == 0,
                                                    heap::HeapIntegerSize::Isize(e) => e == 0,
                                                },
                                                heap::HeapTypes::Float(float_size) => match *float_size {
                                                    heap::HeapFloatSize::F32(e) => e == 0.0,
                                                    heap::HeapFloatSize::F64(e) => e == 0.0,
                                                },
                                                heap::HeapTypes::Bool(e) => *e == 0,
                                                heap::HeapTypes::String(e) => e.len() == 0,
                                                heap::HeapTypes::Char(e) => char::from_u32(*e).unwrap_or('\0') == '\0',
                                                heap::HeapTypes::Collective(_) => false,
                                                heap::HeapTypes::Array(_) => false,
                                                heap::HeapTypes::Cloak(_) => false,
                                                heap::HeapTypes::Void => false,
                                                heap::HeapTypes::Null => false,
                                            } {1} else { 0}
                                        },
                                        None => panic!("Unexpected runtime error, cannot find referenced heap value '{:#04x}'", found_heap)
                                    }
                            }
                            None => panic!("Unexpected runtime error"),
                        },
                        definite::types::Types::Null => 1,
                        _ => 0,
                    }
                }

                match self.pages.get(&active_page) {
                    Some(page) => {
                        let clone_heap = self.heap.as_solid_heap().unwrap().clone();
                        self.heap
                            .as_solid_heap_mut()
                            .unwrap()
                            .insert(heap::HeapTypes::Bool(resolve_negative(
                                Some((page.clone(), clone_heap)),
                                *negative.value,
                            )))
                    }
                    None => panic!("Unexpected runtime error, cannot find page"),
                }
            }
            definite::types::Types::VariableType(e) => match self.pages.get(&active_page) {
                Some(page) => resolve_reference(page.clone(), e),
                None => panic!("Unexpected runtime error, cannot find page"),
            },
            definite::types::Types::Null => self
                .heap
                .as_solid_heap_mut()
                .unwrap()
                .insert(heap::HeapTypes::Null),
        }
    }

    pub fn add_item_to_stack(
        &mut self,
        page_id: u64,
        item: definite::items::Collecting,
    ) -> Option<(usize, bool, bool)> {
        match item {
            definite::items::Collecting::ImportItem(import_item) => {
                if !self.pages.contains_key(&(import_item.resolution_id as u64)) {
                    self.pages.insert(
                        import_item.resolution_id.clone() as u64,
                        Page {
                            page_id: import_item.resolution_id as u64,
                            headers: BTreeMap::new(),
                            stack: stack::Stack::new(import_item.resolution_id.clone() as usize),
                            step: 0,
                        },
                    );
                }
                let ret = self.add_item_to_stack(page_id, *import_item.item.clone());
                if let Some(element_id) = ret {
                    if element_id.1 && !element_id.2 {
                        match self.pages.get_mut(&page_id) {
                            Some(page) => {
                                page.stack.register_bridge_reference(
                                    import_item.resolution_id as usize,
                                    element_id.0,
                                );
                            }
                            None => {
                                panic!(
                                    "UNEXPECTED RUNTIME BEHAVIOUR, FAILED TO FIND PAGE: {:#?}",
                                    page_id
                                );
                            }
                        }
                    }
                    Some((element_id.0, element_id.1, true))
                } else {
                    panic!(
                        "UNEXPECTED RUNTIME BEHAVIOUR, UNRESPONSIVE ELEMENT: {:#?}",
                        import_item.item
                    );
                }
            }
            definite::items::Collecting::Variable(variable) => {
                let type_name = match variable.rtype {
                    definite::definers::DefinerCollecting::Array(_) => "array".to_owned(),
                    definite::definers::DefinerCollecting::Future(_) => "future".to_owned(),
                    definite::definers::DefinerCollecting::GrowableArray(_) => {
                        "growableArray".to_owned()
                    }
                    definite::definers::DefinerCollecting::Generic(e) => e.rtype,
                    definite::definers::DefinerCollecting::Function(_) => "function".to_owned(),
                    definite::definers::DefinerCollecting::Cloak(_) => "cloak".to_owned(),
                    definite::definers::DefinerCollecting::Collective(_) => "collective".to_owned(),
                    definite::definers::DefinerCollecting::Nullable(_) => "nullAble".to_owned(),
                    definite::definers::DefinerCollecting::Dynamic => "dyn".to_owned(),
                };
                let type_id = self.look_up_for_item_by_name(&type_name, page_id);
                let value_heap_id = self.add_data_to_heap(variable.value, page_id).clone();

                match self.pages.get_mut(&page_id) {
                    Some(page) => match type_id {
                        Some(type_element) => {
                            let rtype = match type_element.0 {
                                stack::StackElements::Class(class) => {
                                    stack::StackElement::Type((class.id, type_element.1))
                                }
                                stack::StackElements::Generic(generic) => {
                                    stack::StackElement::Generic((generic.id, type_element.1))
                                }
                                _ => panic!("Unexpected runtime behaviour"),
                            };

                            let element_id = page.stack.register_variable(
                                rtype,
                                Some(value_heap_id),
                                variable.dynamic,
                            );
                            page.headers.insert(element_id, variable.name);
                            Some((element_id, variable.public, false))
                        }
                        None => {
                            panic!(
                                "Runtime failed to find element '{:#?}' in page '{:#04x}'",
                                type_name, page_id as u64,
                            );
                        }
                    },
                    None => panic!("Runtime failed to find page: '{}';", page_id as u64,),
                }
            }
            definite::items::Collecting::Function(function) => {
                let mut child_stack = stack::Stack::new((page_id + 1) as usize);
                let mut child_headers: BTreeMap<usize, String> = BTreeMap::new();

                let mut params: Vec<(stack::StackElement, usize)> = Vec::new(); //((type, referenced_page), inner_page_pos)
                let mut ret: stack::StackElement = stack::StackElement::Type((0, 0));

                for (id, param) in function.parameters.into_iter().enumerate() {
                    /*
                        Find referenced type data in pages, register header and param data in inner_page
                    */
                    let rtype = match param.rtype {
                        definite::definers::DefinerCollecting::Array(_) => "array".to_owned(),
                        definite::definers::DefinerCollecting::Future(_) => "future".to_owned(),
                        definite::definers::DefinerCollecting::GrowableArray(_) => {
                            "growableArray".to_owned()
                        }
                        definite::definers::DefinerCollecting::Generic(e) => e.rtype,
                        definite::definers::DefinerCollecting::Function(_) => "function".to_owned(),
                        definite::definers::DefinerCollecting::Cloak(_) => "cloak".to_owned(),
                        definite::definers::DefinerCollecting::Collective(_) => {
                            "collective".to_owned()
                        }
                        definite::definers::DefinerCollecting::Nullable(_) => "nullAble".to_owned(),
                        definite::definers::DefinerCollecting::Dynamic => "dyn".to_owned(),
                    };
                    let type_id = self.look_up_for_item_by_name(&rtype, page_id as u64);
                    match type_id {
                        Some(type_element) => {
                            let rtype = match type_element.0 {
                                stack::StackElements::Class(class) => {
                                    stack::StackElement::Type((class.id, type_element.1))
                                }
                                stack::StackElements::Generic(generic) => {
                                    stack::StackElement::Generic((generic.id, type_element.1))
                                }
                                _ => panic!("Unexpected runtime behaviour"),
                            };
                            child_headers
                                .insert(child_stack.register_parameter(rtype.clone()), param.name);
                            params.push((rtype.clone(), id));
                        }
                        None => {
                            panic!("Runtime failed to find element for function parameter: '{}', page_id: {:#04x}; DUMP:\n\n{}",
                                        rtype,
                                        page_id,
                                        crate::runtime::panic_dumper(&*self),
                                    );
                        }
                    }
                }

                let rtype = match function.return_type {
                    definite::definers::DefinerCollecting::Array(_) => "array".to_owned(),
                    definite::definers::DefinerCollecting::Future(_) => "future".to_owned(),
                    definite::definers::DefinerCollecting::GrowableArray(_) => {
                        "growableArray".to_owned()
                    }
                    definite::definers::DefinerCollecting::Generic(e) => e.rtype,
                    definite::definers::DefinerCollecting::Function(_) => "function".to_owned(),
                    definite::definers::DefinerCollecting::Cloak(_) => "cloak".to_owned(),
                    definite::definers::DefinerCollecting::Collective(_) => "collective".to_owned(),
                    definite::definers::DefinerCollecting::Nullable(_) => "nullAble".to_owned(),
                    definite::definers::DefinerCollecting::Dynamic => "dyn".to_owned(),
                };
                //Match return type with pages, register to function
                let type_id = self.look_up_for_item_by_name(&rtype, page_id as u64);
                match type_id {
                    Some(type_element) => {
                        ret = match type_element.0 {
                            stack::StackElements::Class(class) => {
                                stack::StackElement::Type((class.id, type_element.1))
                            }
                            stack::StackElements::Generic(generic) => {
                                stack::StackElement::Generic((generic.id, type_element.1))
                            }
                            _ => panic!("Unexpected runtime behaviour"),
                        };
                    }
                    None => {
                        panic!(
                            "Runtime failed to find element;\n\r DUMP: {}, for return type: '{}', page_id: {:#04x}",
                            crate::runtime::panic_dumper(&*self),
                            &rtype,
                            page_id
                        )
                    }
                }
                match self.pages.get_mut(&page_id) {
                    Some(page) => {
                        let inner_page_id = rand::random::<u64>();
                        let element_id =
                            page.stack
                                .register_function(params, inner_page_id as usize, ret);
                        page.headers.insert(element_id, function.name);

                        //Import upper imports to inner scope
                        for element in page.stack.elements.clone() {
                            if let stack::StackElements::Bridge(bridge) = element {
                                child_stack.register_bridge(bridge.page_id, None);
                                for i in bridge.targets {
                                    child_stack.register_bridge_reference(bridge.page_id, i);
                                }
                            }
                        }

                        self.pages.insert(
                            inner_page_id,
                            Page {
                                page_id: inner_page_id,
                                headers: child_headers,
                                stack: child_stack,
                                step: 0,
                            },
                        );

                        for item in function.inside_code {
                            self.add_item_to_stack(page_id, item);
                        }
                        Some((element_id, function.public, false))
                    }
                    None => {
                        panic!("Runtime failed to find page: '{}';", page_id as u64);
                    }
                }
            }
            definite::items::Collecting::ForLoop(_) => Some((0, false, false)),
            definite::items::Collecting::Condition(condition) => {
                let mut chains: Vec<stack::ConditionChainType> =
                    Vec::with_capacity(condition.chains.len());
                for chain in condition.chains {
                    let inner_page_id = rand::random::<u64>();
                    let mut child_stack = stack::Stack::new(inner_page_id as usize);

                    let condition_heap_id = match chain.rtype {
                        definite::items::condition::ConditionType::Else => 0,
                        _ => self.add_data_to_heap(*chain.condition, page_id),
                    };

                    match self.pages.get_mut(&page_id) {
                        Some(page) => {
                            //Import upper imports to inner scope
                            for element in page.stack.elements.clone() {
                                if let stack::StackElements::Bridge(bridge) = element {
                                    child_stack.register_bridge(bridge.page_id, None);
                                    for i in bridge.targets {
                                        child_stack.register_bridge_reference(bridge.page_id, i);
                                    }
                                }
                            }

                            for item in chain.code {
                                self.add_item_to_stack(page_id, item);
                            }
                        }
                        None => {
                            panic!("Runtime failed to find page: '{}';", page_id as u64,);
                        }
                    }

                    self.pages.insert(
                        inner_page_id,
                        Page {
                            page_id: inner_page_id,
                            headers: BTreeMap::new(),
                            stack: child_stack,
                            step: 0,
                        },
                    );

                    chains.push(match chain.rtype {
                        definite::items::condition::ConditionType::If => {
                            stack::ConditionChainType::If((
                                condition_heap_id,
                                inner_page_id as usize,
                            ))
                        }
                        definite::items::condition::ConditionType::ElseIf => {
                            stack::ConditionChainType::ElseIf((
                                condition_heap_id,
                                inner_page_id as usize,
                            ))
                        }
                        definite::items::condition::ConditionType::Else => {
                            stack::ConditionChainType::Else(inner_page_id as usize)
                        }
                    });
                }

                let element_id = match self.pages.get_mut(&page_id) {
                    Some(page) => page.stack.register_condition_chain(chains),
                    None => {
                        panic!("Runtime failed to find page: '{}';", page_id as u64,);
                    }
                };

                Some((element_id, true, false))
            }
            definite::items::Collecting::Class(class) => match self.pages.get_mut(&page_id) {
                Some(page) => {
                    match page.header_exists(&class.name) {
                        Some(e) => Some((e, class.public, false)),
                        None => {
                            let mut class_items: Vec<definite::items::Collecting> =
                                vec![definite::items::Collecting::Constructor(class.constructor)];

                            class_items.extend(
                                class
                                    .getters
                                    .into_iter()
                                    .map(|x| definite::items::Collecting::Getter(x))
                                    .collect::<Vec<_>>(),
                            );

                            class_items.extend(
                                class
                                    .setters
                                    .into_iter()
                                    .map(|x| definite::items::Collecting::Setter(x))
                                    .collect::<Vec<_>>(),
                            );

                            class_items.extend(
                                class
                                    .methods
                                    .into_iter()
                                    .map(|x| definite::items::Collecting::Function(x))
                                    .collect::<Vec<_>>(),
                            );

                            class_items.extend(
                                class
                                    .properties
                                    .into_iter()
                                    .map(|x| definite::items::Collecting::Variable(x))
                                    .collect::<Vec<_>>(),
                            );
                            let inner_page_id = rand::random::<u64>();

                            let mut generics = Vec::new();
                            let mut child_headers: BTreeMap<usize, String> = BTreeMap::new();
                            let mut child_stack = stack::Stack::new(inner_page_id as usize);

                            for (id, generic) in class.generic_definings.into_iter().enumerate() {
                                child_headers.insert(id, generic.name.clone());
                                generics.push(id);
                                child_stack.register_generic(id);
                            }

                            //Import upper imports to inner scope
                            for element in page.stack.elements.clone() {
                                if let stack::StackElements::Bridge(bridge) = element {
                                    child_stack.register_bridge(bridge.page_id, None);
                                    for i in bridge.targets {
                                        child_stack.register_bridge_reference(bridge.page_id, i);
                                    }
                                }
                            }

                            let element_id =
                                page.stack.register_class(inner_page_id as usize, generics);
                            page.headers.insert(element_id, class.name.clone());
                            self.pages.insert(
                                inner_page_id,
                                Page {
                                    page_id: inner_page_id,
                                    headers: child_headers,
                                    stack: child_stack,
                                    step: 0,
                                },
                            );

                            for item in class_items {
                                self.add_item_to_stack(inner_page_id, item);
                            }
                            Some((element_id, class.public, false))
                        }
                    }
                }
                None => panic!("Runtime failed to find page: '{:#04x}';", page_id as u64),
            },
            definite::items::Collecting::Ret(ret) => {
                let heap_value_id = self.add_data_to_heap(ret.value, page_id).clone() - 1;
                match self.pages.get_mut(&page_id) {
                    Some(page) => {
                        let element_id = page.stack.register_ret(heap_value_id);
                        Some((element_id, true, false))
                    }
                    None => panic!("Runtime failed to find page: '{}';", page_id as u64),
                }
            }
            definite::items::Collecting::Constructor(_) => Some((0, false, false)),
            definite::items::Collecting::Caller(_) => Some((0, false, false)),
            definite::items::Collecting::Import(import) => {
                if !self.pages.contains_key(&(import.resolution_id as u64)) {
                    self.pages.insert(
                        import.resolution_id.clone() as u64,
                        Page {
                            page_id: import.resolution_id as u64,
                            headers: BTreeMap::new(),
                            stack: stack::Stack::new(import.resolution_id.clone() as usize),
                            step: 0,
                        },
                    );
                }

                match self.pages.get_mut(&page_id) {
                    Some(page) => {
                        if page
                            .stack
                            .clone()
                            .element_exists(import.resolution_id as usize)
                            .is_none()
                        {
                            page.stack
                                .register_bridge(import.resolution_id as usize, None);
                        }
                    }
                    None => panic!("Runtime failed to find page: '{}';", page_id as u64),
                }
                Some((import.id as usize, import.public, true))
            }
            definite::items::Collecting::FileKey(_) => Some((0, false, false)),
            definite::items::Collecting::Getter(_) => Some((0, false, false)),
            definite::items::Collecting::Setter(_) => Some((0, false, false)),
            definite::items::Collecting::NativeClass => Some((0, false, false)),
            definite::items::Collecting::ValueCall(_) => Some((0, false, false)),
            definite::items::Collecting::Enum(_) => Some((0, false, false)),
            definite::items::Collecting::NativeFunction(function) => {
                let mut params: Vec<(stack::StackElement, usize, String)> = Vec::new(); //((type, referenced_page), param_name)
                let mut ret: stack::StackElement = stack::StackElement::Type((0, 0));

                for (id, param) in function.parameters.into_iter().enumerate() {
                    /*
                        Find referenced type data in pages, register header and param data in inner_page
                    */
                    let rtype = match param.rtype {
                        definite::definers::DefinerCollecting::Array(_) => "array".to_owned(),
                        definite::definers::DefinerCollecting::Future(_) => "future".to_owned(),
                        definite::definers::DefinerCollecting::GrowableArray(_) => {
                            "growableArray".to_owned()
                        }
                        definite::definers::DefinerCollecting::Generic(e) => e.rtype,
                        definite::definers::DefinerCollecting::Function(_) => "function".to_owned(),
                        definite::definers::DefinerCollecting::Cloak(_) => "cloak".to_owned(),
                        definite::definers::DefinerCollecting::Collective(_) => {
                            "collective".to_owned()
                        }
                        definite::definers::DefinerCollecting::Nullable(_) => "nullAble".to_owned(),
                        definite::definers::DefinerCollecting::Dynamic => "dyn".to_owned(),
                    };
                    let type_id = self.look_up_for_item_by_name(&rtype, page_id as u64);
                    match type_id {
                        Some(type_element) => {
                            let rtype = match type_element.0 {
                                stack::StackElements::Class(class) => {
                                    stack::StackElement::Type((class.id, type_element.1))
                                }
                                stack::StackElements::Generic(generic) => {
                                    stack::StackElement::Generic((generic.id, type_element.1))
                                }
                                _ => panic!("Unexpected runtime behaviour"),
                            };
                            params.push((rtype.clone(), id, param.name));
                        }
                        None => {
                            panic!(
                                "Runtime failed to find element for function parameter: '{}';",
                                page_id,
                            )
                        }
                    }
                }

                let rtype = match function.return_type {
                    definite::definers::DefinerCollecting::Array(_) => "array".to_owned(),
                    definite::definers::DefinerCollecting::Future(_) => "future".to_owned(),
                    definite::definers::DefinerCollecting::GrowableArray(_) => {
                        "growableArray".to_owned()
                    }
                    definite::definers::DefinerCollecting::Generic(e) => e.rtype,
                    definite::definers::DefinerCollecting::Function(_) => "function".to_owned(),
                    definite::definers::DefinerCollecting::Cloak(_) => "cloak".to_owned(),
                    definite::definers::DefinerCollecting::Collective(_) => "collective".to_owned(),
                    definite::definers::DefinerCollecting::Nullable(_) => "nullAble".to_owned(),
                    definite::definers::DefinerCollecting::Dynamic => "dyn".to_owned(),
                };
                //Match return type with pages, register to function
                let type_id = self.look_up_for_item_by_name(&rtype, page_id as u64);
                match type_id {
                    Some(type_element) => {
                        ret = match type_element.0 {
                            stack::StackElements::Class(class) => {
                                stack::StackElement::Type((class.id, type_element.1))
                            }
                            stack::StackElements::Generic(generic) => {
                                stack::StackElement::Generic((generic.id, type_element.1))
                            }
                            _ => panic!("Unexpected runtime behaviour"),
                        };
                    }
                    None => {
                        panic!(
                            "Runtime failed to find element for return type: '{}';",
                            &rtype
                        )
                    }
                }

                match self.pages.get_mut(&page_id) {
                    Some(page) => {
                        let mut rebuilded_params: Vec<(stack::StackElement, usize)> = Vec::new();

                        for param in params {
                            rebuilded_params.push((param.0, param.1 + page.headers.len()));
                            page.headers
                                .insert(param.1 + page.headers.len() + 1, param.2);
                        }

                        let element_id = page.stack.register_native_function(rebuilded_params, ret);
                        page.headers.insert((element_id) as usize, function.name);
                        Some((element_id, function.public, false))
                    }
                    None => {
                        panic!("Runtime failed to find page: '{}';", page_id);
                    }
                }
            }
            definite::items::Collecting::None => Some((0, false, false)),
        }
    }

    pub fn glb_look_up_for_item_by_name(&self, name: &str) -> Option<stack::StackElements> {
        std::println!("[WARNING] NOT IDEAL TO REQUIRE ITEM FROM OTHER SCOPES");
        let mut found = false;
        let mut found_type = stack::StackElements::None;

        'page_loop: for (_, current_page) in self.pages.clone() {
            match current_page.header_exists(name) {
                Some(id) => {
                    //If current page has the type
                    match current_page.stack.clone().element_exists(id) {
                        Some(element_exists) => {
                            found_type = element_exists;
                            found = true;
                            break 'page_loop;
                        }
                        _ => (),
                    }
                }
                None => {
                    //Else look up in imported pages
                    for i in current_page.stack.elements.clone() {
                        if let stack::StackElements::Bridge(bridge) = i {
                            match self.pages.get(&(bridge.page_id as u64)) {
                                Some(page) => match page.header_exists(name) {
                                    Some(e) => match page.stack.clone().element_exists(e) {
                                        Some(found_rtype) => {
                                            found = true;
                                            found_type = found_rtype;
                                            break 'page_loop;
                                        }
                                        None => panic!("UNEXPECTED RUNTIME BEHAVIOUR"),
                                    },
                                    None => (),
                                },
                                None => panic!("UNEXPECTED BRIDGE ERROR"),
                            }
                        }
                    }
                }
            }
        }

        if found {
            Some(found_type)
        } else {
            None
        }
    }

    pub fn look_up_for_item_by_name(
        &self,
        name: &str,
        current_page_id: u64,
    ) -> Option<(stack::StackElements, usize)> {
        match self.pages.get(&current_page_id.clone()) {
            Some(current_page) => {
                match current_page.header_exists(name) {
                    Some(id) => {
                        //If current page has the type
                        if let Some(element) = current_page.stack.clone().element_exists(id) {
                            Some((element, current_page_id as usize))
                        } else {
                            None
                        }
                    }
                    None => {
                        //Else look up in imported pages

                        let mut found_bridge = 0;
                        let mut found = false;
                        let mut found_type = stack::StackElements::None;

                        'search: for i in current_page.stack.elements.clone() {
                            if let stack::StackElements::Bridge(bridge) = i {
                                match self.pages.get(&(bridge.page_id as u64)) {
                                    Some(page) => match page.header_exists(name) {
                                        Some(e) => match page.stack.clone().element_exists(e) {
                                            Some(found_rtype) => {
                                                found = true;
                                                found_type = found_rtype;
                                                found_bridge = bridge.page_id;
                                                break 'search;
                                            }
                                            None => panic!("UNEXPECTED RUNTIME BEHAVIOUR"),
                                        },
                                        None => (),
                                    },
                                    None => panic!("UNEXPECTED BRIDGE ERROR"),
                                }
                            }
                        }

                        if found {
                            Some((found_type, found_bridge))
                        } else {
                            None
                        }
                    }
                }
            }
            None => None,
        }
    }

    pub fn look_up_for_item(&self, page_id: u64, target: usize) -> Option<stack::StackElements> {
        match self.pages.get(&page_id) {
            Some(page) => {
                let element = page.stack.clone().element_exists(target);
                match element.clone() {
                    Some(e) => match e {
                        stack::StackElements::Bridge(e) => {
                            self.look_up_for_item(e.page_id as u64, target)
                        }
                        _ => element,
                    },
                    None => None,
                }
            }
            None => None,
        }
    }

    pub fn run(&mut self) {
        if self.tasks.len() == 0 {
            self.controller.notifier.call(NotifierMessage {
                stack_id: 0,
                stack_step: 0,
                message_type: NotifierMessageType::ThreadStopped,
            });
        } else {
            let current_job = self.tasks.last().unwrap().clone();
            match self.pages.get_mut(&current_job) {
                Some(page) => {
                    let _frame_response =
                        self.controller.require_frame.call((current_job, page.step));
                    match page.stack.elements.get(page.step) {
                        Some(element) => {
                            std::println!("THREAD RUN COMMAND: {:#?}", element);
                            //match element {
                            //    stack::StackElements::Function(_) => todo!(),
                            //    stack::StackElements::Class(_) => todo!(),
                            //    stack::StackElements::Variable(e) => {},
                            //};
                            page.step += 1;
                            if page.stack.elements.len() == page.step {
                                self.tasks.pop();
                                self.controller.notifier.call(NotifierMessage {
                                    stack_id: current_job,
                                    stack_step: 0,
                                    message_type: NotifierMessageType::StackComplete,
                                });
                            } else {
                                self.controller.notifier.call(NotifierMessage {
                                    stack_id: current_job,
                                    stack_step: 0,
                                    message_type: NotifierMessageType::StackStep,
                                });
                            }
                        }
                        None => {
                            self.controller.notifier.call(NotifierMessage {
                                stack_id: current_job,
                                stack_step: 0,
                                message_type: NotifierMessageType::CommandFault,
                            });
                            self.tasks = vec![];
                        }
                    }
                }
                None => {
                    self.controller.notifier.call(NotifierMessage {
                        stack_id: current_job,
                        stack_step: 0,
                        message_type: NotifierMessageType::StackIdFault,
                    });
                    self.tasks = vec![];
                }
            }
        }

        if !self.hang {}
    }
}
