use crate::utils::ProgramRepository;
use alloc::{format, vec::Vec};
use ellie_core::{error::Error, utils::PageExport};
use ellie_tokenizer::tokenizer::{Page, Pager};

/// Tokenize file
/// ## Parameters
/// * `program_repository` - Program repository a struct that implements [`ProgramRepository`] trait
/// ## Returns
/// If no syntax error, return [`PageExport<Page>`] of tokenized file which is parsed pages.
/// If syntax error, return [`Vec<Error>`]
/// [`Result<PageExport<Page>, Vec<Error>>`]
pub fn tokenize_file(
    program_repository: &mut dyn ProgramRepository,
) -> Result<PageExport<Page>, Vec<Error>> {
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
        Ok(_) => Ok(pager.pages),
        Err(errors) => Err(errors),
    }
}

/// Tokenize given text
/// ## Parameters
/// * `text` - Text to tokenize [`&str`]
#[macro_export]
macro_rules! tokenizeText {
    ($text:expr) => {{
        use ellie_engine::{
            ellie_core::defs,
            ellie_tokenizer::tokenizer::{Pager, ResolvedImport},
            utils::{MainProgram, ProgramRepository},
        };

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

        pager.run()
    }};
}
