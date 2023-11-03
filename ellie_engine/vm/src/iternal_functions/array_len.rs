use alloc::{string::ToString, vec::Vec};

use crate::{
    raw_type::StaticRawType,
    thread::Isolate,
    utils::{ThreadInfo, VmNativeAnswer, VmNativeCallParameters},
};

use super::InternalFunction;

pub fn array_len_fn(
    isolate: &mut Isolate,
    thread_info: ThreadInfo,
    args: Vec<VmNativeCallParameters>,
) -> VmNativeAnswer {
    if args.len() != 1 {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 1 argument(s)".to_string(),
        );
    }
    match &args[0] {
        VmNativeCallParameters::Static(static_type) => {
            if static_type.type_id.is_static_array() {
                let location_of_array = static_type.to_uint();
                match isolate
                    .stack_memory
                    .get(&thread_info.get_real_pos_with_location(location_of_array + 1))
                {
                    Some(static_data) => {
                        let array_len = static_data.to_uint();
                        VmNativeAnswer::Ok(crate::utils::VmNativeCallParameters::Static(
                            StaticRawType::from_int(array_len as isize),
                        ))
                    }
                    None => {
                        return VmNativeAnswer::RuntimeError(
                            "Memory corruption occurred (array_len)".to_string(),
                        );
                    }
                }
            } else {
                return VmNativeAnswer::RuntimeError(
                    "Signature mismatch expected an array".to_string(),
                );
            }
        }
        VmNativeCallParameters::Dynamic(_) => VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 'static' argument".to_string(),
        ),
    }
}

pub const ARRAY_LEN: InternalFunction = InternalFunction {
    name: &"array_len",
    callback: array_len_fn,
};
