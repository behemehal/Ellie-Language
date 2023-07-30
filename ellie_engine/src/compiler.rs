use crate::utils::{CompileOutput, CompilerSettings};
use alloc::{borrow::ToOwned, string::String, vec::Vec};
use ellie_core::{defs::Version, error::Error, utils::PageExport};
use ellie_parser::parser;
use ellie_tokenizer::tokenizer::Page;

/// Tokenize file
/// ## Parameters
/// * `main_hash` - Hash of main file
/// * `modules` - Modules to import
/// * `compiler_settings` - Compiler settings
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
        Version::build_from_string(crate::engine_constants::ELLIE_ENGINE_VERSION.to_owned()),
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

#[macro_export]
macro_rules! parseText {
    ($text:expr) => {{
        use ellie_engine::ellie_core::defs;
        use ellie_engine::ellie_parser::parser;
        use ellie_engine::ellie_tokenizer::tokenizer::{Pager, ResolvedImport};
        use ellie_engine::utils::{CompileOutput, MainProgram, ProgramRepository};

        #[derive(Clone)]
        struct Repository {
            target_path: String,
        }
        let mut program_repository = Repository {
            target_path: String::from("./main.ei"),
        };

        impl ProgramRepository for Repository {
            fn read_main(&mut self) -> MainProgram {
                let text = $text;

                MainProgram {
                    file_content: text.to_string(),
                    file_name: "main.ei".to_string(),
                    file_hash: 0,
                    start_directory: format!("<ellie_module_main>"),
                }
            }

            fn read_module(
                &mut self,
                link_module: bool,
                current_path: String,
                requested_path: String,
            ) -> ResolvedImport {
                ResolvedImport {
                    found: false,
                    resolve_error: "Module resolver is not implemented on macros".to_owned(),
                    ..Default::default()
                }
            }
        }
        let main_program = program_repository.read_main();

        let mut pager = Pager::new(
            main_program.file_content,
            main_program.file_name,
            format!("{}/", main_program.start_directory),
            move |link_module, path, module_identifier| {
                program_repository.read_module(link_module, path.clone(), module_identifier)
            },
            main_program.file_hash,
        );

        match pager.run() {
            Ok(_) => {
                let mut parser = parser::Parser::new(
                    pager.pages,
                    0,
                    defs::Version::build_from_string("1.0.0".to_string()),
                    "main".to_string(),
                    "".to_string(),
                    false,
                    false,
                    defs::Version::build_from_string(
                        ellie_engine::engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                    ),
                );
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
            Err(errors) => Err(errors),
        }
    }};
}
