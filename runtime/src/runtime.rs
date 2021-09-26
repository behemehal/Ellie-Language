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
        let mut pages: BTreeMap<u64, thread::Page> = BTreeMap::new();

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

        pub fn loop_item(
            pages: &mut BTreeMap<u64, thread::Page>,
            item: definite::items::Collecting,
            page_id: u64,
        ) {
            let current_pages_len = pages.len();
            match pages.get_mut(&page_id) {
                Some(page) => match item {
                    definite::items::Collecting::ImportItem(e) => {
                        let child_page_id = page.page_id.clone();
                        loop_item(pages, *e.item, child_page_id);
                    }
                    definite::items::Collecting::Variable(variable) => {
                        page.headers.insert(
                            page.stack.register_variable(
                                stack::StackElement::Type(0),
                                Some(add_data_to_heap(&mut page.heap, variable.value).clone()),
                                variable.dynamic,
                            ),
                            variable.name,
                        );
                    }
                    definite::items::Collecting::Function(function) => {
                        std::println!("'Function' NOT YET IMPLEMENTED, REFERENCED HEAP REQUIRED");
                        ()
                    },
                    definite::items::Collecting::ForLoop(_) => todo!(),
                    definite::items::Collecting::Condition(_) => todo!(),
                    definite::items::Collecting::Class(class) => {
                        let generics = class
                            .generic_definings
                            .clone()
                            .into_iter()
                            .map(|x| {
                                let id = page.headers.len();
                                page.headers.insert(id, x.name);
                                id
                            })
                            .collect::<Vec<_>>();

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
                        page.headers
                            .insert(page.stack.register_class(page_id, generics), class.name);
                        let mut child_headers: BTreeMap<usize, String> = BTreeMap::new();

                        for (id, generic) in class.generic_definings.into_iter().enumerate() {
                            child_headers.insert(id, generic.name);
                        }

                        pages.insert(
                            page_id as u64,
                            thread::Page {
                                page_id: page_id as u64,
                                headers: child_headers,
                                heap: heap::Heap::new(),
                                stack: stack::Stack::new(page_id),
                                step: page_id as usize,
                            },
                        );

                        for item in class_items {
                            loop_item(pages, item, page_id as u64);
                        }
                    }
                    definite::items::Collecting::Ret(_) => todo!(),
                    definite::items::Collecting::Constructor(_) => {
                        std::println!("'Constructor' NOT YET IMPLEMENTED");
                        ()
                    }
                    definite::items::Collecting::Caller(_) => {
                        std::println!("'Caller' NOT YET IMPLEMENTED");
                        ()
                    }
                    definite::items::Collecting::Getter(_) => {
                        std::println!("'Getter' NOT YET IMPLEMENTED");
                        ()
                    }
                    definite::items::Collecting::Setter(_) => {
                        std::println!("'Setter' NOT YET IMPLEMENTED");
                        ()
                    }
                    definite::items::Collecting::NativeClass => {
                        std::println!("'NativeClass' NOT YET IMPLEMENTED");
                        ()
                    }
                    definite::items::Collecting::ValueCall(_) => {
                        std::println!("'ValueCall' NOT YET IMPLEMENTED");
                        ()
                    }
                    definite::items::Collecting::Enum(_) => {
                        std::println!("'Enum' NOT YET IMPLEMENTED");
                        ()
                    }
                    definite::items::Collecting::NativeFunction(_) => {
                        std::println!("'NativeFunction' NOT YET IMPLEMENTED");
                        ()
                    }
                    _ => () //Ignore rest of the elements, because they do not have a corresponding task to do
                },
                None => match item {
                    definite::items::Collecting::ImportItem(e) => {
                        let stack_id = pages.len() + 1;
                        pages.insert(
                            e.resolution_id.clone(),
                            thread::Page {
                                page_id: e.resolution_id,
                                headers: BTreeMap::new(),
                                heap: heap::Heap::new(),
                                stack: stack::Stack::new(stack_id),
                                step: page_id as usize,
                            },
                        );
                    }
                    definite::items::Collecting::Import(_) => (),
                    definite::items::Collecting::FileKey(_) => (),
                    definite::items::Collecting::None => (),
                    _ => {
                        //Page required elements
                        panic!(
                                "UNEXPECTED PARSER BEHAVIOUR, PAGE '{}' SHOULD HAVE BEEN EXISTED; {:#?}",
                                page_id,
                                item
                            )
                    }
                },
            }
        }

        for item in code {
            //Split code to pages
            loop_item(&mut pages, item, 0);
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
        let main_thread = thread::Thread::new(0, pages, thread_controller);
        self.threads.push(main_thread);
    }

    pub fn dump(&self) -> String {
        let mut dump_data = "DUMP:\r\n---\n\r".to_owned();
        for thread in &self.threads {
            let mut stack_dump = String::new();

            let thread_id = thread.id.clone();

            for stack in thread.pages.clone() {
                stack_dump += &format!(
                    "Stack {:#04x}:\n\r{}\n\rHEAP:\n\r{}",
                    stack.0,
                    stack.1.stack.dump(),
                    stack.1.heap.clone().dump()
                );
            }

            dump_data += &format!(
                "Thread {:#04x}:\n\rSTACKS:\n\r{}\n\r\n\r---",
                thread_id, stack_dump
            );
        }
        dump_data
    }
}
