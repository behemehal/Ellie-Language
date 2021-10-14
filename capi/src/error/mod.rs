use ellie_core::error;
use libc::c_char;

#[repr(C)]
pub struct Error {
    pub code: u8,
    pub path: *mut c_char,
    pub scope: *mut c_char,
    pub message: *mut c_char,
    pub title: *mut c_char,
    pub builded_message: BuildedError,
    pub debug_message: *mut c_char,
    pub pos: crate::defs::Cursor,
}

pub fn build_error_from(error: ellie_core::error::Error) -> Error {
    Error {
        code: error.code,
        path: error.path.as_ptr() as *mut i8,
        scope: error.scope.as_ptr() as *mut i8,
        message: error.message.as_ptr() as *mut i8,
        title: error.title.as_ptr() as *mut i8,
        builded_message: build_builded_error_from(error.builded_message),
        debug_message: error.debug_message.as_ptr() as *mut i8,
        pos: crate::defs::Cursor {
            range_start: crate::defs::CursorPosition(
                error.pos.range_start.0,
                error.pos.range_start.1,
            ),
            range_end: crate::defs::CursorPosition(error.pos.range_end.0, error.pos.range_end.1),
        },
    }
}

#[repr(C)]
pub struct ErrorBuildField {
    pub key: *mut c_char,
    pub value: *mut c_char,
}

#[repr(C)]
pub struct BuildedError {
    pub builded: *mut c_char,
    pub fields: *mut ErrorBuildField,
}

pub fn build_builded_error_from(error: error::BuildedError) -> BuildedError {
    BuildedError {
        builded: error.builded.as_ptr() as *mut i8,
        fields: error
            .fields
            .into_iter()
            .map(|field| ErrorBuildField {
                key: field.key.as_ptr() as *mut i8,
                value: field.value.as_ptr() as *mut i8,
            })
            .collect::<Vec<_>>()
            .as_mut_ptr(),
    }
}
