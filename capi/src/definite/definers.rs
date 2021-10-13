use alloc::boxed::Box;
use ellie_core::definite::definers;
use libc::c_char;

#[repr(C)]
pub struct ArrayType {
    pub rtype: Box<DefinerCollecting>,
    pub len: crate::definite::types::integer::IntegerType,
}

#[repr(C)]
pub struct FutureType {
    pub value: Box<DefinerCollecting>,
}

#[repr(C)]
pub struct GrowableArrayType {
    pub rtype: Box<DefinerCollecting>,
}

#[repr(C)]
pub struct GenericType {
    pub rtype: *mut c_char,
}

#[repr(C)]
pub struct FunctionType {
    pub params: *mut DefinerCollecting,
    pub returning: Box<DefinerCollecting>,
}

#[repr(C)]
pub struct CloakType {
    pub rtype: *mut DefinerCollecting,
}

#[repr(C)]
pub struct CollectiveType {
    pub key: Box<DefinerCollecting>,
    pub value: Box<DefinerCollecting>,
}

#[repr(C)]
pub struct NullableType {
    pub value: Box<DefinerCollecting>,
}

#[repr(C)]
pub enum DefinerCollecting {
    Array(ArrayType),
    Future(FutureType),
    GrowableArray(GrowableArrayType),
    Generic(GenericType),
    Function(FunctionType),
    Cloak(CloakType),
    Collective(CollectiveType),
    Nullable(NullableType),
    Dynamic,
}

pub fn build_definer_from(
    from: ellie_core::definite::definers::DefinerCollecting,
) -> DefinerCollecting {
    match from {
        definers::DefinerCollecting::Array(e) => DefinerCollecting::Array(ArrayType {
            rtype: Box::new(build_definer_from(*e.rtype)),
            len: crate::definite::types::integer::build_integer_from(e.len),
        }),
        definers::DefinerCollecting::Future(e) => DefinerCollecting::Future(FutureType {
            value: Box::new(build_definer_from(*e.value)),
        }),
        definers::DefinerCollecting::GrowableArray(e) => {
            DefinerCollecting::GrowableArray(GrowableArrayType {
                rtype: Box::new(build_definer_from(*e.rtype)),
            })
        }
        definers::DefinerCollecting::Generic(e) => DefinerCollecting::Generic(GenericType {
            rtype: e.rtype.as_ptr() as *mut i8,
        }),
        definers::DefinerCollecting::Function(e) => DefinerCollecting::Function(FunctionType {
            params: e
                .params
                .into_iter()
                .map(|param| build_definer_from(param))
                .collect::<Vec<_>>()
                .as_mut_ptr(),
            returning: Box::new(build_definer_from(*e.returning)),
        }),
        definers::DefinerCollecting::Cloak(e) => DefinerCollecting::Cloak(CloakType {
            rtype: e
                .rtype
                .into_iter()
                .map(|entry| build_definer_from(entry))
                .collect::<Vec<_>>()
                .as_mut_ptr(),
        }),
        definers::DefinerCollecting::Collective(e) => {
            DefinerCollecting::Collective(CollectiveType {
                key: Box::new(build_definer_from(*e.key)),
                value: Box::new(build_definer_from(*e.value)),
            })
        }
        definers::DefinerCollecting::Nullable(e) => DefinerCollecting::Nullable(NullableType {
            value: Box::new(build_definer_from(*e.value)),
        }),
        definers::DefinerCollecting::Dynamic => DefinerCollecting::Dynamic,
    }
}
