use ellie_engine::ellie_vm_gen2::{
    module::{Gen2Module, Gen2Value, ModuleManager},
    program::Program,
    thread::{Thread, VmExit},
};
use ellie_engine::ellie_core::bytecode::TypeId;
use std::ffi::CString;

pub fn build_core_module(program: &Program) -> Gen2Module {
    let mut m = Gen2Module::new("ellieCore");

    for trace in &program.native_traces {
        let hash = trace.function_hash;
        let name = trace.function_name.clone();

        match name.as_str() {
            "println" => {
                m.register_fn("println", hash, move |args, heap| {
                    if args.is_empty() {
                        eprintln!();
                        return Gen2Value::Void;
                    }
                    let arg = args[0];
                    let text = if arg.type_id == TypeId::Reference {
                        let idx: usize = arg.into();
                        heap.get_string(idx).unwrap_or("").to_string()
                    } else {
                        raw_int_to_string(arg)
                    };
                    eprintln!("{}", text);
                    Gen2Value::Void
                });
            }
            "panic" => {
                m.register_fn("panic", hash, move |args, heap| {
                    let msg = if !args.is_empty() && args[0].type_id == TypeId::Reference {
                        let idx: usize = args[0].into();
                        heap.get_string(idx).unwrap_or("panic").to_string()
                    } else {
                        "panic".to_string()
                    };
                    eprintln!("Panic: {}", msg);
                    std::process::exit(1);
                });
            }
            "timestamp" => {
                m.register_fn("timestamp", hash, |_args, _heap| {
                    use std::time::SystemTime;
                    let ms = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as isize;
                    Gen2Value::Raw(ms.into())
                });
            }
            "sleep_ms" => {
                m.register_fn("sleep_ms", hash, |args, _heap| {
                    let ms: isize = if args.is_empty() { 0 } else { args[0].into() };
                    unsafe {
                        let lib = libloading::Library::new("kernel32.dll")
                            .expect("failed to load kernel32.dll");
                        let sleep_fn: libloading::Symbol<unsafe extern "system" fn(u32)> =
                            lib.get(b"Sleep\0").expect("failed to find Sleep");
                        sleep_fn(ms.max(0) as u32);
                    }
                    Gen2Value::Void
                });
            }
            "message_box" => {
                m.register_fn("message_box", hash, |args, heap| {
                    let text = str_arg(args, 0, heap);
                    let caption = str_arg(args, 1, heap);
                    let flags: isize = if args.len() > 2 { args[2].into() } else { 0 };
                    unsafe {
                        let lib = libloading::Library::new("user32.dll")
                            .expect("failed to load user32.dll");
                        let mb: libloading::Symbol<
                            unsafe extern "system" fn(
                                *mut std::ffi::c_void,
                                *const i8,
                                *const i8,
                                u32,
                            ) -> i32,
                        > = lib.get(b"MessageBoxA\0").expect("failed to find MessageBoxA");
                        let text_c = CString::new(text).unwrap_or_default();
                        let caption_c = CString::new(caption).unwrap_or_default();
                        let result = mb(
                            std::ptr::null_mut(),
                            text_c.as_ptr(),
                            caption_c.as_ptr(),
                            flags as u32,
                        );
                        Gen2Value::Raw((result as isize).into())
                    }
                });
            }
            "load_library_handle" => {
                m.register_fn("load_library_handle", hash, |args, heap| {
                    let path = str_arg(args, 0, heap);
                    unsafe {
                        let lib = libloading::Library::new(&path);
                        match lib {
                            Ok(lib) => {
                                // Leak the library so it stays loaded; return raw handle as isize
                                let handle = libloading::os::windows::Library::from(lib).into_raw() as isize;
                                Gen2Value::Raw(handle.into())
                            }
                            Err(e) => {
                                eprintln!("[gen2 vm] load_library_handle failed for '{}': {}", path, e);
                                Gen2Value::Raw(0isize.into())
                            }
                        }
                    }
                });
            }
            "call_library_function" => {
                m.register_fn("call_library_function", hash, |args, heap| {
                    if args.len() < 2 {
                        return Gen2Value::Void;
                    }
                    let handle_val: isize = args[0].into();
                    let fn_name = str_arg(args, 1, heap);
                    // args[2] is the args array (heap ref to ClassInst)
                    // Each element is a ClassInst with field[0]=type_tag(heap ref string), field[1]=value

                    // Collect the typed args from the heap array
                    let fn_args = if args.len() > 2 {
                        collect_fn_args(args[2], heap)
                    } else {
                        vec![]
                    };

                    unsafe {
                        dispatch_windows_fn(handle_val, &fn_name, &fn_args, heap)
                    }
                });
            }
            _ => {
                // Unknown native — register a stub that panics
                let fn_name = name.clone();
                m.register_fn(name.clone(), hash, move |_args, _heap| {
                    eprintln!("[gen2 vm] Unimplemented native: {}", fn_name);
                    Gen2Value::Void
                });
            }
        }
    }

    m
}

