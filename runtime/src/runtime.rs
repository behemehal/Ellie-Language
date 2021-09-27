use crate::heap;
use crate::stack;
use crate::thread;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::rc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite;
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
                std::println!("require_frame: stackid: {} STEP: {}", stack_id, stack_step);
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
                definite::types::Types::Bool(_) => todo!(),
                definite::types::Types::String(_) => todo!(),
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
                definite::types::Types::Negative(_) => todo!(),
                definite::types::Types::VariableType(_) => todo!(),
                definite::types::Types::Null => heap.insert(heap::HeapTypes::Null),
            }
        }

        //Adds element to stack returns id of it

        pub fn loop_item(
            thread: &mut thread::Thread,
            item: definite::items::Collecting,
            page_id: u64,
        ) -> Option<(usize, bool)> {
            let current_pages_len = thread.pages.len();

            match item {
                definite::items::Collecting::ImportItem(import_item) => {
                    if !thread.pages.contains_key(&(import_item.from_import as u64)) {
                        thread.pages.insert(
                            import_item.from_import.clone() as u64,
                            thread::Page {
                                page_id: import_item.from_import as u64,
                                headers: BTreeMap::new(),
                                heap: heap::Heap::new(),
                                stack: stack::Stack::new(import_item.from_import.clone() as usize),
                                step: 0,
                            },
                        );
                    }

                    let ret = loop_item(thread, *import_item.item, import_item.from_import);
                    if let Some(element_id) = ret {
                        match thread.pages.get_mut(&page_id) {
                            Some(page) => {
                                if element_id.1 {
                                    page.stack.register_reference(
                                        import_item.from_import as usize,
                                        element_id.0,
                                    );
                                }
                            }
                            None => {
                                pub fn dumper(thread: &thread::Thread) -> String {
                                    let mut dump_data = "\r\n---\n\r".to_owned();
                                    let mut stack_dump = String::new();
                                    for stack in thread.pages.clone() {
                                        let mut headers = String::new();

                                        for item in stack.1.headers {
                                            headers +=
                                                &format!("\t\t\t{:#04x} : {}\n", item.0, item.1)
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

                                let copy_dump = dumper(&*thread);
                                panic!(
                                    "UNEXPECTED RUNTIME BEHAVIOUR Pageid: {}\n\n: {}",
                                    page_id, copy_dump
                                )
                            }
                        }
                    }
                    //ret

                    Some((import_item.from_import as usize, import_item.public))
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

                    pub fn dumper(thread: &thread::Thread) -> String {
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

                    let copy_dump = dumper(&*thread);
                    let type_id = thread.glb_look_up_for_item_by_name(&type_name);
                    match thread.pages.get_mut(&page_id) {
                        Some(page) => match type_id {
                            Some(type_element) => {
                                let rtype = match type_element {
                                    stack::StackElements::Class(class) => {
                                        stack::StackElement::Type(class.id)
                                    }
                                    stack::StackElements::Generic(generic) => {
                                        stack::StackElement::Generic(generic.id)
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
                                Some((element_id, variable.public))
                            }
                            None => {
                                pub fn dumper(thread: &thread::Thread) -> String {
                                    let mut dump_data = "\r\n---\n\r".to_owned();
                                    let mut stack_dump = String::new();
                                    for stack in thread.pages.clone() {
                                        let mut headers = String::new();

                                        for item in stack.1.headers {
                                            headers +=
                                                &format!("\t\t\t{:#04x} : {}\n", item.0, item.1)
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

                                panic!(
                                    "?? {:#?}, {:#?} - {:#04x}\n\nDUMP: {}",
                                    type_id, &type_name, page_id as u64, copy_dump
                                );
                                panic!("UNEXPECTED RUNTIME ERROR");
                            }
                        },
                        None => panic!("UNEXPECTED RUNTIME ERROR: CANNOT FIND PAGE"),
                    }
                }
                definite::items::Collecting::Function(_) => Some((0, false)),
                definite::items::Collecting::ForLoop(_) => Some((0, false)),
                definite::items::Collecting::Condition(_) => Some((0, false)),
                definite::items::Collecting::Class(class) => match thread.pages.get_mut(&page_id) {
                    Some(page) => {
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

                        let page_id = current_pages_len + 1;

                        let mut generics = Vec::new();
                        let mut child_headers: BTreeMap<usize, String> = BTreeMap::new();
                        let mut child_stack = stack::Stack::new(page_id);
                        for (id, generic) in class.generic_definings.into_iter().enumerate() {
                            child_headers.insert(id, generic.name.clone());
                            generics.push(id);
                            child_stack.register_generic(id);
                        }

                        let element_id = page.stack.register_class(page_id, generics);

                        page.headers.insert(element_id, class.name);

                        thread.pages.insert(
                            page_id as u64,
                            thread::Page {
                                page_id: page_id as u64,
                                headers: child_headers,
                                heap: heap::Heap::new(),
                                stack: child_stack,
                                step: page_id as usize,
                            },
                        );

                        for item in class_items {
                            loop_item(thread, item, page_id as u64);
                        }
                        Some((element_id, class.public))
                    }
                    None => panic!("CANNOT FIND PAGE"),
                },
                definite::items::Collecting::Ret(_) => Some((0, false)),
                definite::items::Collecting::Constructor(_) => Some((0, false)),
                definite::items::Collecting::Caller(_) => Some((0, false)),
                definite::items::Collecting::Import(import) => {
                    if !thread.pages.contains_key(&(import.id as u64)) {
                        thread.pages.insert(
                            import.id.clone() as u64,
                            thread::Page {
                                page_id: import.id as u64,
                                headers: BTreeMap::new(),
                                heap: heap::Heap::new(),
                                stack: stack::Stack::new(import.id.clone() as usize),
                                step: 0,
                            },
                        );
                    }
                    Some((import.id as usize, import.public))
                }
                definite::items::Collecting::FileKey(_) => Some((0, false)),
                definite::items::Collecting::Getter(_) => Some((0, false)),
                definite::items::Collecting::Setter(_) => Some((0, false)),
                definite::items::Collecting::NativeClass => Some((0, false)),
                definite::items::Collecting::ValueCall(_) => Some((0, false)),
                definite::items::Collecting::Enum(_) => Some((0, false)),
                definite::items::Collecting::NativeFunction(_) => Some((0, false)),
                definite::items::Collecting::None => Some((0, false)),
            }
        }

        let mut main_thread = thread::Thread::new(0, thread_controller);

        for item in code {
            //Split code to pages
            loop_item(&mut main_thread, item, 1);
        }

        /*

        for element in code {
            return ();
            match element {
                definite::items::Collecting::ImportItem(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'Function' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::Variable(e) => {
                    //headers.insert(main_stack.register_variable(
                    //    stack::StackElement::Type(0),
                    //    Some(add_data_to_heap(e.value).clone()),
                    //    e.dynamic,
                    //),e.name);
                }
                definite::items::Collecting::Function(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'Function' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::ForLoop(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'ForLoop' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::Condition(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'Condition' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::Class(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'Class' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::Ret(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'Ret' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::Constructor(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'Constructor' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::Caller(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'Caller' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::Import(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'Import' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::FileKey(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'FileKey' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::Getter(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'Getter' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::Setter(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'Setter' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::NativeClass => {
                    #[cfg(feature = "std")]
                    std::println!("'NativeClass' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::ValueCall(value) => match value {
                    definite::types::Types::Integer(_) => (),
                    definite::types::Types::Float(_) => (),
                    definite::types::Types::Bool(_) => (),
                    definite::types::Types::String(_) => (),
                    definite::types::Types::Char(_) => (),
                    definite::types::Types::Collective(_) => (),
                    definite::types::Types::Reference(_) => (),
                    definite::types::Types::Cloak(_) => (),
                    definite::types::Types::Array(_) => (),
                    definite::types::Types::ArrowFunction(_) => (),
                    definite::types::Types::ConstructedClass(_) => {
                        #[cfg(feature = "std")]
                        std::println!("'ConstructedClass' IS NOT SUPPORTED ELEMENT");
                        ()
                    }
                    definite::types::Types::Operator(operator_data) => {
                        match operator_data.operator {
                            definite::types::operator::Operators::ComparisonType(
                                comparison_type,
                            ) => {
                                #[cfg(feature = "std")]
                                std::println!(
                                    "Operator->'ComparisonType' IS NOT SUPPORTED ELEMENT"
                                );
                                ()
                            }
                            definite::types::operator::Operators::LogicalType(logical_type) => {
                                #[cfg(feature = "std")]
                                std::println!("Operator->'LogicalType' IS NOT SUPPORTED ELEMENT");
                                ()
                            }
                            definite::types::operator::Operators::ArithmeticType(
                                arithmetic_type,
                            ) => {
                                match arithmetic_type {
                                    definite::types::arithmetic_type::ArithmeticOperators::Addition => {
                                        //heap.values.get_mut()

                                        match *operator_data.first {
                                            definite::types::Types::Integer(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Integer') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::Float(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Float') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::Bool(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Bool') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::String(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('String') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::Char(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Char') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::Collective(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Collective') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::Reference(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Reference') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::Operator(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Operator') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::Cloak(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Cloak') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::Array(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Array') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::ArrowFunction(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('ArrowFunction') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::ConstructedClass(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('ConstructedClass') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::FunctionCall(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('FunctionCall') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::Void => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Void') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::NullResolver(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('NullResolver') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::Negative(_) => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Negative') IS NOT SUPPORTED ELEMENT");
                                            },
                                            definite::types::Types::VariableType(e) => {
                                                panic!("{:#?}", e)

                                            },
                                            definite::types::Types::Null => {
                                                #[cfg(feature = "std")]
                                                std::println!("Operator->Addition->First('Null') IS NOT SUPPORTED ELEMENT");
                                            },
                                        }

                                    },
                                    definite::types::arithmetic_type::ArithmeticOperators::Subtraction => {
                                        #[cfg(feature = "std")]
                                        std::println!("Operator->ArithmeticType->'Addition' IS NOT SUPPORTED ELEMENT");
                                        ()
                                    },
                                    definite::types::arithmetic_type::ArithmeticOperators::Multiplication => {
                                        #[cfg(feature = "std")]
                                        std::println!("Operator->ArithmeticType->'Addition' IS NOT SUPPORTED ELEMENT");
                                        ()
                                    },
                                    definite::types::arithmetic_type::ArithmeticOperators::Exponentiation => {
                                        #[cfg(feature = "std")]
                                        std::println!("Operator->ArithmeticType->'Addition' IS NOT SUPPORTED ELEMENT");
                                        ()
                                    },
                                    definite::types::arithmetic_type::ArithmeticOperators::Division => {
                                        #[cfg(feature = "std")]
                                        std::println!("Operator->ArithmeticType->'Addition' IS NOT SUPPORTED ELEMENT");
                                        ()
                                    },
                                    definite::types::arithmetic_type::ArithmeticOperators::Modulus => {
                                        #[cfg(feature = "std")]
                                        std::println!("Operator->ArithmeticType->'Addition' IS NOT SUPPORTED ELEMENT");
                                        ()
                                    },
                                    definite::types::arithmetic_type::ArithmeticOperators::Null => {
                                        #[cfg(feature = "std")]
                                        std::println!("Operator->ArithmeticType->'Addition' IS NOT SUPPORTED ELEMENT");
                                        ()
                                    },
                                }
                            }
                            definite::types::operator::Operators::Null => {
                                panic!("UNEXPECTED PARSER RESPONSE");
                            }
                        }
                        ()
                    }
                    definite::types::Types::FunctionCall(_) => {
                        #[cfg(feature = "std")]
                        std::println!("'FunctionCall' IS NOT SUPPORTED ELEMENT");
                        ()
                    }
                    definite::types::Types::Void => {
                        #[cfg(feature = "std")]
                        std::println!("'Void' IS NOT SUPPORTED ELEMENT");
                        ()
                    }
                    definite::types::Types::NullResolver(_) => {
                        #[cfg(feature = "std")]
                        std::println!("'NullResolver' IS NOT SUPPORTED ELEMENT");
                        ()
                    }
                    definite::types::Types::Negative(_) => {
                        #[cfg(feature = "std")]
                        std::println!("'Negative' IS NOT SUPPORTED ELEMENT");
                        ()
                    }
                    definite::types::Types::VariableType(_) => {
                        #[cfg(feature = "std")]
                        std::println!("'VariableType' IS NOT SUPPORTED ELEMENT");
                        ()
                    }
                    definite::types::Types::Null => {
                        #[cfg(feature = "std")]
                        std::println!("'Null' IS NOT SUPPORTED ELEMENT");
                        ()
                    }
                },
                definite::items::Collecting::Enum(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'Enum' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::NativeFunction(_) => {
                    #[cfg(feature = "std")]
                    std::println!("'NativeFunction' IS NOT SUPPORTED ELEMENT");
                    ()
                }
                definite::items::Collecting::None => {
                    #[cfg(feature = "std")]
                    std::println!("'None' IS NOT SUPPORTED ELEMENT");
                    ()
                }
            }
        }

        */
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
                    "\t---\n\tPage {:#04x}:\n\t\tHeaders:\n{}\n\t\tStack:\n\t{}\n\t\tHEAP:\n{}\n",
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
