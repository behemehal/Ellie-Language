pub mod utils;

use std::fs::File;
use std::time::Instant;

use ellie_engine::ellie_bytecode::assembler::{AssembleResult, Assembler, PlatformAttributes};
use ellie_engine::ellie_core::defs::{DebugHeader, DebugInfo, PlatformArchitecture};
use ellie_engine::ellie_vm::program::VmProgram;
use ellie_engine::ellie_vm::utils::ThreadExit;
use ellie_engine::ellie_vm::{
    channel::{EllieModule, FunctionElement, ModuleElements, ModuleManager},
    program::Program,
    raw_type::StaticRawType,
    thread::{Isolate, Thread},
    utils::{ProgramReader, VmNativeAnswer, VmNativeCallParameters},
};
use ellie_engine::{parseText, vm::RFile};
use utils::VecReader;

use crate::utils::{render_text, save_file};

fn run(program: Program, assembler_result: AssembleResult) {
    let main = program.main.clone();
    let mut module_manager = ModuleManager::new();

    let mut vm_program = VmProgram::new_from_vector(program.instructions);

    module_manager.register_module(EllieModule::new("ellieStd".to_owned(), 0));

    let println = match assembler_result
        .native_exports
        .iter()
        .find(|x| x.name == "println")
    {
        Some(e) => e.hash,
        None => 99,
    };

    module_manager
        .get_module(0)
        .unwrap()
        .register_element(ModuleElements::Function(FunctionElement {
            name: "println".to_owned(),
            hash: println,
            callback: Box::new(|thread_info, params| match &params[0] {
                VmNativeCallParameters::Static(_) => VmNativeAnswer::RuntimeError(
                    "println: Expected string, given static argument".to_owned(),
                ),
                VmNativeCallParameters::Dynamic(raw_type) => match raw_type.type_id.id {
                    6 => {
                        let str = raw_type.to_string();
                        println!("{}", str);
                        VmNativeAnswer::Ok(VmNativeCallParameters::Static(
                            StaticRawType::from_void(),
                        ))
                    }
                    _ => VmNativeAnswer::RuntimeError(format!(
                        "println: Expected string, given {:?}",
                        raw_type
                    )),
                },
            }),
        }));

    //let spawn_thread = native_calls.iter().find(|x| x.name == "spawn_thread");

    /*
    let read_line = native_calls.iter().find(|x| x.name == "read_line").unwrap();
    let spawn_thread = native_calls
        .iter()
        .find(|x| x.name == "spawn_thread")
        .unwrap();
    let panic_fn = native_calls
        .iter()
        .find(|x| x.name == "panic")
        .unwrap();



    ellie_mod.register_element(ModuleElements::Function(FunctionElement {
        name: "read_line".to_owned(),
        hash: read_line.hash,
        callback: Box::new(|thread_info, params| {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            VmNativeAnswer::Ok(VmNativeCallParameters::Dynamic(RawType::string(
                input.chars().collect::<Vec<_>>(),
            )))
        }),
    }));

    ellie_mod.register_element(ModuleElements::Function(FunctionElement {
        name: "spawn_thread".to_owned(),
        hash: spawn_thread.hash,
        callback: Box::new(move |thread_info, params| match &params[0] {
            VmNativeCallParameters::Static(e) => {
                if e.type_id.is_int() {
                    //let hash = e.to_int() as usize;
                    //let fn_main = match vm.generate_main_from_function(hash) {
                    //    Ok(e) => e,
                    //    Err(e) => panic!("Malformed call error code: {:#?}", e),
                    //};
                    //panic!("{:?} {:?}", hash, fn_main);
                    //vm.generate_main_from_function(0);
                    VmNativeAnswer::Ok(VmNativeCallParameters::Static(StaticRawType::void()))
                } else {
                    VmNativeAnswer::RuntimeError(format!(
                        "spawn_thread: Expected int, given {} argument",
                        e.type_id.id
                    ))
                }
            }
            VmNativeCallParameters::Dynamic(e) => VmNativeAnswer::RuntimeError(format!(
                "spawn_thread: Expected static, given dynamic argument: {:?}",
                e
            )),
        }),
    }));

    ellie_mod.register_element(ModuleElements::Function(FunctionElement {
        name: "panic".to_owned(),
        hash: panic_fn.hash,
        callback: Box::new(|thread_info, params| match &params[0] {
            VmNativeCallParameters::Static(_) => VmNativeAnswer::RuntimeError(
                "panic: Expected string, given static argument".to_owned(),
            ),
            VmNativeCallParameters::Dynamic(raw_type) => match raw_type.type_id.id {
                6 => {
                    let str = raw_type.to_string();
                    VmNativeAnswer::RuntimeError(format!("Panic: {}", str))
                }
                _ => VmNativeAnswer::RuntimeError(format!(
                    "println: Expected string, given {}",
                    raw_type.type_id.id
                )),
            },
        }),
    })); */

    let isolate = Isolate::new();
    let mut thread = Thread::new(main.hash, PlatformArchitecture::B32, isolate);
    thread.build_thread(main.clone());
    let output = thread.run(&mut module_manager, &vm_program);
    match output {
        ThreadExit::Panic(panic) => {
            println!(
                "\n\n\
THREAD PANIC
============
Reason: {:?}
CodeLoc: {:?}
============
STACK DUMP:
\n
{}
\n
============
HEAP DUMP:
\n
{}
\n
============\n\n",
                panic.reason,
                panic.code_location,
                thread.isolate.stack_dump(),
                thread.isolate.heap_dump(),
            );
            for frame in panic.stack_trace {
                let coresponding_header = assembler_result
                    .debug_headers
                    .iter()
                    .find(|x| frame.pos >= x.start_end.0 && frame.pos <= x.start_end.1);

                match coresponding_header {
                    Some(e) => {
                        fn get_real_path(
                            debug_header: &DebugHeader,
                            debug_file: &DebugInfo,
                        ) -> String {
                            let module_name = debug_header
                                .module_name
                                .split("<ellie_module_")
                                .nth(1)
                                .unwrap()
                                .split(">")
                                .nth(0)
                                .unwrap();
                            let module_path = debug_file
                                .module_map
                                .iter()
                                .find(|map| module_name == map.module_name);
                            let real_path = match module_path {
                                Some(module_path) => match &module_path.module_path {
                                    Some(module_path) => {
                                        let new_path = debug_header.module_name.clone();
                                        let starter_name =
                                            format!("<ellie_module_{}>", module_name);
                                        new_path.replace(&starter_name, &module_path)
                                    }
                                    None => debug_header.module_name.clone(),
                                },
                                None => debug_header.module_name.clone(),
                            };
                            real_path
                        }

                        let real_path = get_real_path(
                            e,
                            &DebugInfo {
                                module_map: assembler_result.module_info.module_maps.clone(),
                                debug_headers: assembler_result.debug_headers.clone(),
                            },
                        );

                        println!(
                            "    at {}:{}:{}",
                            real_path,
                            e.pos.range_start.0 + 1,
                            e.pos.range_start.1 + 1,
                        );
                    }
                    None => {
                        println!("    at {:?}:{}", frame.caller, frame.pos);
                    }
                }
            }
        }
        ThreadExit::ExitGracefully => {
            /* println!(
                                "Thread Exited Gracefully\n
            ============
            STACK DUMP:
            \n
            {}
            \n
            ============
            HEAP DUMP:
            \n
            {}
            \n
            ============\n\n",
                                vm.stack_dump(),
                                vm.heap_dump()
                            );
                         */
        }
    }
}

