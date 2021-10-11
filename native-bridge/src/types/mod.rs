pub mod string;
pub mod int;
pub mod float;

#[repr(C)]
pub enum EllieType {
    StringType(string::StringType),
    IntType(int::IntegerType),
    FloatType(float::FloatType),
}