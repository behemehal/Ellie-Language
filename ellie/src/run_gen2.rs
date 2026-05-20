use ellie_engine::ellie_core::bytecode::TypeId;
use ellie_engine::ellie_vm_gen2::{
    heap::Heap,
    module::{Gen2Module, Gen2Value, ModuleManager},
    program::Program,
    thread::{Thread, VmExit},
};
use std::path::Path;

// ── Native bridge loader ──────────────────────────────────────────────────────
//
// A native bridge is a cdylib (.dll / .so / .dylib) that exports:
//   extern "Rust" fn load_module() -> EllieModule
//
// We load it with libloading, call load_module(), then translate each
// EllieFunction into a Gen2Module function so the gen2 VM can call it.

fn load_bridge(path: &Path, program: &Program) -> Option<Gen2Module> {
    use ellie_native_bridge::rust::{EllieData, EllieModule, FunctionAnswer};

    let lib = unsafe {
        match libloading::Library::new(path) {
            Ok(l) => l,
            Err(e) => {
                eprintln!(
                    "[ellie] Failed to load bridge '{}': {}",
                    path.display(),
                    e
                );
                return None;
            }
        }
    };

    // Retrieve the EllieModule (the bridge leaks the lib intentionally)
    let ellie_mod: ellie_native_bridge::rust::EllieModule = unsafe {
        let load_fn: libloading::Symbol<extern "Rust" fn() -> EllieModule> =
            match lib.get(b"load_module") {
                Ok(f) => f,
                Err(e) => {
                    eprintln!(
                        "[ellie] Bridge '{}' missing load_module: {}",
                        path.display(),
                        e
                    );
                    return None;
                }
            };
        let m = load_fn();
        std::mem::forget(lib); // keep DLL loaded
        m
    };

    let mod_name = ellie_mod.name.to_string();
    let mut gen2_mod = Gen2Module::new(&mod_name);

    for bridge_fn in ellie_mod.functions {
        let fn_name = bridge_fn.name.to_string();

        // Find the native trace hash so the VM can look it up
        let hash = match program
            .native_traces
            .iter()
            .find(|t| t.function_name == fn_name)
        {
            Some(t) => t.function_hash,
            None => {
                eprintln!(
                    "[ellie] Bridge fn '{}' has no matching native trace — skipping",
                    fn_name
                );
                continue;
            }
        };

        let callback = bridge_fn.on_call;

        gen2_mod.register_fn(&fn_name, hash, move |args, heap| {
            // Convert RawType args → EllieData FunctionCallParameters
            let params: Vec<ellie_native_bridge::rust::FunctionCallParameter> = args
                .iter()
                .enumerate()
                .map(|(i, raw)| ellie_native_bridge::rust::FunctionCallParameter {
                    data: raw_to_ellie_data(*raw, heap),
                    memory_location: i,
                })
                .collect();

            match (callback)(params) {
                FunctionAnswer::Ok(data) => ellie_data_to_gen2(data, heap),
                FunctionAnswer::RuntimeError(msg) => {
                    eprintln!("[ellie bridge] RuntimeError: {}", msg);
                    Gen2Value::Void
                }
            }
        });
    }

    eprintln!(
        "[ellie] Loaded bridge '{}' ({})",
        mod_name,
        path.display()
    );
    Some(gen2_mod)
}

fn raw_to_ellie_data(
    raw: ellie_engine::ellie_core::bytecode::RawType,
    heap: &Heap,
) -> ellie_native_bridge::rust::EllieData {
    use ellie_native_bridge::rust::EllieData;
    match raw.type_id {
        TypeId::Int => {
            let v: isize = raw.into();
            EllieData::Integer(v.into())
        }
        TypeId::Bool => {
            let v: u8 = raw.into();
            EllieData::Bool(v != 0)
        }
        TypeId::Byte => {
            let v: u8 = raw.into();
            EllieData::Byte(v)
        }
        TypeId::Float => {
            let bits = u32::from_le_bytes(raw.data[..4].try_into().unwrap());
            EllieData::Float(f32::from_bits(bits))
        }
        TypeId::Double => {
            let bits = u64::from_le_bytes(raw.data);
            EllieData::Double(f64::from_bits(bits))
        }
        TypeId::Char => {
            let code = u32::from_le_bytes(raw.data[..4].try_into().unwrap());
            EllieData::Char(char::from_u32(code).unwrap_or('\0'))
        }
        TypeId::Reference => {
            let idx: usize = raw.into();
            let s = heap.get_string(idx).unwrap_or("").to_string();
            EllieData::String(s)
        }
        _ => EllieData::Null,
    }
}

