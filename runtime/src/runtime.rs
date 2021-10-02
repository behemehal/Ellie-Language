use crate::heap;
use crate::stack;
use crate::thread;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite;
use rand;

pub enum RuntimeEventMessage {
    None,
}

pub enum RuntimeEventResponse {
    None,
}

pub trait RuntimeEventFn {
    fn call(self, args: RuntimeEventMessage) -> RuntimeEventResponse;
}

impl<T> RuntimeEventFn for T
where
    T: FnOnce(RuntimeEventMessage) -> RuntimeEventResponse + Sized,
{
    fn call(self, args: RuntimeEventMessage) -> RuntimeEventResponse {
        self(args)
    }
}

pub struct Runtime {
    pub threads: Vec<thread::Thread>,
    pub events: Box<dyn RuntimeEventFn>,
    initialized: bool,
}

pub fn panic_dumper(thread: &thread::Thread) -> String {
    let mut dump_data = "\r\n---\n\r".to_owned();
    let mut stack_dump = String::new();
    for stack in thread.pages.clone() {
        let mut headers = String::new();

        for item in stack.1.headers {
            headers += &format!("\t\t\t{:#04x} : {}\n", item.0, item.1)
        }

        stack_dump += &format!(
            "\t---\n\tPage {:#04x}:\n\t\tHeaders:\n{}\n\t\tStack:\n\t{}\n\t\tHEAP:\n{}\n",
            stack.0,
            headers,
            stack.1.stack.dump(),
            stack.1.heap.clone().dump()
        );
    }

    dump_data += &format!("Pages:\n{}\n\t---", stack_dump);
    dump_data
}

impl Runtime {
    pub fn spawn(event_listener: Box<dyn RuntimeEventFn>) -> Runtime {
        let mut runtime = Runtime {
            threads: Vec::new(),
            events: event_listener,
            initialized: true,
        };
        runtime.initialized = true;
        runtime
    }

