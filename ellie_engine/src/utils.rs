use alloc::{string::String, vec::Vec};
use ellie_core::defs::Version;
#[cfg(feature = "compiler")]
use ellie_core::warning::Warning;
#[cfg(feature = "compiler")]
use ellie_parser::parser::Module;
#[cfg(feature = "compiler")]
use ellie_tokenizer::tokenizer::ResolvedImport;

#[cfg(feature = "compiler")]
/// Compiler output
/// * warnings `[Vec<Warning>]`
/// * module `[Module]`
pub struct CompileOutput {
    pub warnings: Vec<Warning>,
    pub module: Module,
}

/// Main program struct
pub struct MainProgram {
    /// Main file content
    pub file_content: String,
    /// Main file name
    pub file_name: String,
    /// Main file hash
    pub file_hash: usize,
    /// Program's main directory
    pub start_directory: String,
}

#[cfg(feature = "compiler")]
#[derive(Clone)]
/// EllieC settings
pub struct CompilerSettings {
    /// Module name
    pub name: String,
    /// Main file name
    pub file_name: String,
    /// Check module is library
    pub is_lib: bool,
    /// Module description
    pub description: String,
    /// Enable experimental features for Compiler
    pub experimental_features: bool,
    /// Module version
    pub version: Version,
    /// ByteCode architecture
    pub byte_code_architecture: ellie_core::defs::PlatformArchitecture,
}

/// Repository interface is channel for communication between compiler and code
#[cfg(feature = "compiler")]
pub trait ProgramRepository {
    /// Return main program and its hash
    /// ## Returns
    /// [`MainProgram`]
    fn read_main(&mut self) -> MainProgram;

    /// Read module by name and return ResolvedImport
    /// ## Parameters
    /// * `link_module` - Is import a link to module? [`bool`]
    /// * `current_path` - Current path [`String`]
    /// * `requested_path` - Requested path [`String`]
    /// ## Return
    /// [`ResolvedImport`]
    fn read_module(
        &mut self,
        link_module: bool,
        current_path: String,
        requested_path: String,
    ) -> ResolvedImport;
}

pub struct ModuleMap {
    pub module_name: String,
    pub module_path: Option<String>,
}