fn raw_int_to_string(r: ellie_engine::ellie_core::bytecode::RawType) -> String {
    let v: isize = r.into();
    v.to_string()
}

fn str_arg(
    args: &[ellie_engine::ellie_core::bytecode::RawType],
    idx: usize,
    heap: &ellie_engine::ellie_vm_gen2::heap::Heap,
) -> String {
    if idx >= args.len() {
        return String::new();
    }
    let r = args[idx];
    if r.type_id == TypeId::Reference {
        let i: usize = r.into();
        heap.get_string(i).unwrap_or("").to_string()
    } else {
        raw_int_to_string(r)
    }
}

/// A typed argument extracted from an Ellie `(string, dyn)` cloak/tuple
enum FnArg {
    U32(u32),
    U64(u64),
    Pointer(Vec<u8>),  // null-terminated C string bytes
    I32(i32),
    I64(i64),
}

/// Walk the heap ClassInst array representing `[(string, dyn)]` and collect typed args.
fn collect_fn_args(
    arr_ref: ellie_engine::ellie_core::bytecode::RawType,
    heap: &ellie_engine::ellie_vm_gen2::heap::Heap,
) -> Vec<FnArg> {
    use ellie_engine::ellie_core::bytecode::TypeId;
    let mut result = Vec::new();

    // The array is a ClassInst where field 0 = length, fields 1..n = element refs
    // Each element is a ClassInst where field 0 = type_tag (heap ref), field 1 = value
    let arr_idx: usize = arr_ref.into();
    let zero = ellie_engine::ellie_core::bytecode::RawType {
        type_id: TypeId::Int,
        size: 0,
        data: [0; 8],
    };

    let n = {
        let len_raw = heap.get_field(arr_idx, 0).unwrap_or(zero);
        let v: isize = len_raw.into();
        v as usize
    };

    for i in 0..n {
        let elem_raw = match heap.get_field(arr_idx, i + 1) {
            Some(r) => r,
            None => continue,
        };
        if elem_raw.type_id != TypeId::Reference {
            continue;
        }
        let elem_idx: usize = elem_raw.into();

        // field 0 = type tag (heap string ref)
        let tag_raw = heap.get_field(elem_idx, 0).unwrap_or(zero);
        let tag = if tag_raw.type_id == TypeId::Reference {
            let ti: usize = tag_raw.into();
            heap.get_string(ti).unwrap_or("").to_string()
        } else {
            String::new()
        };

        // field 1 = value
        let val_raw = heap.get_field(elem_idx, 1).unwrap_or(zero);

        let arg = match tag.as_str() {
            "uint32" => {
                let v: isize = val_raw.into();
                FnArg::U32(v as u32)
            }
            "uint64" => {
                let v: isize = val_raw.into();
                FnArg::U64(v as u64)
            }
            "int32" => {
                let v: isize = val_raw.into();
                FnArg::I32(v as i32)
            }
            "int64" | "int" => {
                let v: isize = val_raw.into();
                FnArg::I64(v as i64)
            }
            "pointer" => {
                // Value is a string heap ref → convert to null-terminated bytes
                if val_raw.type_id == TypeId::Reference {
                    let si: usize = val_raw.into();
                    let s = heap.get_string(si).unwrap_or("");
                    let mut bytes = s.as_bytes().to_vec();
                    bytes.push(0); // null-terminate
                    FnArg::Pointer(bytes)
                } else {
                    let v: isize = val_raw.into();
                    FnArg::U64(v as u64)
                }
            }
            _ => {
                let v: isize = val_raw.into();
                FnArg::U64(v as u64)
            }
        };
        result.push(arg);
    }
    result
}

