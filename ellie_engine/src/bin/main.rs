use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::panic;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

use ellie_bytecode::assembler::AssembleResult;
use ellie_bytecode::assembler::{Assembler, PlatformAttributes};
use ellie_core::defs::PlatformArchitecture;
use ellie_core::defs::VmNativeAnswer;
use ellie_core::defs::VmNativeCallParameters;
use ellie_core::raw_type::RawType;
use ellie_core::raw_type::StaticRawType;
use ellie_engine::parseText;
use ellie_engine::utils::CompileOutput;
use ellie_engine::vm::RFile;
use ellie_parser::parser::NativeCall;
use ellie_tokenizer::tokenizer::ResolvedImport;
use ellie_vm::channel::EllieModule;
use ellie_vm::channel::FunctionElement;
use ellie_vm::channel::ModuleElements;
use ellie_vm::channel::ModuleManager;
use ellie_vm::program::MainProgram;
use ellie_vm::program::Program;
use ellie_vm::utils::ProgramReader;
use ellie_vm::vm::VM;

struct VecReader<'a> {
    data: &'a mut Vec<u8>,
    pos: usize,
}

impl<'a> VecReader<'a> {
    fn new(data: &'a mut Vec<u8>) -> VecReader<'a> {
        VecReader { data: data, pos: 0 }
    }
}

impl<'a> Read for VecReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = std::cmp::min(buf.len(), self.data.len() - self.pos);
        let slice = &self.data[self.pos..self.pos + len];
        buf[..len].copy_from_slice(slice);
        self.pos += len;
        Ok(len)
    }
}

struct StringWrite {
    data: String,
}

impl StringWrite {
    fn new() -> StringWrite {
        StringWrite {
            data: String::new(),
        }
    }
}

impl Write for StringWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.push_str(std::str::from_utf8(buf).unwrap());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn run(program: Program, native_calls: Vec<NativeCall>) {
    let mut module_manager = ModuleManager::new();

    module_manager.register_module(EllieModule::new("ellieStd".to_owned(), 0));
    let main = program.main.clone();

    let mut vm = VM::new(PlatformArchitecture::B64);

    vm.load_program(program.instructions);

    let ellie_mod = module_manager.get_module(0).unwrap();

    let println = native_calls.iter().find(|x| x.name == "println").unwrap();
    ellie_mod.register_element(ModuleElements::Function(FunctionElement {
        name: "println".to_owned(),
        hash: println.hash,
        callback: Box::new(|thread_info, params| match &params[0] {
            VmNativeCallParameters::Static(_) => VmNativeAnswer::RuntimeError(
                "println: Expected string, given static argument".to_owned(),
            ),
            VmNativeCallParameters::Dynamic(raw_type) => match raw_type.type_id.id {
                6 => {
                    let str = raw_type.to_string();
                    println!("{}", str);
                    VmNativeAnswer::Ok(VmNativeCallParameters::Static(StaticRawType::void()))
                }
                _ => VmNativeAnswer::RuntimeError(format!(
                    "println: Expected string, given {:?}",
                    raw_type
                )),
            },
        }),
    }));
    let getTime = native_calls.iter().find(|x| x.name == "getTime").unwrap();

    ellie_mod.register_element(ModuleElements::Function(FunctionElement {
        name: "getTime".to_owned(),
        hash: getTime.hash,
        callback: Box::new(|thread_info, params| {
            let ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            VmNativeAnswer::Ok(VmNativeCallParameters::Static(StaticRawType::integer(
                (ms as isize).to_le_bytes().to_vec(),
            )))
        }),
    }));

    let spawn_thread = native_calls.iter().find(|x| x.name == "spawn_thread");

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

    vm.create_thread(main.clone()).unwrap();

    let output = vm.run_thread(main.hash, &mut module_manager);
    match output {
        Some(e) => match e {
            ellie_vm::utils::ThreadExit::Panic(panic) => {
                panic!(
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
                    vm.stack_dump(),
                    vm.heap_dump(),
                );
            }
            ellie_vm::utils::ThreadExit::ExitGracefully => {
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
        },
        None => todo!(),
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

fn get_program_from_result(assembler_result: AssembleResult) -> Program {
    let mut program_arr = assembler_result.render_binary_to_vector();
    let mut binding = VecReader::new(&mut program_arr);
    let mut r_file = RFile::new(&mut binding);
    let mut program_reader = ProgramReader::new(&mut r_file);
    let mut program = Program::new();
    program.build_from_reader(&mut program_reader).unwrap();
    program
}

fn render_text(assembler_result: &AssembleResult) {
    let mut assembly_code_string = StringWrite::new();
    assembler_result.alternate_render(&mut assembly_code_string);

    println!("{}", assembly_code_string.data);
}

fn save_file(assembler_result: &AssembleResult) {
    let mut assembly_code_string = StringWrite::new();
    assembler_result.alternate_render(&mut assembly_code_string);

    let mut file = File::create(
        "C:\\Users\\ahmet\\Desktop\\Projects\\Ellie-Language\\tools\\debug_files\\test.eic",
    )
    .unwrap();

    let mut dfile = File::create(
        "C:\\Users\\ahmet\\Desktop\\Projects\\Ellie-Language\\tools\\debug_files\\test.eig",
    )
    .unwrap();

    assembler_result.render_binary(&mut file, &mut dfile);
}

fn compile() -> (AssembleResult, Vec<NativeCall>) {
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

        //fn fib(n: int) : int {
        //    if n < 2 {
        //        ret n;
        //    }
        //    ret fib(n - 1) + fib(n - 2);
        //}

        //class Age {
        //    co(num);
        //    v num : int;
        //    pub fn age() : int {
        //        ret self.num;
        //    }
        //}

        //class Human {
        //    co(name, age);
        //    v name : string;
        //    v age : Age;
        //}

        fn println(s: string);
        fn getTime() : int;
        //fn read_line() : string;
        //fn panic(reason: string);
        fn spawn_thread(f: @():void);
        
        //fn spaned() {
        //    println(\"Spaned\");
        //}

        fn main() {
            //This doesnt work here!!
            fn spaned() {
                v test = 123;
            }
            v qtest = 1234;
            //spawn_thread(spaned);

            //v start_time = getTime();
            //println(\"StartTime: \" + (start_time));
            //fib(28);
            //v end_time = getTime();
            //println(\"Time: \" + (end_time - start_time));
            //println(\"EndTime: \" + (end_time - start_time));


            //v mname = \"Ahmet\";

            //mname += \"123\";

            //println(\"mname: \" + mname );
            
            //v arr = [mname];

            //arr[0] += \"123\";

            //println(arr[0]!);
            
            //v age = new Age(1);
            //v human = new Human(\"Ahmet\", age);
            //println(\"test: \" + human.age.age());
            //println(\"age: \" + human.age.num);
            //panic(\"Let's see\");
        }
    "
    )
    .unwrap();
    let native_map = compile_output.module.get_natives();
    let mut assembler = Assembler::new(
        compile_output.module,
        PlatformAttributes {
            architecture: PlatformArchitecture::B64, //64 Bit Limit
            memory_size: 512000,                     //512kb memory limit
        },
    );
    (assembler.assemble(vec![]), native_map)
}

fn main() {
    let time = Instant::now();
    let assembler_result = compile();
    render_text(&assembler_result.0);
    //println!("{:#?}", assembler_result.1);
    //save_file(&assembler_result.0);

    let program = get_program_from_result(assembler_result.0);
    //let program = run_from_file();
    //run(program, assembler_result.1);
    run(program, assembler_result.1);
    println!("Complete in {}ms", time.elapsed().as_secs_f32());
}