fn ellie_data_to_gen2(
    data: ellie_native_bridge::rust::EllieData,
    heap: &mut Heap,
) -> Gen2Value {
    use ellie_native_bridge::rust::EllieData;
    match data {
        EllieData::Integer(i) => Gen2Value::Raw(i.as_isize.into()),
        EllieData::Bool(b) => {
            let mut r = zero_raw();
            r.type_id = TypeId::Bool;
            r.data[0] = b as u8;
            Gen2Value::Raw(r)
        }
        EllieData::Byte(b) => {
            let mut r = zero_raw();
            r.type_id = TypeId::Byte;
            r.data[0] = b;
            Gen2Value::Raw(r)
        }
        EllieData::Float(f) => {
            let mut r = zero_raw();
            r.type_id = TypeId::Float;
            r.data[..4].copy_from_slice(&f.to_bits().to_le_bytes());
            Gen2Value::Raw(r)
        }
        EllieData::Double(f) => {
            let mut r = zero_raw();
            r.type_id = TypeId::Double;
            r.data.copy_from_slice(&f.to_bits().to_le_bytes());
            Gen2Value::Raw(r)
        }
        EllieData::Char(c) => {
            let mut r = zero_raw();
            r.type_id = TypeId::Char;
            r.data[..4].copy_from_slice(&(c as u32).to_le_bytes());
            Gen2Value::Raw(r)
        }
        EllieData::String(s) => Gen2Value::Str(s),
        EllieData::Void => Gen2Value::Void,
        _ => Gen2Value::Void,
    }
}

fn zero_raw() -> ellie_engine::ellie_core::bytecode::RawType {
    ellie_engine::ellie_core::bytecode::RawType {
        type_id: TypeId::Int,
        size: 8,
        data: [0; 8],
    }
}

// ── Core module (println, timestamp, sleep_ms, etc.) ─────────────────────────
// Identical to ellievm/src/run_gen2.rs but lives here so ellie is self-contained.