    pub fn run(&mut self, code: Vec<definite::items::Collecting>) {
        let mut thread_controller = thread::ThreadController::new();
        thread_controller.notifier = Box::new(move |x: thread::NotifierMessage| {
            std::println!(
                "THREAD MESSAGE: {}",
                alloc::format!("{:?}", x.clone()).to_uppercase()
            );

            //event_listener.call(RuntimeEventMessage::None);
        });

        thread_controller.require_frame =
            Box::new(move |(stack_id, stack_step)| -> thread::ThreadMessage {
                std::println!("require_frame: stack_id: {} STEP: {}", stack_id, stack_step);
                //self.events.call(RuntimeEventMessage::None);
                thread::ThreadMessage::CONTINUE
            });

        pub fn add_data_to_heap(heap: &mut heap::Heap, data: definite::types::Types) -> usize {
            match data {
                definite::types::Types::Integer(e) => {
                    heap.insert(heap::HeapTypes::Integer(match e.value {
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
                    heap.insert(heap::HeapTypes::Float(match e.value {
                        definite::types::float::FloatSize::F32(e) => heap::HeapFloatSize::F32(e),
                        definite::types::float::FloatSize::F64(e) => heap::HeapFloatSize::F64(e),
                    }))
                }
                definite::types::Types::Bool(bool_type) => {
                    heap.insert(heap::HeapTypes::Bool(if bool_type.value { 1 } else { 0 }))
                }
                definite::types::Types::String(string_type) => {
                    heap.insert(heap::HeapTypes::String(string_type.value.as_ptr()))
                }
                definite::types::Types::Char(_) => todo!(),
                definite::types::Types::Collective(_) => todo!(),
                definite::types::Types::Reference(_) => todo!(),
                definite::types::Types::Operator(_) => todo!(),
                definite::types::Types::Cloak(_) => todo!(),
                definite::types::Types::Array(_) => todo!(),
                definite::types::Types::ArrowFunction(_) => todo!(),
                definite::types::Types::ConstructedClass(_) => todo!(),
                definite::types::Types::FunctionCall(_) => todo!(),
                definite::types::Types::Void => todo!(),
                definite::types::Types::NullResolver(_) => todo!(),
                definite::types::Types::Negative(negative) => {
                    pub fn resolve_negative(value: definite::types::Types) -> u8 {
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
                                resolve_negative(*negative_type.value)
                            }
                            definite::types::Types::VariableType(_) => todo!(),
                            definite::types::Types::Null => 1,
                            _ => 0,
                        }
                    }
                    heap.insert(heap::HeapTypes::Bool(resolve_negative(*negative.value)))
                }

                definite::types::Types::VariableType(_) => todo!(),
                definite::types::Types::Null => heap.insert(heap::HeapTypes::Null),
            }
        }

        //Adds element to stack returns id of it
        pub fn loop_item(
            thread: &mut thread::Thread,
            item: definite::items::Collecting,
            page_id: u64,
        ) -> Option<(usize, bool, bool)> {
            //id, publicity, is_import
            match item {
                definite::items::Collecting::ImportItem(import_item) => {
                    if !thread
                        .pages
                        .contains_key(&(import_item.resolution_id as u64))
                    {
                        thread.pages.insert(
                            import_item.resolution_id.clone() as u64,
                            thread::Page {
                                page_id: import_item.resolution_id as u64,
                                headers: BTreeMap::new(),
                                heap: heap::Heap::new(),
                                stack: stack::Stack::new(import_item.resolution_id.clone() as usize),
                                step: 0,
                            },
                        );
                    }

                    let ret =
                        loop_item(thread, *import_item.item.clone(), import_item.resolution_id);
                    if let Some(element_id) = ret {
                        match thread.pages.get_mut(&page_id) {
                            Some(page) => {
                                if element_id.1 && !element_id.2 {
                                    page.stack.register_bridge_reference(
                                        import_item.resolution_id as usize,
                                        element_id.0,
                                    );
                                }
                                Some((element_id.0, element_id.1, true))
                            }
                            None => {
                                panic!(
                                    "UNEXPECTED RUNTIME BEHAVIOUR Pageid: {:#04x}\n\n: {}",
                                    page_id,
                                    panic_dumper(&*thread)
                                );
                            }
                        }
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
                        definite::definers::DefinerCollecting::Collective(_) => {
                            "collective".to_owned()
                        }
                        definite::definers::DefinerCollecting::Nullable(_) => "nullAble".to_owned(),
                        definite::definers::DefinerCollecting::Dynamic => "dyn".to_owned(),
                    };
                    let type_id = thread.look_up_for_item_by_name(&type_name, page_id);
                    let copy_dump = panic_dumper(&*thread);
                    match thread.pages.get_mut(&page_id) {
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
                                    Some(
                                        add_data_to_heap(&mut page.heap, variable.value).clone()
                                            - 1,
                                    ),
                                    variable.dynamic,
                                );
                                page.headers.insert(element_id, variable.name);
                                Some((element_id, variable.public, false))
                            }
                            None => {
                                panic!(
                                    "Runtime failed to find element '{:#?}' in page '{:#04x}'\n\nDUMP: {}",
                                    type_name,
                                    page_id as u64,
                                    copy_dump
                                );
                            }
                        },
                        None => panic!(
                            "Runtime failed to find page: '{}';\n\nDUMP: {}",
                            page_id as u64,
                            panic_dumper(&*thread)
                        ),
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
                            definite::definers::DefinerCollecting::Function(_) => {
                                "function".to_owned()
                            }
                            definite::definers::DefinerCollecting::Cloak(_) => "cloak".to_owned(),
                            definite::definers::DefinerCollecting::Collective(_) => {
                                "collective".to_owned()
                            }
                            definite::definers::DefinerCollecting::Nullable(_) => {
                                "nullAble".to_owned()
                            }
                            definite::definers::DefinerCollecting::Dynamic => "dyn".to_owned(),
                        };
                        let type_id = thread.look_up_for_item_by_name(&rtype, page_id as u64);
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
                                child_headers.insert(
                                    child_stack.register_parameter(rtype.clone()),
                                    param.name,
                                );
                                params.push((rtype.clone(), id));
                            }
                            None => {
                                panic!(
                                            "Runtime failed to find element for function parameter: '{}', page_id: {:#04x};\n\nDUMP: {}",
                                            rtype,
                                            page_id,
                                            panic_dumper(&*thread)
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
                        definite::definers::DefinerCollecting::Collective(_) => {
                            "collective".to_owned()
                        }
                        definite::definers::DefinerCollecting::Nullable(_) => "nullAble".to_owned(),
                        definite::definers::DefinerCollecting::Dynamic => "dyn".to_owned(),
                    };
                    //Match return type with pages, register to function
                    let type_id = thread.look_up_for_item_by_name(&rtype, page_id as u64);
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
                                "Runtime failed to find element for return type: '{}';\n\nDUMP: {}",
                                &rtype,
                                panic_dumper(&*thread)
                            )
                        }
                    }
                    match thread.pages.get_mut(&page_id) {
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

                            thread.pages.insert(
                                inner_page_id,
                                thread::Page {
                                    page_id: inner_page_id,
                                    headers: child_headers,
                                    heap: heap::Heap::new(),
                                    stack: child_stack,
                                    step: 0,
                                },
                            );

                            for item in function.inside_code {
                                loop_item(thread, item, inner_page_id);
                            }
                            Some((element_id, function.public, false))
                        }
                        None => {
                            panic!(
                                "Runtime failed to find page: '{}';\n\nDUMP: {}",
                                page_id as u64,
                                panic_dumper(&*thread)
                            );
                        }
                    }
                }
                definite::items::Collecting::ForLoop(_) => Some((0, false, false)),
                definite::items::Collecting::Condition(_) => Some((0, false, false)),
                definite::items::Collecting::Class(class) => match thread.pages.get_mut(&page_id) {
                    Some(page) => {
                        match page.header_exists(&class.name) {
                            Some(e) => Some((e, class.public, false)),
                            None => {
                                let mut class_items: Vec<definite::items::Collecting> =
                                    vec![definite::items::Collecting::Constructor(
                                        class.constructor,
                                    )];

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

                                for (id, generic) in class.generic_definings.into_iter().enumerate()
                                {
                                    child_headers.insert(id, generic.name.clone());
                                    generics.push(id);
                                    child_stack.register_generic(id);
                                }

                                //Import upper imports to inner scope
                                for element in page.stack.elements.clone() {
                                    if let stack::StackElements::Bridge(bridge) = element {
                                        child_stack.register_bridge(bridge.page_id, None);
                                        for i in bridge.targets {
                                            child_stack
                                                .register_bridge_reference(bridge.page_id, i);
                                        }
                                    }
                                }

                                let element_id =
                                    page.stack.register_class(inner_page_id as usize, generics);
                                page.headers.insert(element_id, class.name.clone());

                                thread.pages.insert(
                                    inner_page_id,
                                    thread::Page {
                                        page_id: inner_page_id,
                                        headers: child_headers,
                                        heap: heap::Heap::new(),
                                        stack: child_stack,
                                        step: 0,
                                    },
                                );

                                for item in class_items {
                                    loop_item(thread, item, inner_page_id);
                                }
                                Some((element_id, class.public, false))
                            }
                        }
                    }
                    None => panic!(
                        "Runtime failed to find page: '{}';\n\nDUMP: {}",
                        page_id as u64,
                        panic_dumper(&*thread)
                    ),
                },
                definite::items::Collecting::Ret(ret) => match thread.pages.get_mut(&page_id) {
                    Some(page) => {
                        let element_id = page
                            .stack
                            .register_ret(add_data_to_heap(&mut page.heap, ret.value).clone() - 1);
                        Some((element_id, true, false))
                    }
                    None => panic!(
                        "Runtime failed to find page: '{}';\n\nDUMP: {}",
                        page_id as u64,
                        panic_dumper(&*thread)
                    ),
                },
                definite::items::Collecting::Constructor(_) => Some((0, false, false)),
                definite::items::Collecting::Caller(_) => Some((0, false, false)),
                definite::items::Collecting::Import(import) => {
                    if !thread.pages.contains_key(&(import.resolution_id as u64)) {
                        thread.pages.insert(
                            import.resolution_id.clone() as u64,
                            thread::Page {
                                page_id: import.resolution_id as u64,
                                headers: BTreeMap::new(),
                                heap: heap::Heap::new(),
                                stack: stack::Stack::new(import.resolution_id.clone() as usize),
                                step: 0,
                            },
                        );
                    }

                    match thread.pages.get_mut(&page_id) {
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
                        None => panic!(
                            "Runtime failed to find page: '{}';\n\nDUMP: {}",
                            page_id as u64,
                            panic_dumper(&*thread)
                        ),
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
                            definite::definers::DefinerCollecting::Function(_) => {
                                "function".to_owned()
                            }
                            definite::definers::DefinerCollecting::Cloak(_) => "cloak".to_owned(),
                            definite::definers::DefinerCollecting::Collective(_) => {
                                "collective".to_owned()
                            }
                            definite::definers::DefinerCollecting::Nullable(_) => {
                                "nullAble".to_owned()
                            }
                            definite::definers::DefinerCollecting::Dynamic => "dyn".to_owned(),
                        };
                        let type_id = thread.look_up_for_item_by_name(&rtype, page_id as u64);
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
                                            "Runtime failed to find element for function parameter: '{}', page_id: {:#04x},page_id: {}, page: {:#?};\n\nDUMP: {}",
                                            rtype,
                                            page_id,
                                            page_id,
                                            thread.pages.get(&page_id),
                                            panic_dumper(&*thread)
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
                        definite::definers::DefinerCollecting::Collective(_) => {
                            "collective".to_owned()
                        }
                        definite::definers::DefinerCollecting::Nullable(_) => "nullAble".to_owned(),
                        definite::definers::DefinerCollecting::Dynamic => "dyn".to_owned(),
                    };
                    //Match return type with pages, register to function
                    let type_id = thread.look_up_for_item_by_name(&rtype, page_id as u64);
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
                                "Runtime failed to find element for return type: '{}';\n\nDUMP: {}",
                                &rtype,
                                panic_dumper(&*thread)
                            )
                        }
                    }

                    match thread.pages.get_mut(&page_id) {
                        Some(page) => {
                            let mut rebuilded_params: Vec<(stack::StackElement, usize)> =
                                Vec::new();

                            for param in params {
                                rebuilded_params.push((param.0, param.1));
                                page.headers.insert(param.1, param.2);
                            }

                            let element_id =
                                page.stack.register_native_function(rebuilded_params, ret);
                            page.headers.insert(element_id as usize, function.name);
                            Some((element_id, function.public, false))
                        }
                        None => {
                            panic!(
                                "Runtime failed to find page: '{}';\n\nDUMP: {}",
                                page_id as u64,
                                panic_dumper(&*thread)
                            );
                        }
                    }
                }
                definite::items::Collecting::None => Some((0, false, false)),
            }
        }

        let mut main_thread = thread::Thread::new(1, thread_controller);

        for item in code {
            //Split code to pages
            loop_item(&mut main_thread, item, 1);
        }
        self.threads.push(main_thread);
    }

    pub fn dump(&self) -> String {
        let mut dump_data = "DUMP: Şükür kavuşturana\r\n---\n\r".to_owned();
        for thread in &self.threads {
            let mut stack_dump = String::new();
            for stack in thread.pages.clone() {
                let mut headers = String::new();

                for item in stack.1.headers {
                    headers += &format!("\t\t\t{:#04x} : {}\n", item.0, item.1)
                }

                stack_dump += &format!(
                    "\t---\n\tPage {:#04x}:\n\t\tHeaders:\n{}\n\t\tStack:\n\t{}\n\t\tHEAP:\n\t{}\n",
                    stack.0,
                    headers,
                    stack.1.stack.dump(),
                    stack.1.heap.clone().dump()
                );
            }

            dump_data += &format!("Pages:\n{}\n\t---", stack_dump);
        }
        dump_data
    }
}
