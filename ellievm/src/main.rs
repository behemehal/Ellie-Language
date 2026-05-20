mod commands;
pub mod debugger;
mod debugger_messages;
mod run;
mod run_gen2;
mod stream;
mod utils;

#[macro_use]
extern crate lazy_static;

use core::{ffi::c_void, ptr::addr_of_mut};
use ellie_engine::{
    ellie_core::defs::PlatformArchitecture,
    ellie_renderer_utils::{
        options, outputs,
        utils::{CliColor, ColorDisplay, Colors, TextStyles},
    },
    ellie_vm::{
        assert_arg_size,
        channel::{EllieModule, FunctionElement, ModuleElements},
        program::Program,
        raw_type::{RawType, StaticRawType},
        utils::{EllieData, ProgramReader, RawFunctionData, VmNativeAnswer},
    },
    engine_constants,
    vm::{parse_debug_file, RFile},
};

use libffi::low::*;

use run::VmSettings;
use std::{
    ffi::CString, fs::File, io::Read, os::windows::io::AsRawHandle, path::Path, time::SystemTime,
};

fn main() {
    let app = options::generate_ellievm_options();
    let matches = app.get_matches();
    let version = "0.1.0".to_string();
    let cli_color = &CliColor;

    match matches.subcommand() {
        Some(("run", matches)) => {
            let mut _debugger_wait = false;
            if !matches.is_present("allowPanics") {
                std::panic::set_hook(Box::new(|e| {
                    if e.to_string().contains("@Halt") {
                        println!(
                            "\n\n{}-----------------{}\n",
                            cli_color.color(Colors::Blue),
                            cli_color.color(Colors::Reset)
                        );
                        println!(
                            "{}{}VM halted{}\n",
                            cli_color.color(Colors::Yellow),
                            cli_color.text_style(TextStyles::Bold),
                            cli_color.color(Colors::Reset)
                        );
                        println!(
                            "{}{}{}",
                            cli_color.color(Colors::Blue),
                            e.to_string().split("@Halt:").collect::<Vec<&str>>()[1]
                                .split('@')
                                .collect::<Vec<&str>>()[0]
                                .trim(),
                            cli_color.color(Colors::Red)
                        );
                        println!(
                            "\n{}-----------------{}\n\n",
                            cli_color.color(Colors::Blue),
                            cli_color.color(Colors::Reset)
                        );
                        return;
                    }

                    println!(
                        "\n\n{}-----------------{}\n",
                        cli_color.color(Colors::Blue),
                        cli_color.color(Colors::Reset)
                    );
                    println!(
                        "{}{}Oh no! A internal error occured;{}",
                        cli_color.color(Colors::Red),
                        cli_color.text_style(TextStyles::Bold),
                        cli_color.color(Colors::Red)
                    );
                    println!(
                        "{}Can you please share this error with us? This can help us solve issue faster. All you have to do is follow the link below {}[{}CTRL + Mouse Left Click might help{}]",
                        cli_color.color(Colors::Green),
                        cli_color.color(Colors::Reset),
                        cli_color.color(Colors::Yellow),
                        cli_color.color(Colors::Reset),
                    );

                    let line_and_col = if let Some(real_loc) = e.location() {
                        format!("{}:{}", real_loc.line(), real_loc.column())
                    } else {
                        "?:?".to_string()
                    };
                    println!("\n{}{}https://github.com/behemehal/Ellie-Language/issues/new?labels=vm,bug,Internal%20Error&title=Ellie%20Internal%20Error-{}&body=%23%20Ellie%20Internal%20Error%0AGenerated%20by%20ellievm%20located%20at%20{}%0AEllieVersion:{}%0A{}", cli_color.text_style(TextStyles::Underline),cli_color.color(Colors::Green),line_and_col, line_and_col, ellie_engine::engine_constants::ELLIE_ENGINE_VERSION, cli_color.color(Colors::Reset));
                    println!(
                        "\n{}-----------------{}\n\n",
                        cli_color.color(Colors::Blue),
                        cli_color.color(Colors::Reset)
                    );
                    std::process::exit(1);
                }));
            }

            let mut vm_settings = VmSettings {
                json_log: matches.is_present("jsonLog"),
                warnings: true,
                heap_dump: matches.is_present("heapDump"),
                architecture: match matches.value_of("targetArchitecture") {
                    Some(e) => {
                        if e == "64" {
                            PlatformArchitecture::B64
                        } else if e == "32" {
                            PlatformArchitecture::B32
                        } else if e == "16" {
                            PlatformArchitecture::B16
                        } else {
                            println!(
                                "{}Error:{} Unknown architecture '{}{}{}'",
                                cli_color.color(Colors::Red),
                                cli_color.color(Colors::Reset),
                                cli_color.color(Colors::Yellow),
                                e,
                                cli_color.color(Colors::Reset),
                            );
                            std::process::exit(1);
                        }
                    }
                    None => unreachable!(),
                },
                modules: Vec::new(),
            };

            let mut ellie_core_module = EllieModule::new("ellieCore".to_string());

            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "println",
                Box::new(|_, args| {
                    assert_arg_size!(args, 1);

                    // Extract the argument using the macro
                    let arg = match args[0].clone().data.into_string() {
                        Ok(e) => e,
                        Err(_) => {
                            return VmNativeAnswer::RuntimeError(
                                "Signature mismatch expected 'string' argument".to_string(),
                            )
                        }
                    };

                    eprintln!("{arg}");

                    VmNativeAnswer::Ok(().into())
                }),
            )));

            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "timestamp",
                Box::new(|_, args| {
                    if !args.is_empty() {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 0 argument(s)".to_string(),
                        );
                    }
                    VmNativeAnswer::Ok(
                        SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_millis()
                            .into(),
                    )
                }),
            )));

            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "_openFile",
                Box::new(|_, args| {
                    if args.len() != 1 {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 1 argument(s)".to_string(),
                        );
                    }

                    let path = match &args[0].raw_data {
                        RawFunctionData::Static(_e) => {
                            return VmNativeAnswer::RuntimeError(
                                "Signature mismatch, expected 'dynamic' argument".to_string(),
                            )
                        }
                        RawFunctionData::Dynamic(dynamic_value) => {
                            if dynamic_value.is_string() {
                                dynamic_value.to_string()
                            } else {
                                return VmNativeAnswer::RuntimeError(
                                    "Signature mismatch expected 'string' argument".to_string(),
                                );
                            }
                        }
                    };

                    match File::open(&path) {
                        Ok(e) => {
                            println!("Opened file: {:?}", e);
                            let handle = Box::new(e);
                            let handle = Box::into_raw(handle);

                            println!("Handle: {:?}", handle);
                            //handle as *mut usize,

                            VmNativeAnswer::Ok(((handle as *mut usize) as usize).into())
                        }
                        Err(e) => VmNativeAnswer::Ok(
                            format!("Failed to open file '{}', ({})", path, e.to_string()).into(),
                        ),
                    }
                }),
            )));

            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "_readToEnd",
                Box::new(|_, args| {
                    if args.len() != 1 {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 1 argument(s)".to_string(),
                        );
                    }
                    match &args[0].raw_data {
                        RawFunctionData::Static(static_value) => {
                            if static_value.type_id.is_int() {
                                let file_handle = static_value.to_int() as *mut usize;
                                let file = unsafe { &mut *(file_handle as *mut File) };

                                let mut file_contents = String::new();
                                match file.read_to_string(&mut file_contents) {
                                    Ok(_) => VmNativeAnswer::Ok(file_contents.into()),
                                    Err(e) => VmNativeAnswer::RuntimeError(format!(
                                        "Failed to read file ({})",
                                        e.to_string()
                                    )),
                                }
                            } else {
                                VmNativeAnswer::RuntimeError(
                                    "Signature mismatch expected 'int' argument".to_string(),
                                )
                            }
                        }
                        RawFunctionData::Dynamic(_) => VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected static argument".to_string(),
                        ),
                    }
                }),
            )));

            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "panic",
                Box::new(|_, args| {
                    if args.len() != 1 {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 1 argument(s)".to_string(),
                        );
                    }
                    match &args[0].raw_data {
                        RawFunctionData::Static(_e) => VmNativeAnswer::RuntimeError(
                            "Signature mismatch, expected 'dynamic' argument".to_string(),
                        ),
                        RawFunctionData::Dynamic(dynamic_value) => {
                            if dynamic_value.is_string() {
                                VmNativeAnswer::RuntimeError(dynamic_value.to_string())
                            } else {
                                VmNativeAnswer::RuntimeError(
                                    "Signature mismatch expected 'string' argument".to_string(),
                                )
                            }
                        }
                    }
                }),
            )));

            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "render_value",
                Box::new(|ti, args| {
                    println!("Thread info: {:#?}", ti);
                    println!("Args Length: {:#?}", args.len());
                    if args.len() != 1 {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 1 argument(s)".to_string(),
                        );
                    }

                    if args.len() != 1 {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 1 argument(s)".to_string(),
                        );
                    }

                    println!("Args memory_location: {:#?}", args[0].memory_location);
                    println!("Args raw_data       : {:#?}", args[0].raw_data);
                    println!("Args data           : {:#?}", args[0].data);

                    VmNativeAnswer::RuntimeError("Debugging".to_string())
                }),
            )));

            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "stdout_handle",
                Box::new(|_, args| {
                    if !args.is_empty() {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 0 argument(s)".to_string(),
                        );
                    }

                    std::io::stdout().as_raw_handle();

                    let handle = Box::new(std::io::stdout());
                    let handle = Box::into_raw(handle);

                    VmNativeAnswer::Ok(((handle as *mut usize) as usize).into())
                }),
            )));

            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "load_library_handle",
                Box::new(|_, args| {
                    if args.is_empty() {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 1 argument(s)".to_string(),
                        );
                    }

                    let library_path = args
                        .first()
                        .unwrap()
                        .data
                        .clone()
                        .into_string()
                        .unwrap()
                        .clone();

                    unsafe {
                        let library = libloading::Library::new(library_path.as_str()).unwrap();
                        let handle = Box::new(library);
                        let handle = Box::into_raw(handle);

                        println!("Handle: {:?}", handle);
                        //handle as *mut usize,

                        VmNativeAnswer::Ok(((handle as *mut usize) as usize).into())
                    }
                }),
            )));

            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "call_library_function",
                Box::new(|_, args| {
                    if args.len() != 3 {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 3 argument(s)".to_string(),
                        );
                    }

                    let library_handle = args[0].data.clone().into_integer().unwrap().as_usize;
                    let function_name = args[1].data.clone().into_string().unwrap();

                    println!("Library Handle: {:#?}", library_handle);
                    println!("Function Name: {:#?}", function_name);

                    unsafe {
                        //let library = &*(library_handle as *const libloading::Library);

                        let library = libloading::Library::new("user32.dll")
                            .expect("Failed to load library, did you build c lib?");

                        let function: libloading::Symbol<unsafe extern "C" fn()> =
                            match library.get(function_name.as_str().as_bytes()) {
                                Ok(e) => e,
                                Err(e) => {
                                    return VmNativeAnswer::RuntimeError(format!(
                                        "Failed to get function '{}', ({})",
                                        function_name,
                                        e.to_string()
                                    ));
                                }
                            };

                        let mut ffi_type_args: Vec<*mut ffi_type> = Vec::new();
                        let mut ffi_args: Vec<*mut c_void> = Vec::new();

                        for cloak in args[2].data.clone().into_array().unwrap() {
                            let cloak = cloak.into_array().unwrap();
                            let type_id = cloak[0].clone().into_string().unwrap();
                            let value = cloak[1].clone();

                            println!("Setting up ffi type: {:#?}", type_id);

                            //ffi_type_void
                            //ffi_type_uint8
                            //ffi_type_sint8
                            //ffi_type_uint16
                            //ffi_type_sint16
                            //ffi_type_uint32
                            //ffi_type_sint32
                            //ffi_type_uint64
                            //ffi_type_sint64
                            //ffi_type_float
                            //ffi_type_double
                            //ffi_type_pointer

                            match type_id.as_str() {
                                "void" => ffi_type_args.push(core::ptr::addr_of_mut!(types::void)),
                                "uint8" => {
                                    ffi_type_args.push(core::ptr::addr_of_mut!(types::uint8))
                                }
                                "sint8" => {
                                    ffi_type_args.push(core::ptr::addr_of_mut!(types::sint8))
                                }
                                "uint16" => {
                                    ffi_type_args.push(core::ptr::addr_of_mut!(types::uint16))
                                }
                                "sint16" => {
                                    ffi_type_args.push(core::ptr::addr_of_mut!(types::sint16))
                                }
                                "uint32" => {
                                    ffi_type_args.push(core::ptr::addr_of_mut!(types::uint32))
                                }
                                "sint32" => {
                                    ffi_type_args.push(core::ptr::addr_of_mut!(types::sint32))
                                }
                                "uint64" => {
                                    ffi_type_args.push(core::ptr::addr_of_mut!(types::uint64))
                                }
                                "sint64" => {
                                    ffi_type_args.push(core::ptr::addr_of_mut!(types::sint64))
                                }
                                "float" => {
                                    ffi_type_args.push(core::ptr::addr_of_mut!(types::float))
                                }
                                "double" => {
                                    ffi_type_args.push(core::ptr::addr_of_mut!(types::double))
                                }
                                "pointer" => {
                                    println!("SETTING UP POINTER");
                                    ffi_type_args.push(core::ptr::addr_of_mut!(types::pointer))
                                }
                                _ => {
                                    return VmNativeAnswer::RuntimeError(format!(
                                        "Unknown ffi type '{}'",
                                        type_id
                                    ));
                                }
                            }

                            match type_id.as_str() {
                                "void" => {
                                    if value.is_void() {
                                        ffi_args.push(&0_u64 as *const _ as *mut c_void);
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'void' on ffi argument",
                                        ));
                                    }
                                }
                                "uint8" => {
                                    if value.is_integer() {
                                        ffi_args.push(
                                            &(value.into_integer().unwrap().as_usize as u8)
                                                as *const _
                                                as *mut c_void,
                                        );
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'uint8' on ffi argument",
                                        ));
                                    }
                                }
                                "sint8" => {
                                    if value.is_integer() {
                                        ffi_args.push(
                                            &(value.into_integer().unwrap().as_usize as i8)
                                                as *const _
                                                as *mut c_void,
                                        );
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'sint8' on ffi argument",
                                        ));
                                    }
                                }
                                "uint16" => {
                                    if value.is_integer() {
                                        ffi_args.push(
                                            &(value.into_integer().unwrap().as_usize as u16)
                                                as *const _
                                                as *mut c_void,
                                        );
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'uint16' on ffi argument",
                                        ));
                                    }
                                }
                                "sint16" => {
                                    if value.is_integer() {
                                        ffi_args.push(
                                            &(value.into_integer().unwrap().as_usize as i16)
                                                as *const _
                                                as *mut c_void,
                                        );
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'sint16' on ffi argument",
                                        ));
                                    }
                                }
                                "uint32" => {
                                    if value.is_integer() {
                                        ffi_args.push(
                                            &(value.into_integer().unwrap().as_usize as u32)
                                                as *const _
                                                as *mut c_void,
                                        );
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'uint32' on ffi argument",
                                        ));
                                    }
                                }
                                "sint32" => {
                                    if value.is_integer() {
                                        ffi_args.push(
                                            &(value.into_integer().unwrap().as_usize as i32)
                                                as *const _
                                                as *mut c_void,
                                        );
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'sint32' on ffi argument",
                                        ));
                                    }
                                }
                                "uint64" => {
                                    if value.is_integer() {
                                        ffi_args.push(
                                            &(value.into_integer().unwrap().as_usize as u64)
                                                as *const _
                                                as *mut c_void,
                                        );
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'uint64' on ffi argument",
                                        ));
                                    }
                                }
                                "sint64" => {
                                    if value.is_integer() {
                                        ffi_args.push(
                                            &(value.into_integer().unwrap().as_usize as i64)
                                                as *const _
                                                as *mut c_void,
                                        );
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'sint64' on ffi argument",
                                        ));
                                    }
                                }
                                "float" => {
                                    if value.is_float() {
                                        ffi_args.push(
                                            &(value.into_float().unwrap() as f32) as *const _
                                                as *mut c_void,
                                        );
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'float' on ffi argument",
                                        ));
                                    }
                                }
                                "double" => {
                                    if value.is_float() {
                                        ffi_args.push(
                                            &(value.into_float().unwrap() as f64) as *const _
                                                as *mut c_void,
                                        );
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'double' on ffi argument",
                                        ));
                                    }
                                }
                                "pointer" => {
                                    if value.is_string() {
                                        println!("Pointer: {:#?}", value);
                                        let cstring =
                                            CString::new(value.clone().into_string().unwrap().as_str())
                                                .unwrap()
                                                .into_bytes();
                                        let cstring = &cstring.as_ptr() as *const _ as *mut c_void;
                                        ffi_args.push(cstring);
                                    } else {
                                        return VmNativeAnswer::RuntimeError(format!(
                                            "Expected 'pointer' string on ffi argument",
                                        ));
                                    }
                                }
                                _ => {
                                    return VmNativeAnswer::RuntimeError(format!(
                                        "Unknown ffi type '{}'",
                                        type_id
                                    ));
                                }
                            }
                        }

                        let mut cif: ffi_cif = Default::default();

                        match prep_cif(
                            &mut cif,
                            libffi::low::ffi_abi_FFI_DEFAULT_ABI,
                            args.len(),
                            addr_of_mut!(types::sint32),
                            ffi_type_args.as_mut_ptr(),
                        ) {
                            Ok(c) => c,
                            Err(e) => {
                                let reason = match e {
                                    libffi::low::Error::Typedef => {
                                        "Bad or unsupported type representation"
                                    }
                                    libffi::low::Error::Abi => "Given a bad or unsupported ABI",
                                };
                                return VmNativeAnswer::RuntimeError(format!(
                                    "Failed to prepare ffi_cif ({})",
                                    reason
                                ));
                            }
                        }

                        let fn_ptr = CodePtr::from_ptr(function.try_as_raw_ptr().unwrap());

                        let text = CString::new("This is the title").unwrap().into_bytes();
                        let text = &text.as_ptr() as *const _ as *mut c_void;
                        let caption = CString::new("This is the caption").unwrap().into_bytes();
                        let caption = &caption.as_ptr() as *const _ as *mut c_void;


                        let mut values: [*mut c_void; 4] = [
                            &0_u64 as *const _ as *mut c_void,                // HWND
                            ffi_args[1],
                            ffi_args[2],
                            &0_u32 as *const _ as *mut c_void, // uType
                        ];

                        let mut ffi_args: &mut [*mut c_void] = &mut ffi_args;

                        let result =
                            libffi::low::call::<i32>(&mut cif, fn_ptr, values.as_mut_ptr());

                        println!("MessageBoxW returned: {}", result);

                        println!("ffi_type_args: {:#?}", ffi_type_args);
                        println!("ffi_args: {:#?}", ffi_args);

                        //let result = function();
                        VmNativeAnswer::Ok(result.into())
                    }
                }),
            )));

            vm_settings.modules.push(ellie_core_module);

            let debug_file = match matches.value_of("debugInfo") {
                Some(e) => {
                    let path = Path::new(e);
                    if path.is_file() {
                        let mut file_contents = String::new();
                        match File::open(e) {
                            Ok(mut e) => {
                                e.read_to_string(&mut file_contents).unwrap();
                                match parse_debug_file(file_contents) {
                                    Ok(e) => Some(e),
                                    Err(e) => {
                                        println!(
                                            "{}Error:{} {}",
                                            cli_color.color(Colors::Red),
                                            cli_color.color(Colors::Reset),
                                            e
                                        );
                                        std::process::exit(1);
                                    }
                                }
                            }
                            Err(e) => {
                                println!(
                                    "{}Error:{} Failed to read file {}[{}]{}",
                                    cli_color.color(Colors::Red),
                                    cli_color.color(Colors::Reset),
                                    cli_color.color(Colors::Cyan),
                                    e,
                                    cli_color.color(Colors::Reset)
                                );
                                std::process::exit(1);
                            }
                        }
                    } else {
                        println!(
                            "{}Error:{} Given path is not a file",
                            cli_color.color(Colors::Red),
                            cli_color.color(Colors::Reset)
                        );
                        std::process::exit(1);
                    }
                }
                None => None,
            };

            let target_path_str = matches.value_of("target").unwrap();
            let path = Path::new(target_path_str);

            // Auto-detect gen2 binary by extension (.eic2) or explicit path ending
            let is_gen2 = target_path_str.ends_with(".eic2");

            if is_gen2 {
                // --- Gen2 path ---
                if !path.exists() || !path.is_file() {
                    println!(
                        "{}Error:{} Target path does not exist or is not a file",
                        cli_color.color(Colors::Red),
                        cli_color.color(Colors::Reset)
                    );
                    std::process::exit(1);
                }
                let bytes = match std::fs::read(path) {
                    Ok(b) => b,
                    Err(e) => {
                        println!(
                            "{}Error:{} Failed to read file {}[{}]{}",
                            cli_color.color(Colors::Red),
                            cli_color.color(Colors::Reset),
                            cli_color.color(Colors::Cyan),
                            e,
                            cli_color.color(Colors::Reset)
                        );
                        std::process::exit(1);
                    }
                };
                use ellie_engine::ellie_vm_gen2::program::Program as Gen2Program;
                let gen2_program = match Gen2Program::load_from_bytes(&bytes) {
                    Ok(p) => p,
                    Err(e) => {
                        println!(
                            "{}Error:{} Failed to load gen2 program {}[{}]{}",
                            cli_color.color(Colors::Red),
                            cli_color.color(Colors::Reset),
                            cli_color.color(Colors::Cyan),
                            e,
                            cli_color.color(Colors::Reset)
                        );
                        std::process::exit(1);
                    }
                };
                run_gen2::run_gen2(gen2_program);
            } else {
                // --- Gen1 path ---
                let program = if path.exists() {
                    if path.is_file() {
                        match File::open(path) {
                            Ok(mut e) => {
                                let mut reader = RFile::new(&mut e);
                                let mut program_reader = ProgramReader::new(&mut reader);
                                let mut program = Program::new();
                                match program.build_from_reader(&mut program_reader) {
                                    Ok(_) => program,
                                    Err(e) => {
                                        println!(
                                            "{}Error:{} Failed to read program {}[{:?}]{}",
                                            cli_color.color(Colors::Red),
                                            cli_color.color(Colors::Reset),
                                            cli_color.color(Colors::Cyan),
                                            e,
                                            cli_color.color(Colors::Reset)
                                        );
                                        std::process::exit(1);
                                    }
                                }
                            }
                            Err(e) => {
                                println!(
                                    "{}Error:{} Failed to read file {}[{}]{}",
                                    cli_color.color(Colors::Red),
                                    cli_color.color(Colors::Reset),
                                    cli_color.color(Colors::Cyan),
                                    e,
                                    cli_color.color(Colors::Reset)
                                );
                                std::process::exit(1);
                            }
                        }
                    } else {
                        println!(
                            "{}Error:{} Given path is not a file",
                            cli_color.color(Colors::Red),
                            cli_color.color(Colors::Reset)
                        );
                        std::process::exit(1);
                    }
                } else {
                    println!(
                        "{}Error:{} Target path does not exist",
                        cli_color.color(Colors::Red),
                        cli_color.color(Colors::Reset)
                    );
                    std::process::exit(1);
                };
                run::run(program, vm_settings, debug_file);
            }
        }
        Some(("debug", matches)) => {
            if !matches.is_present("allowPanics") {
                std::panic::set_hook(Box::new(|e| {
                    if e.to_string().contains("@Halt") {
                        println!(
                            "\n\n{}-----------------{}\n",
                            cli_color.color(Colors::Blue),
                            cli_color.color(Colors::Reset)
                        );
                        println!(
                            "{}{}VM halted{}\n",
                            cli_color.color(Colors::Yellow),
                            cli_color.text_style(TextStyles::Bold),
                            cli_color.color(Colors::Reset)
                        );
                        println!(
                            "{}{}{}",
                            cli_color.color(Colors::Blue),
                            e.to_string().split("@Halt:").collect::<Vec<&str>>()[1]
                                .split('@')
                                .collect::<Vec<&str>>()[0]
                                .trim(),
                            cli_color.color(Colors::Red)
                        );
                        println!(
                            "\n{}-----------------{}\n\n",
                            cli_color.color(Colors::Blue),
                            cli_color.color(Colors::Reset)
                        );
                        return;
                    }

                    println!(
                        "\n\n{}-----------------{}\n",
                        cli_color.color(Colors::Blue),
                        cli_color.color(Colors::Reset)
                    );
                    println!(
                        "{}{}Oh no! A internal error occured;{}",
                        cli_color.color(Colors::Red),
                        cli_color.text_style(TextStyles::Bold),
                        cli_color.color(Colors::Red)
                    );
                    println!(
                        "{}Can you please share this error with us? This can help us solve issue faster. All you have to do is follow the link below {}[{}CTRL + Mouse Left Click might help{}]",
                        cli_color.color(Colors::Green),
                        cli_color.color(Colors::Reset),
                        cli_color.color(Colors::Yellow),
                        cli_color.color(Colors::Reset),
                    );

                    let line_and_col = if let Some(real_loc) = e.location() {
                        format!("{}:{}", real_loc.line(), real_loc.column())
                    } else {
                        "?:?".to_string()
                    };
                    println!("\n{}{}https://github.com/behemehal/Ellie-Language/issues/new?labels=vm,bug,Internal%20Error&title=Ellie%20Internal%20Error-{}&body=%23%20Ellie%20Internal%20Error%0AGenerated%20by%20ellievm%20located%20at%20{}%0AEllieVersion:{}%0A{}", cli_color.text_style(TextStyles::Underline),cli_color.color(Colors::Green),line_and_col, line_and_col, ellie_engine::engine_constants::ELLIE_ENGINE_VERSION, cli_color.color(Colors::Reset));
                    println!(
                        "\n{}-----------------{}\n\n",
                        cli_color.color(Colors::Blue),
                        cli_color.color(Colors::Reset)
                    );
                    std::process::exit(1);
                }));
            }

            /* let mut ellie_core_module = EllieModule::new("ellieCore".to_string());
            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "println",
                Box::new(|_, args| {
                    if args.len() != 1 {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 1 argument(s)".to_string(),
                        );
                    }
                    match &args[0] {
                        VmNativeCallParameters::Static(_) => VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 'dynamic' argument".to_string(),
                        ),
                        VmNativeCallParameters::Dynamic(dynamic_value) => {
                            if dynamic_value.is_string() {
                                eprintln!("{}", dynamic_value.to_string());
                                VmNativeAnswer::Ok(VmNativeCallParameters::Static(
                                    StaticRawType::from_void(),
                                ))
                            } else {
                                VmNativeAnswer::RuntimeError(
                                    "Signature mismatch expected 'string' argument".to_string(),
                                )
                            }
                        }
                    }
                }),
            )));

            vm_settings.modules.push(ellie_core_module); */

            /* let path = Path::new(matches.value_of("target").unwrap().clone());
            let program = if path.exists() {
                if path.is_file() {
                    match File::open(path) {
                        Ok(mut e) => {
                            let mut reader = RFile::new(&mut e);
                            let mut program_reader = ProgramReader::new(&mut reader);
                            let mut program = Program::new();
                            match program.build_from_reader(&mut program_reader) {
                                Ok(_) => program,
                                Err(e) => {
                                    println!(
                                        "{}Error:{} Failed to read program {}[{:?}]{}",
                                        cli_color.color(Colors::Red),
                                        cli_color.color(Colors::Reset),
                                        cli_color.color(Colors::Cyan),
                                        e,
                                        cli_color.color(Colors::Reset)
                                    );
                                    std::process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            println!(
                                "{}Error:{} Failed to read file {}[{}]{}",
                                cli_color.color(Colors::Red),
                                cli_color.color(Colors::Reset),
                                cli_color.color(Colors::Cyan),
                                e,
                                cli_color.color(Colors::Reset)
                            );
                            std::process::exit(1);
                        }
                    }
                } else {
                    println!(
                        "{}Error:{} Given path is not a file",
                        cli_color.color(Colors::Red),
                        cli_color.color(Colors::Reset)
                    );
                    std::process::exit(1);
                }
            } else {
                println!(
                    "{}Error:{} Target path does not exist",
                    cli_color.color(Colors::Red),
                    cli_color.color(Colors::Reset)
                );
                std::process::exit(1);
            }; */

            let imported_commands = matches
                .values_of("insertCommands")
                .unwrap_or_default()
                .map(str::to_string)
                .collect::<Vec<_>>();
            debugger::debug(matches.is_present("jsonLog"), imported_commands);
        }
        Some(("version", matches)) => {
            if matches.is_present("detailed") {
                if matches.is_present("jsonLog") {
                    let mut output = outputs::VERSION_DETAILED.clone();
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: version,
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "git_hash".to_string(),
                        value: engine_constants::ELLIE_BUILD_GIT_HASH.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "git_hash".to_string(),
                        value: engine_constants::ELLIE_BUILD_GIT_HASH.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "build_date".to_string(),
                        value: engine_constants::ELLIE_BUILD_DATE.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "engine_version".to_string(),
                        value: engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "engine_code".to_string(),
                        value: engine_constants::ELLIE_ENGINE_VERSION_NAME.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "vm_version".to_string(),
                        value: engine_constants::ELLIE_VM_VERSION.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "core_version".to_string(),
                        value: engine_constants::ELLIE_CORE_VERSION.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "EllieVM v{} ({}: {}){}\n\nEllie v{} - Code: {}\nVM Version: v{}\nCore version: v{}\n",
                        version,
                        engine_constants::ELLIE_BUILD_GIT_HASH,
                        engine_constants::ELLIE_BUILD_DATE,
                        if engine_constants::ELLIE_BUILD_GIT_BRANCH != "main" {
                            format!(
                                " [{}{}{}] ",
                                cli_color.color(Colors::Yellow),
                                engine_constants::ELLIE_BUILD_GIT_BRANCH,
                                cli_color.color(Colors::Reset)
                            )
                        } else {
                            String::new()
                        },
                        engine_constants::ELLIE_ENGINE_VERSION,
                        engine_constants::ELLIE_ENGINE_VERSION_NAME,
                        engine_constants::ELLIE_VM_VERSION,
                        engine_constants::ELLIE_CORE_VERSION,
                    );
                }
            } else if matches.is_present("jsonLog") {
                let mut output = outputs::VERSION.clone();
                output.extra.push(outputs::CliOuputExtraData {
                    key: "version".to_string(),
                    value: engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                });
                output.extra.push(outputs::CliOuputExtraData {
                    key: "git_hash".to_string(),
                    value: engine_constants::ELLIE_BUILD_GIT_HASH.to_owned(),
                });
                output.extra.push(outputs::CliOuputExtraData {
                    key: "git_branch".to_string(),
                    value: engine_constants::ELLIE_BUILD_GIT_BRANCH.to_owned(),
                });
                output.extra.push(outputs::CliOuputExtraData {
                    key: "build_date".to_string(),
                    value: engine_constants::ELLIE_BUILD_DATE.to_owned(),
                });
                println!("{}", serde_json::to_string(&output).unwrap());
            } else {
                println!(
                    "EllieVM v{} ({} : {}){}",
                    version,
                    engine_constants::ELLIE_BUILD_GIT_HASH,
                    engine_constants::ELLIE_BUILD_DATE,
                    if engine_constants::ELLIE_BUILD_GIT_BRANCH != "main" {
                        format!(
                            " [{}{}{}] ",
                            cli_color.color(Colors::Yellow),
                            engine_constants::ELLIE_BUILD_GIT_BRANCH,
                            cli_color.color(Colors::Reset)
                        )
                    } else {
                        String::new()
                    },
                );
            }
        }
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
