use alloc::{string::ToString, vec::Vec};

use crate::{
    raw_type::StaticRawType,
    thread::Isolate,
    utils::{ThreadInfo, VmNativeAnswer, VmNativeCallParameters},
};

use super::InternalFunction;

pub fn frame_pos_fn(
    _isolate: &mut Isolate,
    thread_info: ThreadInfo,
    args: Vec<VmNativeCallParameters>,
) -> VmNativeAnswer {
    if args.len() != 0 {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 0 argument(s)".to_string(),
        );
    }
    VmNativeAnswer::Ok(VmNativeCallParameters::Static(StaticRawType::from_uint(
        thread_info.get_real_pos(),
    )))
}

pub fn code_pos_fn(
    _isolate: &mut Isolate,
    thread_info: ThreadInfo,
    args: Vec<VmNativeCallParameters>,
) -> VmNativeAnswer {
    if args.len() != 0 {
        return VmNativeAnswer::RuntimeError(
            "Signature mismatch expected 0 argument(s)".to_string(),
        );
    }
    VmNativeAnswer::Ok(VmNativeCallParameters::Static(StaticRawType::from_uint(
        thread_info.pos,
    )))
}

pub const FRAME_POS: InternalFunction = InternalFunction {
    name: &"frame_pos",
    callback: frame_pos_fn,
};

pub const CODE_POS: InternalFunction = InternalFunction {
    name: &"code_pos",
    callback: code_pos_fn,
};
