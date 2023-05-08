#[cfg(feature = "std")]
use std::io::Read;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use ellie_core::defs::{CursorPosition, DebugHeader, DebugHeaderType};
#[cfg(feature = "std")]
use ellie_vm::utils::Reader;
use ellie_vm::{
    program::{Program, ProgramReadErrors},
    utils::ProgramReader,
};

#[cfg(feature = "std")]
pub struct RFile<'a, T> {
    pub source: &'a mut T,
}

#[cfg(feature = "std")]
impl<'a, T> RFile<'a, T> {
    pub fn new(source: &'a mut T) -> Self {
        RFile { source }
    }
}

#[cfg(feature = "std")]
impl<'a, T> Reader for RFile<'a, T>
where
    T: Read,
{
    fn read(&mut self) -> Option<u8> {
        let mut b = [0u8];
        match self.source.read_exact(&mut b) {
            Ok(_) => {
                return Some(b[0]);
            }
            Err(_) => None,
        }
    }
}

use ellie_core::defs::{DebugInfo, ModuleMap};

/// Parse debug file
/// ## Parameters
/// * `dbg_file` - Debug file [`String`]
/// ## Returns
/// If parse success, return [`DebugInfo`] else return error inside [`String`]
/// ## Example
/// ```rust
/// use ellie_engine::vm;
/// let file = r#"\
// main_file: /Users/ahmtcn123/Desktop/Projects/Ellie-Language/test_dir
///---
///0:1:<ellie_module_main>/main.ei:main:3:0:9:0:15608529263637909756
///"#;
/// let debug_info = vm::parse_debug_file(file.to_string());
/// ```
pub fn parse_debug_file(dbg_file: String) -> Result<DebugInfo, String> {
    let mut dbg_headers = dbg_file.split("\n").collect::<Vec<_>>().into_iter();
    let mut module_maps_ended = false;

    let mut module_maps = Vec::new();

    while let Some(line) = dbg_headers.next() {
        if line == "---" {
            module_maps_ended = true;
            break;
        } else {
            let line = line.split(":").collect::<Vec<_>>();
            let module_name = line[0].to_string();
            let path = line[1..].join(":").to_string().trim().to_string();
            module_maps.push(ModuleMap {
                module_name,
                module_path: if path == "-" { None } else { Some(path) },
            });
        }
    }

    if !module_maps_ended {
        return Err(format!("Broken debug header, line: {}", module_maps.len()));
    }

    let mut debug_headers = Vec::new();

    for (idx, header) in dbg_headers.enumerate() {
        let line = header.split(":").collect::<Vec<_>>();
        if line.len() != 9 {
            return Err(format!("Broken debug header, line: {}", idx + 1));
        }

        let start_end = (
            match line[0].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!("Broken debug header, line: {}", idx + 1));
                }
            },
            match line[1].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!("Broken debug header, line: {}", idx + 1));
                }
            },
        );

        let pos_range_start = (
            match line[4].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!("Broken debug header, line: {}", idx + 1));
                }
            },
            match line[5].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!("Broken debug header, line: {}", idx + 1));
                }
            },
        );

        let pos_range_end = (
            match line[6].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!("Broken debug header, line: {}", idx + 1));
                }
            },
            match line[7].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!("Broken debug header, line: {}", idx + 1));
                }
            },
        );

        let pos = ellie_core::defs::Cursor {
            range_start: CursorPosition(pos_range_start.0, pos_range_start.1),
            range_end: CursorPosition(pos_range_end.0, pos_range_end.1),
        };

        let hash = match line[8].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!("Broken debug header, line: {}", idx + 1));
            }
        };

        debug_headers.push(DebugHeader {
            start_end,
            module: line[2].to_string(),
            name: line[3].to_string(),
            pos,
            rtype: DebugHeaderType::Variable,
            hash,
        })
    }

    Ok(DebugInfo {
        module_map: module_maps,
        debug_headers,
    })
}

//Deprecated
#[deprecated]
pub fn read_program<T: ellie_vm::utils::Reader>(
    program_reader: &mut T,
) -> Result<Program, ProgramReadErrors> {
    let mut program_reader = ProgramReader::new(program_reader);
    let mut program = Program::new();
    match program.build_from_reader(&mut program_reader) {
        Ok(_) => Ok(program),
        Err(e) => Err(e),
    }
}
