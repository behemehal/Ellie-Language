//use libc::c_double;
//use libc::c_float;

#[repr(C)]
pub enum FloatSize {
    F32(f32), //F32(c_float),
    F64(f64), //F64(c_double),
}

#[repr(C)]
pub struct FloatType {
    pub value: FloatSize,
}