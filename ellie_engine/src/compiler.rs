use crate::utils::{CompileOutput, CompilerSettings};
use alloc::{borrow::ToOwned, string::String, vec::Vec};
use ellie_core::{error::Error, utils::PageExport};
use ellie_parser::parser;
use ellie_tokenizer::tokenizer::Page;

/// Tokenize file
/// ## Parameters
/// * `program_repository` - Program repository a struct that implements [`ProgramRepository`] trait
/// ## Returns
/// If no syntax error, return [`PageExport<Page>`] of tokenized file which is parsed pages.
/// If syntax error, return [`Vec<Error>`]
/// [`Result<PageExport<Page>, Vec<Error>>`]
pub fn parse_pages(
    main_hash: usize,
    modules: Vec<(parser::Module, Option<String>)>,
    tokenized_pages: PageExport<Page>,
    compiler_settings: CompilerSettings,
) -> Result<CompileOutput, Vec<Error>> {
    let mut parser = parser::Parser::new(
        tokenized_pages,
        main_hash,
        compiler_settings.version,
        compiler_settings.name,
        compiler_settings.description,
        compiler_settings.is_lib,
        compiler_settings.experimental_features,
        ellie_core::defs::Version::build_from_string(
            crate::engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
        ),
    );

    for (module, _) in modules.iter() {
        parser.import_module(module.clone());
    }

    let module = parser.parse();

    if parser.informations.has_no_errors() {
        Ok(CompileOutput {
            warnings: parser.informations.warnings.clone(),
            module,
        })
    } else {
        Err(parser.informations.errors)
    }
}
