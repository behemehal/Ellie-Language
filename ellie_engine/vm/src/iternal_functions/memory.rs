use std::println;

use super::InternalFunction;
use crate::{
    isolate::Isolate,
    raw_type::{RawType, StaticRawType},
    utils::{FunctionCallParameter, RawFunctionData, ThreadInfo, VmNativeAnswer},
};
use alloc::{string::ToString, vec::Vec};

pub fn frame_pos_fn(
    _isolate: &mut Isolate,
    thread_info: ThreadInfo,
    args: Vec<FunctionCallParameter>,
) -> VmNativeAnswer {
    if !args.is_empty() {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 0 argument(s)".to_string(),
        );
    }
    VmNativeAnswer::Ok(thread_info.get_real_pos().into())
}

pub fn code_pos_fn(
    _isolate: &mut Isolate,
    thread_info: ThreadInfo,
    args: Vec<FunctionCallParameter>,
) -> VmNativeAnswer {
    if !args.is_empty() {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 0 argument(s)".to_string(),
        );
    }
    VmNativeAnswer::Ok(thread_info.pos.into())
}

pub fn variable_size_fn(
    isolate: &mut Isolate,
    thread_info: ThreadInfo,
    args: Vec<FunctionCallParameter>,
) -> VmNativeAnswer {
    if args.len() != 1 {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 1 argument(s)".to_string(),
        );
    }

    todo!()

    /* match &args[0] {
        VmNativeCallParameters::Static(static_raw_type) => {
            static_raw_type.
        },
        VmNativeCallParameters::Dynamic(raw_type) => todo!(),
    } */
}

pub fn alloc_fn(
    isolate: &mut Isolate,
    thread_info: ThreadInfo,
    args: Vec<FunctionCallParameter>,
) -> VmNativeAnswer {
    if args.len() != 2 {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 2 argument(s)".to_string(),
        );
    }

    let pointer = match &args[0].raw_data {
        RawFunctionData::Static(static_raw_type) => {
            if static_raw_type.type_id.is_int() {
                static_raw_type.to_uint()
            } else {
                return VmNativeAnswer::RuntimeError(
                    "Expected integer as first argument".to_string(),
                );
            }
        }
        RawFunctionData::Dynamic(_) => {
            return VmNativeAnswer::RuntimeError("Expected integer as first argument".to_string());
        }
    };

    let size = match &args[1].raw_data {
        RawFunctionData::Static(static_raw_type) => {
            if static_raw_type.type_id.is_int() {
                static_raw_type.to_uint()
            } else {
                return VmNativeAnswer::RuntimeError(
                    "Expected integer as second argument".to_string(),
                );
            }
        }
        RawFunctionData::Dynamic(_) => {
            return VmNativeAnswer::RuntimeError("Expected integer as second argument".to_string());
        }
    };

    isolate
        .heap_memory
        .set(&pointer, RawType::array(Vec::new()));

    VmNativeAnswer::Ok(().into())
}

pub fn platform_size_fn(
    _isolate: &mut Isolate,
    thread_info: ThreadInfo,
    args: Vec<FunctionCallParameter>,
) -> VmNativeAnswer {
    if args.len() != 0 {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 0 argument(s)".to_string(),
        );
    }

    VmNativeAnswer::Ok(thread_info.arch.usize_len().into())
}

pub fn realloc(
    isolate: &mut Isolate,
    thread_info: ThreadInfo,
    args: Vec<FunctionCallParameter>,
) -> VmNativeAnswer {
    if args.len() != 2 {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 2 argument(s)".to_string(),
        );
    }

    let pointer = match &args[0].raw_data {
        RawFunctionData::Static(static_raw_type) => {
            if static_raw_type.type_id.is_int() {
                static_raw_type.to_uint()
            } else {
                return VmNativeAnswer::RuntimeError(
                    "Expected integer as first argument".to_string(),
                );
            }
        }
        RawFunctionData::Dynamic(_) => {
            return VmNativeAnswer::RuntimeError("Expected integer as first argument".to_string());
        }
    };

    let size = match &args[1].raw_data {
        RawFunctionData::Static(static_raw_type) => {
            if static_raw_type.type_id.is_int() {
                static_raw_type.to_int()
            } else {
                return VmNativeAnswer::RuntimeError(
                    "Expected integer as second argument".to_string(),
                );
            }
        }
        RawFunctionData::Dynamic(_) => {
            return VmNativeAnswer::RuntimeError("Expected integer as second argument".to_string());
        }
    };

    println!("Reallocating {} to {}", pointer, size);

    let mut data = isolate.heap_memory.get_mut(&pointer).unwrap();
    //println!("Data: {:?}", data.data);

    data.resize(size as usize, 0);

    //println!("New data: {:?}", data);

    VmNativeAnswer::Ok(().into())
}

pub fn ptr(
    isolate: &mut Isolate,
    thread_info: ThreadInfo,
    args: Vec<FunctionCallParameter>,
) -> VmNativeAnswer {
    if args.len() != 1 {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 1 argument(s)".to_string(),
        );
    }

    VmNativeAnswer::Ok(args[0].memory_location.into())
}

