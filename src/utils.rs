use ellie_core::{defs::Version, warning::Warning};
use ellie_parser::parser::Module;
use ellie_tokenizer::tokenizer::ResolvedImport;

pub struct CompileOutput {
    pub warnings: Vec<Warning>,
    pub module: Module,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputTypesSelector {
    /// Outputs module in binary format. This is the default export mode for modules.
    Bin,
    /// Not supported yet
    DependencyAnalysis,
    /// Compiled module as json
    Json,
    /// ByteCode binary format
    ByteCode,
    /// ByteCode assembly text
    ByteCodeAsm,
    /// ByteCode debug file
    ByteCodeDebug,
    /// No output
    Nop,
}

pub struct MainProgram {
    pub file_content: String,
    pub file_name: String,
    pub file_hash: usize,
    pub start_directory: String,
}

#[derive(Clone)]
pub struct CompilerSettings {
    pub name: String,
    pub file_name: String,
    pub is_lib: bool,
    pub description: String,
    pub experimental_features: bool,
    pub version: Version,
    pub byte_code_architecture: ellie_core::defs::PlatformArchitecture,
}

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