fn run_from_file() -> Program {
    let mut file = File::open(
        "C:\\Users\\ahmet\\Desktop\\Projects\\Ellie-Language\\tools\\debug_files\\test.eic",
    )
    .unwrap();
    let mut reader = RFile::new(&mut file);
    let mut program_reader = ProgramReader::new(&mut reader);
    let mut program = Program::new();
    program.build_from_reader(&mut program_reader).unwrap();
    program
}

fn get_program_from_result(assembler_result: &AssembleResult) -> Program {
    let mut program_arr = assembler_result.render_binary_to_vector();
    let mut binding = VecReader::new(&mut program_arr);
    let mut r_file = RFile::new(&mut binding);
    let mut program_reader = ProgramReader::new(&mut r_file);
    let mut program = Program::new();
    program.build_from_reader(&mut program_reader).unwrap();
    program
}

fn compile() -> AssembleResult {
    let compile_output = parseText!(
        "
        @dont_fix_variant=true;
        class void {}
        @dont_fix_variant=true;
        class function {}
        @dont_fix_variant=true;
        class int {}
        @dont_fix_variant=true;
        class float {}
        @dont_fix_variant=true;
        class double {}
        @dont_fix_variant=true;
        class string {}
        @dont_fix_variant=true;
        class bool {}
        @dont_fix_variant=true;
        class array {}
        @dont_fix_variant=true;
        class nullAble {}
        
        fn println(s: string);
        fn panic(reason: string);

        fn fib(n: int) : int {
            if n < 2 {
                ret n;
            }
            ret fib(n - 1) + fib(n - 2);
        }

        class Age {
            co(num);
            v num : int;
            pub fn age() {
                println(\"Age access\");
            }
        }

        class Human {
            co(name, age);
            v name : string;
            v age : Age;
        }



        //fn read_line() : string;
        //fn spawn_thread(f: @():void);

        fn threadSpawn(s: string);
        
        //fn spaned() {
        //    println(\"Spaned\");
        //}

        fn main(d: double) {
            v age = new Age(1);
            v human = new Human(\"Ahmet\", age);
            println(\"age: \" + human.age.num);
            ((human.age.age();
            //panic(\"Let's see\");
        }
    "
    )
    .unwrap();
    let mut assembler = Assembler::new(
        compile_output.module,
        PlatformAttributes {
            architecture: PlatformArchitecture::B32, //64 Bit Limit
            memory_size: 512000,                     //512kb memory limit
        },
    );
    assembler.assemble(vec![])
}

fn main() {
    let time = Instant::now();
    let assembler_result = compile();
    render_text(&assembler_result);
    //println!("{:#?}", assembler_result.1);
    save_file(&assembler_result);

    let program = get_program_from_result(&assembler_result);
    //let program = run_from_file();
    //run(program, assembler_result.1);
    run(program, assembler_result);
    println!("Complete in {}ms", time.elapsed().as_secs_f32());
}