pub fn index_mut(
    isolate: &mut Isolate,
    thread_info: ThreadInfo,
    args: Vec<FunctionCallParameter>,
) -> VmNativeAnswer {
    if args.len() != 3 {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 2 argument(s)".to_string(),
        );
    }

    let array_ptr = match &args[0].raw_data {
        RawFunctionData::Static(static_raw_type) => {
            if static_raw_type.type_id.is_int() {
                static_raw_type.to_uint()
            } else {
                return VmNativeAnswer::RuntimeError(
                    "Expected integer as second argument".to_string(),
                );
            }
        }
        RawFunctionData::Dynamic(_) => {
            return VmNativeAnswer::RuntimeError("Expected integer as second argument".to_string());
        }
    };

    let index = match &args[1].raw_data {
        RawFunctionData::Static(static_raw_type) => {
            if static_raw_type.type_id.is_int() {
                static_raw_type.to_int()
            } else {
                return VmNativeAnswer::RuntimeError(
                    "Expected integer as first argument".to_string(),
                );
            }
        }
        RawFunctionData::Dynamic(_) => {
            return VmNativeAnswer::RuntimeError("Expected integer as first argument".to_string());
        }
    };

    if index < 0 {
        return VmNativeAnswer::RuntimeError("Index must be positive".to_string());
    }

    let data = match &args[2].raw_data {
        RawFunctionData::Static(static_raw_type) => {
            if static_raw_type.type_id.is_int() {
                static_raw_type.to_uint()
            } else {
                return VmNativeAnswer::RuntimeError(
                    "Expected integer as third argument".to_string(),
                );
            }
        }
        RawFunctionData::Dynamic(_) => {
            return VmNativeAnswer::RuntimeError("Expected integer as third argument".to_string());
        }
    };

    let mut array_heap_data = match isolate.heap_memory.get_mut(&array_ptr) {
        Some(heap_entry) => heap_entry,
        None => {
            return VmNativeAnswer::RuntimeError("Pointer not found".to_string());
        }
    };

    /*
    let entry_len = {
        if array_heap_data.data.len() == (thread_info.arch.usize_len() as usize + 1) {
            //platform safety:
            //If current platform is 64 bit, but targeted arch is 64
            //we can use 64 bit to store the length of the array
            //----------------
            //If current platform is 64 bit, but targeted arch is 32
            //we can use 4 bytes of the usize's 64bit 8 bytes to store the length of the array
            //----------------
            //If current platform is 64 bit, but targeted arch is 16
            //we can use 2 bytes of the usize's 64bit 8 bytes to store the length of the array
            let len = data.len().to_le_bytes().to_vec();
            heap_value.data.extend(match arch {
                ellie_core::defs::PlatformArchitecture::B16 => {
                    vec![len[0], len[1]]
                }
                ellie_core::defs::PlatformArchitecture::B32 => {
                    vec![len[0], len[1], len[2], len[3]]
                }
                ellie_core::defs::PlatformArchitecture::B64 => len,
            });
            data.len()
        } else {
            let array_len_range = {
                let type_id_size = arch.type_id_size() as usize;
                let usize_len = arch.usize_len() as usize;
                type_id_size..(type_id_size + usize_len)
            };
            usize::from_le_bytes(
                heap_value.data[array_len_range].try_into().unwrap(),
            )
        }
    };
    */

    //if heap_entry.data.len() <= index as usize {
    //    heap_entry.resize(index as usize + 1, 0);
    //}

    let index_size = if index == 0 {
        index as usize
    } else {
        index as usize * thread_info.arch.usize_len() as usize
    };

    /*
    println!(
        "Setting {} to {:?}, {}, {:?}",
        index,
        data,
        heap_entry.data.len(),
        heap_entry.data
    );
    */

    let new_entry_data = RawType::from_usize(data).to_bytes();

    let ptr = (
        index_size,
        index_size + thread_info.arch.usize_len() as usize,
    );

    println!("Setting {} to {:?}", ptr.0, ptr.1);

    //heap_entry.data[ptr.0..ptr.1].copy_from_slice(&RawType::from_usize(data).to_bytes());

    VmNativeAnswer::Ok(().into())
}

pub const FRAME_POS: InternalFunction = InternalFunction {
    name: "frame_pos",
    callback: frame_pos_fn,
};

pub const CODE_POS: InternalFunction = InternalFunction {
    name: "code_pos",
    callback: code_pos_fn,
};

pub const ALLOC: InternalFunction = InternalFunction {
    name: "alloc",
    callback: alloc_fn,
};

pub const REALLOC: InternalFunction = InternalFunction {
    name: "realloc",
    callback: realloc,
};

pub const PTR: InternalFunction = InternalFunction {
    name: "ptr",
    callback: ptr,
};

pub const PLATFORM_SIZE: InternalFunction = InternalFunction {
    name: "platform_size",
    callback: platform_size_fn,
};

pub const INDEX_MUT: InternalFunction = InternalFunction {
    name: "index_mut",
    callback: index_mut,
};
