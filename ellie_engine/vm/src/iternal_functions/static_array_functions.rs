use alloc::{string::ToString, vec::Vec};

use crate::{
    isolate::Isolate,
    raw_type::StaticRawType,
    utils::{EllieData, FunctionCallParameter, RawFunctionData, ThreadInfo, VmNativeAnswer},
};

use super::InternalFunction;

pub fn array_len_fn(
    isolate: &mut Isolate,
    _thread_info: ThreadInfo,
    args: Vec<FunctionCallParameter>,
) -> VmNativeAnswer {
    if args.len() != 1 {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 1 argument(s)".to_string(),
        );
    }
    match &args[0].raw_data {
        RawFunctionData::Static(static_type) => {
            if static_type.type_id.is_static_array() {
                let location_of_array = static_type.to_uint();
                match isolate.stack_memory.get(&(location_of_array + 1)) {
                    Some(static_data) => {
                        let array_len = static_data.to_uint();

                        VmNativeAnswer::Ok(array_len.into())
                    }
                    None => VmNativeAnswer::RuntimeError(
                        "Memory corruption occurred (array_len)".to_string(),
                    ),
                }
            } else {
                VmNativeAnswer::RuntimeError("Signature mismatch expected an array".to_string())
            }
        }
        RawFunctionData::Dynamic(_) => VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 'static' argument".to_string(),
        ),
    }
}

pub const ARRAY_LEN: InternalFunction = InternalFunction {
    name: "array_len",
    callback: array_len_fn,
};
