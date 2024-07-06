use alloc::vec::Vec;
use ellie_bytecode::assembler::{self, AssembleResult, PlatformAttributes};
use ellie_core::defs::ModuleMap;
use ellie_parser::utils::Module;

/// Transpile parsed code
/// ## Parameters
/// * `module` - Module to transpile [`Module`]
/// * `platform_attributes` - Platform attributes [`PlatformAttributes`]
/// * * `module_maps` - Module maps [`Vec<ModuleMap>`]
/// ## Returns
/// [`AssembleResult`] of transpiled code
pub fn transpile_parsed_code(
    module: Module,
    platform_attributes: PlatformAttributes,
    module_maps: Vec<ModuleMap>,
) -> AssembleResult {
    let mut assembler = assembler::Assembler::new(module, platform_attributes);
    assembler.assemble(module_maps)
}

/// Transpile given text macro
/// ## Parameters
/// * `text` - Text to transpile [`&str`]
/// * `platform_attributes` - Platform attributes [`PlatformAttributes`]
/// * * `module_maps` - Module maps [`Vec<ModuleMap>`]
/// ## Returns
/// [`AssembleResult`] of transpiled code
/// ## Example
/// ```rust
/// use ellie_engine::byteCode;
/// let text = r#"\
/// fn main() {
///    print("Hello, World!");
/// }
/// "#;
/// let result = byteCode::transpileText!(text, PlatformAttributes::default(), vec![]);
/// ```
#[macro_export]
macro_rules! transpileText {
    ($text:expr, $platform_attributes:expr, $module_maps:expr) => {{
        use ellie_engine::{
            ellie_bytecode::assembler,
            ellie_core::defs::{ModuleMap},
            ellie_parser::utils::Module,
        };

        let text = $text;
        let module = ellie_parser::parseText!(text);
        let result = ellie_engine::byteCode::transpile_parsed_code(module, $platform_attributes, $module_maps);
        result
    }};
}