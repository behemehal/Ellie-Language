pub mod array;
pub mod as_keyword;
pub mod bool;
pub mod brace_reference;
pub mod byte;
pub mod char;
pub mod class_call;
pub mod cloak;
pub mod collective;
pub mod constructor_parameter;
pub mod decimal;
pub mod dynamic;
pub mod enum_data;
pub mod function;
pub mod function_call;
pub mod function_parameter;
pub mod integer;
pub mod negative;
pub mod null;
pub mod null_resolver;
pub mod operator;
pub mod reference;
pub mod string;
pub mod variable_type;
pub mod void;

use crate::{
    assembler::Assembler,
    instructions::Registers,
    types::Types as ByteCodeTypes,
    utils::{f32_to_le_bytes, f64_to_le_bytes, isize_to_le_bytes},
};
use alloc::vec::Vec;
use ellie_core::{definite::types::Types, defs::PlatformArchitecture};

pub struct TypeTranspilerOptions<'a> {
    assembler: Option<&'a mut Assembler>,
    target_register: Registers,
    target_page: Option<usize>,
    dependencies: Option<Vec<usize>>,
}

impl<'a> Default for TypeTranspilerOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> TypeTranspilerOptions<'a> {
    pub fn new() -> Self {
        Self {
            assembler: None,
            target_register: Registers::A,
            target_page: None,
            dependencies: None,
        }
    }

    pub fn copy<'b>(&self) -> TypeTranspilerOptions<'b> {
        TypeTranspilerOptions {
            assembler: None,
            target_register: self.target_register,
            target_page: self.target_page,
            dependencies: self.dependencies.clone(),
        }
    }

    pub fn with_dependencies(mut self, dependencies: Vec<usize>) -> Self {
        self.dependencies = Some(dependencies);
        self
    }

    pub fn assembler(&self) -> &Assembler {
        self.assembler.as_deref().expect("Assembler is not set")
    }

    pub fn assembler_mut(&mut self) -> &mut Assembler {
        self.assembler.as_deref_mut().expect("Assembler is not set")
    }

    pub fn target_register(&self) -> Registers {
        self.target_register
    }

    pub fn set_target_page(&mut self, target_page: usize) -> &mut Self {
        self.target_page = Some(target_page);
        self
    }

    pub fn set_dependencies(&mut self, dependencies: Vec<usize>) -> &mut Self {
        self.dependencies = Some(dependencies);
        self
    }

    pub fn set_target_register(&mut self, target_register: Registers) -> &mut Self {
        self.target_register = target_register;
        self
    }

    pub fn set_assembler(&mut self, assembler: &'a mut Assembler) -> &mut Self {
        self.assembler = Some(assembler);
        self
    }

    pub fn target_page(&self) -> usize {
        self.target_page.unwrap()
    }

    pub fn dependencies(&mut self) -> Option<Vec<usize>> {
        self.dependencies.clone()
    }
}

pub trait TypeTranspiler {
    /// Transpile a type
    /// ## Arguments
    /// * `options` - [`TypeTranspilerOptions`]
    fn transpile(&self, options: &mut TypeTranspilerOptions);
}

impl TypeTranspiler for Types {
    fn transpile(&self, options: &mut TypeTranspilerOptions) {
        match self {
            Types::Byte(b) => b.transpile(options),
            Types::Integer(i) => i.transpile(options),
            Types::Decimal(d) => d.transpile(options),
            Types::Bool(b) => b.transpile(options),
            Types::String(s) => s.transpile(options),
            Types::Char(c) => c.transpile(options),
            Types::Collective(c) => c.transpile(options),
            Types::Reference(r) => r.transpile(options),
            Types::BraceReference(br) => br.transpile(options),
            Types::EnumData(e) => e.transpile(options),
            Types::Operator(o) => o.transpile(options),
            Types::Cloak(c) => c.transpile(options),
            Types::Array(a) => a.transpile(options),
            Types::Function(f) => f.transpile(options),
            Types::FunctionParameter(fp) => fp.transpile(options),
            Types::ConstructorParameter(cp) => cp.transpile(options),
            Types::ClassCall(cc) => cc.transpile(options),
            Types::FunctionCall(fc) => fc.transpile(options),
            Types::SetterCall(_) => todo!(),
            Types::Void => void::Void.transpile(options),
            Types::NullResolver(nr) => nr.transpile(options),
            Types::Negative(n) => n.transpile(options),
            Types::VariableType(vt) => vt.transpile(options),
            Types::AsKeyword(ak) => ak.transpile(options),
            Types::ClassInstance(_) => todo!("TO BE REMOVED"),
            Types::Null => null::Null.transpile(options),
            Types::Dynamic => dynamic::Dynamic.transpile(options),
        }
    }
}

pub fn convert_type(
    types: &Types,
    _page_hash: Option<Vec<usize>>,
    arch: PlatformArchitecture,
) -> (ByteCodeTypes, Vec<u8>) {
    match types {
        Types::Byte(byte) => (ByteCodeTypes::Byte, byte.value.to_le_bytes().to_vec()),
        Types::Integer(integer) => (
            ByteCodeTypes::Integer,
            isize_to_le_bytes(integer.value, arch),
        ),
        Types::Decimal(decimal) => match decimal.value {
            ellie_core::definite::types::decimal::DecimalTypeEnum::Float(float_value) => {
                (ByteCodeTypes::Float, f32_to_le_bytes(float_value, arch))
            }
            ellie_core::definite::types::decimal::DecimalTypeEnum::Double(double_value) => {
                (ByteCodeTypes::Double, f64_to_le_bytes(double_value, arch))
            }
        },
        Types::Bool(bool) => (
            ByteCodeTypes::Bool,
            (bool.value as u8).to_le_bytes().to_vec(),
        ),
        Types::Char(e) => (ByteCodeTypes::Char, (e.value as u32).to_le_bytes().to_vec()),
        _ => unreachable!("This type is not convertable to raw type"),
    }
}