pub fn build_core_module(program: &Program) -> Gen2Module {
    let mut m = Gen2Module::new("ellieCore");

    for trace in &program.native_traces {
        let hash = trace.function_hash;
        let name = trace.function_name.clone();

        match name.as_str() {
            "println" => {
                m.register_fn("println", hash, |args, heap| {
                    if args.is_empty() {
                        eprintln!();
                        return Gen2Value::Void;
                    }
                    let text = if args[0].type_id == TypeId::Reference {
                        let idx: usize = args[0].into();
                        heap.get_string(idx).unwrap_or("").to_string()
                    } else {
                        let v: isize = args[0].into();
                        v.to_string()
                    };
                    eprintln!("{}", text);
                    Gen2Value::Void
                });
            }
            "panic" => {
                m.register_fn("panic", hash, |args, heap| {
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
            "load_library_handle" => {
                m.register_fn("load_library_handle", hash, |args, heap| {
                    let path = str_arg(args, 0, heap);
                    unsafe {
                        let lib = libloading::Library::new(&path);
                        match lib {
                            Ok(lib) => {
                                let handle =
                                    libloading::os::windows::Library::from(lib).into_raw()
                                        as isize;
                                Gen2Value::Raw(handle.into())
                            }
                            Err(e) => {
                                eprintln!(
                                    "[ellie] load_library_handle failed for '{}': {}",
                                    path, e
                                );
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
                    let fn_args = if args.len() > 2 {
                        collect_fn_args(args[2], heap)
                    } else {
                        vec![]
                    };
                    unsafe { dispatch_ffi(handle_val, &fn_name, &fn_args) }
                });
            }
            "command_exists" => {
                m.register_fn("command_exists", hash, |args, heap| {
                    let cmd = str_arg(args, 0, heap);
                    let exists = command_exists_impl(&cmd);
                    let mut r = zero_raw();
                    r.type_id = TypeId::Bool;
                    r.data[0] = exists as u8;
                    Gen2Value::Raw(r)
                });
            }
            "run_command" => {
                m.register_fn("run_command", hash, |args, heap| {
                    let cmd = str_arg(args, 0, heap);
                    let cmd_args = if args.len() > 1 {
                        collect_string_array(args[1], heap)
                    } else {
                        vec![]
                    };
                    let code = std::process::Command::new(&cmd)
                        .args(&cmd_args)
                        .status()
                        .map(|s| s.code().unwrap_or(-1))
                        .unwrap_or(-1);
                    Gen2Value::Raw((code as isize).into())
                });
            }
            "run_command_silent" => {
                m.register_fn("run_command_silent", hash, |args, heap| {
                    use std::process::Stdio;
                    let cmd = str_arg(args, 0, heap);
                    let cmd_args = if args.len() > 1 {
                        collect_string_array(args[1], heap)
                    } else {
                        vec![]
                    };
                    let code = std::process::Command::new(&cmd)
                        .args(&cmd_args)
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .map(|s| s.code().unwrap_or(-1))
                        .unwrap_or(-1);
                    Gen2Value::Raw((code as isize).into())
                });
            }
            "run_command_output" => {
                m.register_fn("run_command_output", hash, |args, heap| {
                    let cmd = str_arg(args, 0, heap);
                    let cmd_args = if args.len() > 1 {
                        collect_string_array(args[1], heap)
                    } else {
                        vec![]
                    };
                    let out = std::process::Command::new(&cmd)
                        .args(&cmd_args)
                        .output()
                        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
                        .unwrap_or_default();
                    Gen2Value::Str(out)
                });
            }
            "get_env" => {
                m.register_fn("get_env", hash, |args, heap| {
                    let name = str_arg(args, 0, heap);
                    let val = std::env::var(&name).unwrap_or_default();
                    Gen2Value::Str(val)
                });
            }
            "set_env" => {
                m.register_fn("set_env", hash, |args, heap| {
                    let name = str_arg(args, 0, heap);
                    let value = str_arg(args, 1, heap);
                    unsafe { std::env::set_var(&name, &value) };
                    Gen2Value::Void
                });
            }
            "get_cwd" => {
                m.register_fn("get_cwd", hash, |_args, _heap| {
                    let cwd = std::env::current_dir()
                        .map(|p| p.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    Gen2Value::Str(cwd)
                });
            }
            _ => {
                let fn_name_clone = name.clone();
                m.register_fn(name, hash, move |_args, _heap| {
                    eprintln!("[ellie] Unimplemented native: {}", fn_name_clone);
                    Gen2Value::Void
                });
            }
        }
    }

    m
}

// ── FFI helpers ───────────────────────────────────────────────────────────────

fn str_arg(
    args: &[ellie_engine::ellie_core::bytecode::RawType],
    idx: usize,
    heap: &Heap,
) -> String {
    if idx >= args.len() {
        return String::new();
    }
    let r = args[idx];
    if r.type_id == TypeId::Reference {
        let i: usize = r.into();
        heap.get_string(i).unwrap_or("").to_string()
    } else {
        let v: isize = r.into();
        v.to_string()
    }
}

enum FnArg {
    U32(u32),
    U64(u64),
    Pointer(Vec<u8>),
    I32(i32),
    I64(i64),
}

fn collect_fn_args(
    arr_ref: ellie_engine::ellie_core::bytecode::RawType,
    heap: &Heap,
) -> Vec<FnArg> {
    let zero = zero_raw();
    let mut result = Vec::new();
    let arr_idx: usize = arr_ref.into();

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

        let tag_raw = heap.get_field(elem_idx, 0).unwrap_or(zero);
        let tag = if tag_raw.type_id == TypeId::Reference {
            let ti: usize = tag_raw.into();
            heap.get_string(ti).unwrap_or("").to_string()
        } else {
            String::new()
        };

        let val_raw = heap.get_field(elem_idx, 1).unwrap_or(zero);

        let arg = match tag.as_str() {
            "uint32" => FnArg::U32({ let v: isize = val_raw.into(); v as u32 }),
            "uint64" => FnArg::U64({ let v: isize = val_raw.into(); v as u64 }),
            "int32"  => FnArg::I32({ let v: isize = val_raw.into(); v as i32 }),
            "int64" | "int" => FnArg::I64({ let v: isize = val_raw.into(); v as i64 }),
            "pointer" => {
                if val_raw.type_id == TypeId::Reference {
                    let si: usize = val_raw.into();
                    let s = heap.get_string(si).unwrap_or("");
                    let mut bytes = s.as_bytes().to_vec();
                    bytes.push(0);
                    FnArg::Pointer(bytes)
                } else {
                    let v: isize = val_raw.into();
                    FnArg::U64(v as u64)
                }
            }
            _ => FnArg::U64({ let v: isize = val_raw.into(); v as u64 }),
        };
        result.push(arg);
    }
    result
}

fn command_exists_impl(cmd: &str) -> bool {
    if let Some(path_os) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path_os) {
            let candidate = dir.join(cmd);
            if candidate.is_file() {
                return true;
            }
            #[cfg(target_os = "windows")]
            {
                let with_exe = dir.join(format!("{}.exe", cmd));
                if with_exe.is_file() {
                    return true;
                }
                let with_cmd = dir.join(format!("{}.cmd", cmd));
                if with_cmd.is_file() {
                    return true;
                }
            }
        }
    }
    false
}

fn collect_string_array(
    arr_ref: ellie_engine::ellie_core::bytecode::RawType,
    heap: &Heap,
) -> Vec<String> {
    let arr_idx: usize = arr_ref.into();
    let zero = zero_raw();
    let n = {
        let len_raw = heap.get_field(arr_idx, 0).unwrap_or(zero);
        let v: isize = len_raw.into();
        v as usize
    };
    (0..n)
        .filter_map(|i| {
            let elem = heap.get_field(arr_idx, i + 1)?;
            if elem.type_id == TypeId::Reference {
                let si: usize = elem.into();
                Some(heap.get_string(si).unwrap_or("").to_string())
            } else {
                None
            }
        })
        .collect()
}

unsafe fn dispatch_ffi(handle_val: isize, fn_name: &str, fn_args: &[FnArg]) -> Gen2Value {
    use libloading::os::windows::Library as WinLib;
    use std::ffi::CString;

    let lib = WinLib::from_raw(handle_val);
    let c_name = match CString::new(fn_name) {
        Ok(s) => s,
        Err(_) => { std::mem::forget(lib); return Gen2Value::Void; }
    };

    let raw_args: Vec<u64> = fn_args.iter().map(|a| match a {
        FnArg::U32(v) => *v as u64,
        FnArg::U64(v) => *v,
        FnArg::I32(v) => *v as u64,
        FnArg::I64(v) => *v as u64,
        FnArg::Pointer(b) => b.as_ptr() as u64,
    }).collect();

    type Fn0 = unsafe extern "system" fn() -> isize;
    type Fn1 = unsafe extern "system" fn(u64) -> isize;
    type Fn2 = unsafe extern "system" fn(u64, u64) -> isize;
    type Fn3 = unsafe extern "system" fn(u64, u64, u64) -> isize;
    type Fn4 = unsafe extern "system" fn(u64, u64, u64, u64) -> isize;
    type Fn5 = unsafe extern "system" fn(u64, u64, u64, u64, u64) -> isize;

    let fn_ptr: *const () = match lib.get::<*const ()>(c_name.as_bytes_with_nul()) {
        Ok(sym) => *sym,
        Err(e) => {
            eprintln!("[ellie] GetProcAddress failed for '{}': {}", fn_name, e);
            std::mem::forget(lib);
            return Gen2Value::Void;
        }
    };

    let result = match raw_args.len() {
        0 => std::mem::transmute::<*const (), Fn0>(fn_ptr)(),
        1 => std::mem::transmute::<*const (), Fn1>(fn_ptr)(raw_args[0]),
        2 => std::mem::transmute::<*const (), Fn2>(fn_ptr)(raw_args[0], raw_args[1]),
        3 => std::mem::transmute::<*const (), Fn3>(fn_ptr)(raw_args[0], raw_args[1], raw_args[2]),
        4 => std::mem::transmute::<*const (), Fn4>(fn_ptr)(raw_args[0], raw_args[1], raw_args[2], raw_args[3]),
        _ => std::mem::transmute::<*const (), Fn5>(fn_ptr)(raw_args[0], raw_args[1], raw_args[2], raw_args[3], raw_args[4]),
    };

    std::mem::forget(lib);
    Gen2Value::Raw(result.into())
}

// ── Public entry point ────────────────────────────────────────────────────────

fn core_native_path() -> Option<std::path::PathBuf> {
    // Look for the compiled core native lib next to the running binary.
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    #[cfg(target_os = "windows")]
    let name = "ellie_core_native.dll";
    #[cfg(target_os = "macos")]
    let name = "libellie_core_native.dylib";
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let name = "libellie_core_native.so";
    let candidate = dir.join(name);
    if candidate.exists() { Some(candidate) } else { None }
}

pub fn run(program: Program, bridge_paths: &[std::path::PathBuf]) {
    let mut modules = ModuleManager::new();

    // Core native library takes priority over inline fallbacks.
    if let Some(core_lib) = core_native_path() {
        if let Some(m) = load_bridge(&core_lib, &program) {
            modules.register_module(m);
        }
    }

    // Inline fallback for any function not covered by the native library.
    modules.register_module(build_core_module(&program));

    for bp in bridge_paths {
        if let Some(m) = load_bridge(bp, &program) {
            modules.register_module(m);
        }
    }

    let thread = Thread::new();
    match thread.run(&program, &modules) {
        (VmExit::Ok, _) => {}
        (VmExit::Panic(msg), _) => {
            eprintln!("\n[ellie] Panic: {}", msg);
            std::process::exit(1);
        }
    }
}