unsafe fn dispatch_windows_fn(
    handle_val: isize,
    fn_name: &str,
    fn_args: &[FnArg],
    heap: &ellie_engine::ellie_vm_gen2::heap::Heap,
) -> Gen2Value {
    use std::ffi::CString;
    use libloading::os::windows::Library as WinLib;

    let lib = WinLib::from_raw(handle_val);

    let c_name = match CString::new(fn_name) {
        Ok(s) => s,
        Err(_) => {
            std::mem::forget(lib);
            return Gen2Value::Void;
        }
    };

    // Convert FnArgs to u64 values for passing via raw function pointer
    let raw_args: Vec<u64> = fn_args.iter().map(|a| match a {
        FnArg::U32(v) => *v as u64,
        FnArg::U64(v) => *v,
        FnArg::I32(v) => *v as u64,
        FnArg::I64(v) => *v as u64,
        FnArg::Pointer(bytes) => bytes.as_ptr() as u64,
    }).collect();

    // Dispatch based on arg count using typed function pointer casts
    type Fn0 = unsafe extern "system" fn() -> isize;
    type Fn1 = unsafe extern "system" fn(u64) -> isize;
    type Fn2 = unsafe extern "system" fn(u64, u64) -> isize;
    type Fn3 = unsafe extern "system" fn(u64, u64, u64) -> isize;
    type Fn4 = unsafe extern "system" fn(u64, u64, u64, u64) -> isize;
    type Fn5 = unsafe extern "system" fn(u64, u64, u64, u64, u64) -> isize;

    let fn_ptr_raw: *const () = match lib.get::<*const ()>(c_name.as_bytes_with_nul()) {
        Ok(sym) => *sym,
        Err(e) => {
            eprintln!("[gen2 vm] GetProcAddress failed for '{}': {}", fn_name, e);
            std::mem::forget(lib);
            return Gen2Value::Void;
        }
    };

    let result = match raw_args.len() {
        0 => std::mem::transmute::<*const (), Fn0>(fn_ptr_raw)(),
        1 => std::mem::transmute::<*const (), Fn1>(fn_ptr_raw)(raw_args[0]),
        2 => std::mem::transmute::<*const (), Fn2>(fn_ptr_raw)(raw_args[0], raw_args[1]),
        3 => std::mem::transmute::<*const (), Fn3>(fn_ptr_raw)(raw_args[0], raw_args[1], raw_args[2]),
        4 => std::mem::transmute::<*const (), Fn4>(fn_ptr_raw)(raw_args[0], raw_args[1], raw_args[2], raw_args[3]),
        _ => std::mem::transmute::<*const (), Fn5>(fn_ptr_raw)(raw_args[0], raw_args[1], raw_args[2], raw_args[3], raw_args[4]),
    };

    std::mem::forget(lib); // keep library loaded
    Gen2Value::Raw(result.into())
}

pub fn run_gen2(program: Program) {
    let mut modules = ModuleManager::new();
    let core_module = build_core_module(&program);
    modules.register_module(core_module);

    let thread = Thread::new();
    let (exit, _thread) = thread.run(&program, &modules);

    match exit {
        VmExit::Ok => {}
        VmExit::Panic(msg) => {
            eprintln!("\n[gen2 vm] Panic: {}", msg);
            std::process::exit(1);
        }
    }
}
