use libc::c_char;

#[repr(C)]
pub enum EllieTypes {
    String(*mut c_char),
    Int(libc::c_int),
    Float(libc::c_float),
}