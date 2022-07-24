use crate::utils::ProgramRepository;
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
