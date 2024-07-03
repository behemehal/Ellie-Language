use core::hash::{Hash, Hasher};
use std::hash::DefaultHasher;

use ellie_core::{
    defs::{PlatformArchitecture, Version},
    error::Error,
    utils::PageExport,
};
use ellie_engine::{
    compiler, tokenizer,
    utils::{CompileOutput, CompilerSettings, MainProgram, ProgramRepository},
};
use ellie_tokenizer::tokenizer::{Page, ResolvedImport};

#[derive(Clone)]
pub struct Repository {
    pub code: String,
    pub main_hash: usize,
}

impl ProgramRepository for Repository {
    fn read_main(&mut self) -> MainProgram {
        let mut main_file_hasher = DefaultHasher::new();
        self.code.hash(&mut main_file_hasher);
        let first_page_hash = main_file_hasher.finish();
        self.main_hash = first_page_hash as usize;
        MainProgram {
            file_content: self.code.clone(),
            file_name: "playground.ei".to_string(),
            file_hash: first_page_hash as usize,
            start_directory: format!("<ellie_module_playground>",),
        }
    }

    fn read_module(
        &mut self,
        _link_module: bool,
        _current_path: String,
        _requested_path: String,
    ) -> ResolvedImport {
        panic!("Module resolver is not implemented on tests")
    }
}

pub fn tokenize_code(code: &str) -> (usize, PageExport<Page>) {
    let mut program_repository = Repository {
        code: code.to_string(),
        main_hash: 0,
    };
    let pages =
        tokenizer::tokenize_file(&mut program_repository).expect("should've been successful");
    (program_repository.main_hash, pages)
}

pub fn parse_code(
    (main_hash, pages): (usize, PageExport<Page>),
) -> Result<CompileOutput, Vec<ellie_core::error::Error>> {
    compiler::parse_pages(
        main_hash,
        vec![],
        pages,
        CompilerSettings {
            name: "ellie_bench".to_string(),
            file_name: String::from("ellie_bench.ei"),
            is_lib: false,
            description: String::from("No description"),
            experimental_features: false,
            byte_code_architecture: PlatformArchitecture::B32,
            version: Version::build_from_string(&"0.1.0".to_string()),
        },
    )
}
