use ellie_core::defs::Version;
use ellie_vm::program::Program;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

use crate::cli_outputs;
use crate::cli_utils;
use crate::cli_utils::ReadErrorText;
use path_absolutize::Absolutize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use ellie_vm::{
    utils::{ProgramReader, Reader},
    vm::VM,
};

pub struct VmSettings {
    pub json_log: bool,
    pub warnings: bool,
    pub architecture: ellie_bytecode::assembler::PlatformArchitecture,
}

pub struct RFile<'a, T> {
    pub source: &'a mut T,
}

impl<'a, T> RFile<'a, T> {
    fn new(source: &'a mut T) -> Self {
        RFile { source }
    }
}

impl<'a, T> Reader for RFile<'a, T>
where
    T: Read,
{
    fn read(&mut self) -> Option<u8> {
        let mut b = [0u8];
        match self.source.read_exact(&mut b) {
            Ok(_) => Some(b[0]),
            Err(_) => None,
        }
    }
}

pub fn run(target_path: &Path, vm_settings: VmSettings) {
    let mut program = File::open(target_path).unwrap();

    let mut reader = RFile::new(&mut program);
    let mut program_reader = ProgramReader::new(&mut reader);
    let program = match Program::build_from_reader(&mut program_reader) {
        Ok(e) => e,
        Err(e) => {
            println!(
                "{}Error:{} Failed to run program '{}{}{}'",
                cli_utils::Colors::Red,
                cli_utils::Colors::Reset,
                cli_utils::Colors::Yellow,
                ReadErrorText(e),
                cli_utils::Colors::Reset,
            );
            std::process::exit(1);
        }
    };

    let mut vm = VM::new(vm_settings.architecture.get_code());
    vm.execute(program);
    //vm.execute(target_path);
}
