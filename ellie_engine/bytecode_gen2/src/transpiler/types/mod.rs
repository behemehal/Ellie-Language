pub mod integer;
use crate::{
    assembler::Assembler, operand::Registers, types::Types as ByteCodeTypes, utils::{f32_to_le_bytes, f64_to_le_bytes, isize_to_le_bytes}
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
            Types::Integer(i) => i.transpile(options),
            _ => todo!(),
        }
    }
}

pub fn convert_type(
    types: &Types,
    _page_hash: Option<Vec<usize>>,
    arch: PlatformArchitecture,
) -> (ByteCodeTypes, Vec<u8>) {
    match types {
        Types::Integer(integer) => (
            ByteCodeTypes::Integer,
            isize_to_le_bytes(integer.value, arch),
        ),
        _ => unreachable!("This type is not convertable to raw type"),
    }
}
