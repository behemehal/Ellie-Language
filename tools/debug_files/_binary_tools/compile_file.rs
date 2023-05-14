use crate::terminal_utils::*;
use crate::utils::{CompilerSettings, MainProgram, ProgramRepository};
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use ellie_parser::parser;
use ellie_tokenizer::tokenizer::ResolvedImport;
use path_absolutize::Absolutize;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{fs, println};

use ellie_cli_utils::{outputs, utils};

#[derive(Clone)]
pub struct CliCompilerSettings {
    pub json_log: bool,
    pub output_type: OutputTypesSelector,
    pub warnings: bool,
    pub performance_info: bool,
    pub show_debug_lines: bool,
    pub exclude_std: bool,
    pub compiler_settings: CompilerSettings,
}

pub fn get_output_path(
    target_path: &Path,
    output_path: &Path,
    output_type: OutputTypesSelector,
) -> PathBuf {
    if output_path.is_dir() {
        let path = output_path
            .absolutize()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let mut file_name = target_path.file_name().unwrap().to_str().unwrap();

        if file_name.contains(".") {
            file_name = file_name.split(".").nth(0).unwrap();
        }

        Path::new(
            &(path
                + "/"
                + file_name
                + match output_type {
                    OutputTypesSelector::Bin => ".eib",
                    OutputTypesSelector::ByteCode => ".eic",
                    OutputTypesSelector::ByteCodeAsm => ".eia",
                    OutputTypesSelector::ByteCodeDebug => ".eig",
                    _ => ".json",
                }),
        )
        .to_owned()
    } else {
        output_path.to_owned()
    }
}

pub fn compile(
    target_path: &Path,
    output_path: &Path,
    modules: Vec<(parser::Module, Option<String>)>,
    cli_settings: CliCompilerSettings,
) {
    let exit_messages: Mutex<Vec<Box<dyn Fn()>>> = Mutex::new(vec![Box::new(|| {
        println!(
            "{}[?]{}: Ellie v{}",
            utils::Colors::Green,
            utils::Colors::Reset,
            crate::engine_constants::ELLIE_ENGINE_VERSION
        );
    })]);

    if cli_settings.compiler_settings.experimental_features {
        println!(
            "{}[!]{}: Experimental features are enabled.",
            utils::Colors::Red,
            utils::Colors::Reset,
        );
    }

    #[derive(Copy, Clone)]
    struct ColorTerminal;

    let color_terminal = ColorTerminal;

    impl ColorDisplay for ColorTerminal {
        fn color(&self, color: Colors) -> String {
            let color_id = match color {
                Colors::Black => "[30m",
                Colors::Red => "[31m",
                Colors::Green => "[32m",
                Colors::Yellow => "[33m",
                Colors::Blue => "[34m",
                Colors::Magenta => "[35m",
                Colors::Cyan => "[36m",
                Colors::White => "[37m",
                Colors::Reset => "[0m",
            };
            format!("{}{}", '\u{001b}', color_id)
        }

        fn text_style(&self, text_style: TextStyles) -> String {
            let type_id = match text_style {
                TextStyles::Bold => "[1m",
                TextStyles::Dim => "[2m",
                TextStyles::Italic => "[3m",
                TextStyles::Underline => "[4m",
            };
            format!("{}{}", '\u{001b}', type_id)
        }
    }

    #[derive(Clone)]
    struct Repository {
        used_modules: Vec<String>,
        main_hash: usize,
        target_path: String,
        cli_compiler_settings: CliCompilerSettings,
    }

    let mut _used_modules = vec![];

    if !cli_settings.exclude_std {
        _used_modules.push("ellieStd".to_string());
    }

    let mut program_repisotory = Repository {
        main_hash: 0,
        used_modules: _used_modules,
        target_path: target_path.to_str().unwrap().to_string(),
        cli_compiler_settings: cli_settings.clone(),
    };

    impl ProgramRepository for Repository {
        fn read_main(&mut self) -> MainProgram {
            match utils::read_file(self.target_path.clone()) {
                Ok(main_file_content) => {
                    let mut main_file_hasher = DefaultHasher::new();
                    main_file_content.hash(&mut main_file_hasher);
                    let first_page_hash = main_file_hasher.finish();
                    self.main_hash = first_page_hash as usize;
                    MainProgram {
                        file_content: main_file_content,
                        file_name: self
                            .cli_compiler_settings
                            .compiler_settings
                            .file_name
                            .clone(),
                        file_hash: first_page_hash as usize,
                        start_directory: format!(
                            "<ellie_module_{}>",
                            self.cli_compiler_settings.compiler_settings.name
                        ),
                    }
                }
                Err(err) => {
                    if self.cli_compiler_settings.json_log {
                        let mut cli_module_output = outputs::READ_FILE_ERROR.clone();
                        cli_module_output
                            .extra
                            .push(outputs::CliOuputExtraData { key: 0, value: err });
                        cli_module_output.extra.push(outputs::CliOuputExtraData {
                            key: 1,
                            value: self.target_path.clone(),
                        });
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&cli_module_output).unwrap()
                        );
                    } else {
                        println!("Unable to read file ~{} [{}]", self.target_path, err);
                    }
                    std::process::exit(1);
                }
            }
        }

        fn read_module(
            &mut self,
            link_module: bool,
            current_path: String,
            requested_path: String,
        ) -> ResolvedImport {
            let starter_name = format!(
                "<ellie_module_{}>",
                self.cli_compiler_settings.compiler_settings.name
            );

            if link_module {
                self.used_modules.push(requested_path);
                ResolvedImport {
                    found: true,
                    ..Default::default()
                }
            } else {
                match ellie_core::module_path::parse_module_import(&current_path, &requested_path) {
                    Ok(path) => {
                        let real_path = path
                            .replace(
                                &starter_name,
                                Path::new(&self.target_path)
                                    .absolutize()
                                    .unwrap()
                                    .parent()
                                    .unwrap()
                                    .to_str()
                                    .unwrap(),
                            )
                            .clone();
                        if Path::new(&real_path).exists() {
                            match utils::read_file(real_path) {
                                Ok(data) => {
                                    let mut hasher = DefaultHasher::new();
                                    data.hash(&mut hasher);
                                    ResolvedImport {
                                        found: true,
                                        matched: ellie_tokenizer::tokenizer::ImportType::Code(data),
                                        hash: hasher.finish().try_into().unwrap(),
                                        path,
                                        ..Default::default()
                                    }
                                }
                                Err(_) => ResolvedImport {
                                    found: false,
                                    resolve_error: "Cannot find file".to_string(),
                                    ..Default::default()
                                },
                            }
                        } else {
                            ResolvedImport {
                                found: false,
                                resolve_error: "Path does not exist".to_string(),
                                ..Default::default()
                            }
                        }
                    }
                    Err(e) => {
                        if e == 1 {
                            ResolvedImport {
                                found: false,
                                resolve_error: "Cannot access outside of workspace".to_string(),
                                ..Default::default()
                            }
                        } else {
                            unreachable!()
                        }
                    }
                }
            }
        }
    }

    let mut used_modules = Vec::new();
    for module_name in &program_repisotory.used_modules {
        if let Some(module) = modules
            .iter()
            .find(|(module, _)| module.name == *module_name)
        {
            used_modules.push(module.clone());
        } else {
            if program_repisotory.cli_compiler_settings.json_log {
                let mut cli_module_output = outputs::FAILED_TO_FIND_MODULE.clone();
                cli_module_output.extra.push(outputs::CliOuputExtraData {
                    key: 0,
                    value: module_name.to_string(),
                });
                println!(
                    "{}",
                    serde_json::to_string_pretty(&cli_module_output).unwrap()
                );
            } else {
                println!(
                    "{}[Internal Error]{}: Could not find imported module {}'{}'{}",
                    color_terminal.color(Colors::Red),
                    color_terminal.color(Colors::Reset),
                    color_terminal.color(Colors::Cyan),
                    module_name,
                    color_terminal.color(Colors::Reset),
                );
            }
            std::process::exit(1);
        }
    }
    let starter_name = format!("<ellie_module_{}>", cli_settings.compiler_settings.name);

    match crate::tokenizer::tokenize_file(&mut program_repisotory) {
        Ok(pages) => {
            match crate::compiler::parse_pages(
                program_repisotory.main_hash,
                used_modules,
                pages,
                cli_settings.compiler_settings.clone(),
            ) {
                Ok(compile_output) => {
                    if !compile_output.warnings.is_empty() {
                        if cli_settings.json_log {
                            let mut output = outputs::COMPILER_WARNINGS.clone();
                            output.extra.push(outputs::CliOuputExtraData {
                                key: "warnings".to_string(),
                                value: compile_output.warnings.clone(),
                            });
                            println!("{}", serde_json::to_string(&output).unwrap());
                        } else {
                            println!(
                                "{}",
                                print_warnings(
                                    &compile_output.warnings,
                                    |path| {
                                        let path_starter = path.split("/").next().unwrap();
                                        let virtual_path_identifier =
                                            match path_starter.split("<ellie_module_").last() {
                                                Some(e) => e.split(">").next().unwrap(),
                                                None => "",
                                            };
                                        if path_starter == starter_name {
                                            let real_path = path
                                                .replace(
                                                    &starter_name,
                                                    Path::new(target_path)
                                                        .absolutize()
                                                        .unwrap()
                                                        .parent()
                                                        .unwrap()
                                                        .to_str()
                                                        .unwrap(),
                                                )
                                                .clone();
                                            match utils::read_file(real_path) {
                                                Ok(e) => e,
                                                Err(err) => {
                                                    println!(
                                                    "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                                    path,
                                                    color_terminal.color(Colors::Red),
                                                    err,
                                                    color_terminal.color(Colors::Reset)
                                                );
                                                    std::process::exit(1);
                                                }
                                            }
                                        } else if let Some((_, module_path)) =
                                            modules.iter().find(|(module, _)| {
                                                module.name == virtual_path_identifier
                                            })
                                        {
                                            let module_path = module_path.clone().unwrap();
                                            let real_path =
                                                path.replace(&path_starter, &module_path).clone();
                                            match utils::read_file(real_path) {
                                                Ok(e) => e,
                                                Err(err) => {
                                                    println!(
                                                    "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                                    path,
                                                    color_terminal.color(Colors::Red),
                                                    err,
                                                    color_terminal.color(Colors::Reset)
                                                );
                                                    std::process::exit(1);
                                                }
                                            }
                                        } else {
                                            panic!(
                                            "Failed to ouput error. Cannot identify module '{}'",
                                            path,
                                        );
                                        }
                                    },
                                    |path| {
                                        let path_starter = path.split("/").next().unwrap();
                                        let virtual_path_identifier =
                                            match path_starter.split("<ellie_module_").last() {
                                                Some(e) => e.split(">").next().unwrap(),
                                                None => "",
                                            };
                                        if path_starter == starter_name {
                                            path.replace(
                                                &starter_name,
                                                Path::new(target_path)
                                                    .absolutize()
                                                    .unwrap()
                                                    .parent()
                                                    .unwrap()
                                                    .to_str()
                                                    .unwrap(),
                                            )
                                            .clone()
                                        } else if let Some((_, module_path)) =
                                            modules.iter().find(|(module, _)| {
                                                module.name == virtual_path_identifier
                                            })
                                        {
                                            let module_path = module_path.clone().unwrap();
                                            path.replace(&path_starter, &module_path).clone()
                                        } else {
                                            panic!(
                                            "Failed to ouput error. Cannot identify module '{}'",
                                            path,
                                        );
                                        }
                                    },
                                    color_terminal,
                                )
                            );
                        }
                    }

                    let output_path = &get_output_path(
                        target_path,
                        output_path,
                        cli_settings.output_type.clone(),
                    );

                    let dbg_output_path = output_path.file_name().unwrap().to_str().unwrap();

                    let dbg_output_path = &get_output_path(
                        target_path,
                        Path::new(&output_path.to_str().unwrap().replace(dbg_output_path, "")),
                        OutputTypesSelector::ByteCodeDebug,
                    );

                    let mut module_maps = vec![(
                        compile_output.module.name.clone(),
                        Some(
                            Path::new(target_path)
                                .absolutize()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string(),
                        ),
                    )];

                    module_maps.extend(
                        modules
                            .iter()
                            .map(|(module, path)| (module.name.clone(), path.clone()))
                            .collect::<Vec<_>>(),
                    );

                    match cli_settings.output_type {
                        OutputTypesSelector::Bin => {
                            let bytes = bincode::serialize(&compile_output.module).unwrap();
                            if let Err(write_error) = fs::write(output_path, bytes) {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_FILE_ERROR.clone();

                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: "path".to_string(),
                                        value: format!("{:?}", write_error),
                                    });
                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "\nFailed to write output. [{}{:?}{}]",
                                        color_terminal.color(Colors::Red),
                                        write_error,
                                        color_terminal.color(Colors::Reset),
                                    );
                                }
                            } else {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_BINARY_SUCCEDED.clone();
                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: 0,
                                        value: output_path
                                            .absolutize()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_owned(),
                                    });
                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "{}[!]{}: Binary output written to {}{}{}",
                                        color_terminal.color(Colors::Green),
                                        color_terminal.color(Colors::Reset),
                                        color_terminal.color(Colors::Yellow),
                                        output_path.absolutize().unwrap().to_str().unwrap(),
                                        color_terminal.color(Colors::Reset)
                                    );
                                }
                            }
                        }
                        OutputTypesSelector::DependencyAnalysis => {
                            println!(
                                "{}[Error]{}: Dependency analysis output is not supported yet.",
                                color_terminal.color(Colors::Red),
                                color_terminal.color(Colors::Reset),
                            );
                            std::process::exit(1);
                        }
                        OutputTypesSelector::Json => {
                            let json = serde_json::to_string(&compile_output.module).unwrap();
                            if let Err(write_error) = fs::write(&output_path, json) {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_FILE_ERROR.clone();
                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: "path".to_string(),
                                        value: format!("{:?}", write_error),
                                    });

                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "\nFailed to write output. [{}{:?}{}]",
                                        color_terminal.color(Colors::Red),
                                        write_error,
                                        color_terminal.color(Colors::Reset),
                                    );
                                }
                            } else {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_JSON_SUCCEDED.clone();
                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: 0,
                                        value: output_path
                                            .absolutize()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_owned(),
                                    });
                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "{}[!]{}: JSON output written to {}{}{}",
                                        color_terminal.color(Colors::Green),
                                        color_terminal.color(Colors::Reset),
                                        color_terminal.color(Colors::Yellow),
                                        output_path.absolutize().unwrap().to_str().unwrap(),
                                        color_terminal.color(Colors::Reset),
                                    );
                                }
                            }
                        }
                        OutputTypesSelector::ByteCode => {
                            if !cli_settings.json_log {
                                println!(
                                    "{}[?]{}: ByteCode compiling to {} bit architecture",
                                    color_terminal.color(Colors::Green),
                                    color_terminal.color(Colors::Reset),
                                    match cli_settings.compiler_settings.byte_code_architecture {
                                        ellie_core::defs::PlatformArchitecture::B16 => "16",
                                        ellie_core::defs::PlatformArchitecture::B32 => "32",
                                        ellie_core::defs::PlatformArchitecture::B64 => "64",
                                    }
                                );
                            }
                            let mut assembler = ellie_bytecode::assembler::Assembler::new(
                                compile_output.module,
                                ellie_bytecode::assembler::PlatformAttributes {
                                    architecture: ellie_core::defs::PlatformArchitecture::B64, //64 Bit Limit
                                    memory_size: 512000, //512kb memory limit
                                },
                            );
                            let assembler_result = assembler.assemble(module_maps);
                            let mut output_file = File::create(output_path).unwrap_or_else(|err| {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_FILE_ERROR.clone();
                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: "path".to_string(),
                                        value: format!("{:?}", err),
                                    });
                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "\nFailed to create file {}{}{}. [{}{:?}{}]",
                                        color_terminal.color(Colors::Cyan),
                                        output_path.absolutize().unwrap().to_str().unwrap(),
                                        color_terminal.color(Colors::Reset),
                                        color_terminal.color(Colors::Red),
                                        err,
                                        color_terminal.color(Colors::Reset),
                                    );
                                }
                                std::process::exit(1);
                            });
                            let mut dbg_file =
                                File::create(dbg_output_path).unwrap_or_else(|err| {
                                    if cli_settings.json_log {
                                        let mut output = outputs::WRITE_FILE_ERROR.clone();
                                        output.extra.push(outputs::CliOuputExtraData {
                                            key: "path".to_string(),
                                            value: format!("{:?}", err),
                                        });
                                        println!("{}", serde_json::to_string(&output).unwrap())
                                    } else {
                                        println!(
                                            "\nFailed to create file {}{}{}. [{}{:?}{}]",
                                            color_terminal.color(Colors::Cyan),
                                            dbg_output_path.absolutize().unwrap().to_str().unwrap(),
                                            color_terminal.color(Colors::Reset),
                                            color_terminal.color(Colors::Red),
                                            err,
                                            color_terminal.color(Colors::Reset),
                                        );
                                    }
                                    std::process::exit(1);
                                });
                            assembler_result.render_binary(&mut output_file, &mut dbg_file);
                            if cli_settings.json_log {
                                let mut output = outputs::WRITE_BYTE_CODE_SUCCEDED.clone();
                                output.extra.push(outputs::CliOuputExtraData {
                                    key: 0,
                                    value: output_path
                                        .absolutize()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_owned(),
                                });
                                println!("{}", serde_json::to_string(&output).unwrap())
                            } else {
                                println!(
                                    "{}[!]{}: ByteCode output written to {}{}{}",
                                    color_terminal.color(Colors::Green),
                                    color_terminal.color(Colors::Reset),
                                    color_terminal.color(Colors::Yellow),
                                    output_path.absolutize().unwrap().to_str().unwrap(),
                                    color_terminal.color(Colors::Reset),
                                );
                                println!(
                                    "{}[!]{}: ByteCode debug file written to {}{}{}",
                                    color_terminal.color(Colors::Green),
                                    color_terminal.color(Colors::Reset),
                                    color_terminal.color(Colors::Yellow),
                                    dbg_output_path.absolutize().unwrap().to_str().unwrap(),
                                    color_terminal.color(Colors::Reset),
                                );
                            }
                        }
                        OutputTypesSelector::ByteCodeAsm => {
                            if !cli_settings.json_log {
                                println!(
                                    "{}[?]{}: ByteCode compiling to {} bit architecture",
                                    color_terminal.color(Colors::Green),
                                    color_terminal.color(Colors::Reset),
                                    match cli_settings.compiler_settings.byte_code_architecture {
                                        ellie_core::defs::PlatformArchitecture::B16 => "16",
                                        ellie_core::defs::PlatformArchitecture::B32 => "32",
                                        ellie_core::defs::PlatformArchitecture::B64 => "64",
                                    }
                                );
                            }
                            let mut assembler = ellie_bytecode::assembler::Assembler::new(
                                compile_output.module,
                                ellie_bytecode::assembler::PlatformAttributes {
                                    architecture: ellie_core::defs::PlatformArchitecture::B64, //64 Bit Limit
                                    memory_size: 512000, //512kb memory limit
                                },
                            );
                            let assembler_result = assembler.assemble(module_maps);
                            let output_file = File::create(output_path).unwrap_or_else(|err| {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_FILE_ERROR.clone();
                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: "path".to_string(),
                                        value: format!("{:?}", err),
                                    });
                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "\nFailed to create file {}{}{}. [{}{:?}{}]",
                                        color_terminal.color(Colors::Cyan),
                                        output_path.absolutize().unwrap().to_str().unwrap(),
                                        color_terminal.color(Colors::Reset),
                                        color_terminal.color(Colors::Red),
                                        err,
                                        color_terminal.color(Colors::Reset),
                                    );
                                }
                                std::process::exit(1);
                            });
                            assembler_result.alternate_render(output_file);

                            if cli_settings.json_log {
                                let mut output = outputs::WRITE_BYTE_CODE_ASM_SUCCEDED.clone();
                                output.extra.push(outputs::CliOuputExtraData {
                                    key: 0,
                                    value: output_path
                                        .absolutize()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_owned(),
                                });
                                println!("{}", serde_json::to_string(&output).unwrap())
                            } else {
                                println!(
                                    "{}[!]{}: ByteCodeAsm output written to {}{}{}",
                                    color_terminal.color(Colors::Green),
                                    color_terminal.color(Colors::Reset),
                                    color_terminal.color(Colors::Yellow),
                                    output_path.absolutize().unwrap().to_str().unwrap(),
                                    color_terminal.color(Colors::Reset),
                                );
                            }
                        }
                        OutputTypesSelector::ByteCodeDebug => unreachable!(),
                        OutputTypesSelector::Nop => (),
                    }
                    if !cli_settings.json_log {
                        for message in exit_messages.lock().unwrap().iter() {
                            (message)();
                        }
                    }
                }
                Err(errors) => {
                    if cli_settings.json_log {
                        let mut output = outputs::COMPILER_ERRORS.clone();
                        output.extra.push(outputs::CliOuputExtraData {
                            key: "errors".to_string(),
                            value: errors,
                        });
                        println!("{}", serde_json::to_string(&output).unwrap());
                    } else {
                        println!(
                            "{}",
                            print_errors(
                                &errors,
                                |path| {
                                    let path_starter = path.split("/").next().unwrap();
                                    let virtual_path_identifier =
                                        match path_starter.split("<ellie_module_").last() {
                                            Some(e) => e.split(">").next().unwrap(),
                                            None => "",
                                        };
                                    if path_starter == starter_name {
                                        let real_path = path
                                            .replace(
                                                &starter_name,
                                                Path::new(target_path)
                                                    .absolutize()
                                                    .unwrap()
                                                    .parent()
                                                    .unwrap()
                                                    .to_str()
                                                    .unwrap(),
                                            )
                                            .clone();
                                        match utils::read_file(real_path) {
                                            Ok(e) => e,
                                            Err(err) => {
                                                panic!(
                                            "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                            path,
                                            utils::Colors::Red,
                                            err,
                                            utils::Colors::Reset
                                        );
                                            }
                                        }
                                    } else if let Some((_, module_path)) = modules
                                        .iter()
                                        .find(|(module, _)| module.name == virtual_path_identifier)
                                    {
                                        if let Some(module_path) = module_path.clone() {
                                            let real_path =
                                                path.replace(&path_starter, &module_path).clone();
                                            match utils::read_file(real_path.clone()) {
                                                Ok(e) => e,
                                                Err(err) => {
                                                    exit_messages.lock().unwrap().push(Box::new(move || {
                                                    println!(
                                                        "{}[!]{}: Failed to read module targeted code director y: {}{}{} - [{}]",
                                                        utils::Colors::Red,
                                                        utils::Colors::Reset,
                                                        utils::Colors::Yellow,
                                                        real_path.clone(),
                                                        utils::Colors::Reset,
                                                        err,
                                                    );
                                                }));
                                                    module_path
                                                }
                                            }
                                        } else {
                                            exit_messages.lock().unwrap().push(Box::new(move || {
                                                println!(
                                                    "{}[!]{}: Path not provided no output will be shown: {}{}{}",
                                                    utils::Colors::Red,
                                                    utils::Colors::Reset,
                                                    utils::Colors::Yellow,
                                                    path,
                                                    utils::Colors::Reset,
                                                );
                                            }));
                                            "No output path provided".to_string()
                                        }
                                    } else {
                                        panic!(
                                            "Failed to ouput error. Cannot identify module '{}'",
                                            path,
                                        );
                                    }
                                },
                                cli_settings.show_debug_lines,
                                |path| {
                                    let path_starter = path.split("/").next().unwrap();
                                    let virtual_path_identifier =
                                        match path_starter.split("<ellie_module_").last() {
                                            Some(e) => e.split(">").next().unwrap(),
                                            None => "",
                                        };
                                    if path_starter == starter_name {
                                        path.replace(
                                            &starter_name,
                                            Path::new(target_path)
                                                .absolutize()
                                                .unwrap()
                                                .parent()
                                                .unwrap()
                                                .to_str()
                                                .unwrap(),
                                        )
                                        .clone()
                                    } else if let Some((_, module_path)) = modules
                                        .iter()
                                        .find(|(module, _)| module.name == virtual_path_identifier)
                                    {
                                        if let Some(module_path) = module_path.clone() {
                                            path.replace(&path_starter, &module_path).clone()
                                        } else {
                                            exit_messages.lock().unwrap().push(Box::new(move || {
                                                println!(
                                                    "{}[!]{}: Path not provided no output will be shown: {}{}{}",
                                                    utils::Colors::Red,
                                                    utils::Colors::Reset,
                                                    utils::Colors::Yellow,
                                                    path,
                                                    utils::Colors::Reset,
                                                );
                                            }));
                                            "[No output path provided]".to_string()
                                        }
                                    } else {
                                        panic!(
                                            "Failed to ouput error. Cannot identify module '{}'",
                                            path,
                                        );
                                    }
                                },
                                color_terminal,
                            )
                        );
                        for message in exit_messages.lock().unwrap().iter() {
                            (message)();
                        }
                    }
                }
            }
        }
        Err(pager_errors) => {
            if cli_settings.json_log {
                let mut output = outputs::COMPILER_ERRORS.clone();
                output.extra.push(outputs::CliOuputExtraData {
                    key: "errors".to_string(),
                    value: pager_errors,
                });
                println!("{}", serde_json::to_string(&output).unwrap());
            } else {
                println!(
                    "{}",
                    print_errors(
                        &pager_errors,
                        |path| match utils::read_file(
                            &path.replace(
                                &starter_name,
                                Path::new(target_path)
                                    .absolutize()
                                    .unwrap()
                                    .parent()
                                    .unwrap()
                                    .to_str()
                                    .unwrap(),
                            ),
                        ) {
                            Ok(e) => e,
                            Err(err) => {
                                println!(
                                    "{}[Internal Error]{} Cannot build error, read file failed '{}' {}[{}]{}",
                                    color_terminal.color(Colors::Red),
                                    color_terminal.color(Colors::Reset),
                                    path,
                                    color_terminal.color(Colors::Red),
                                    err,
                                    color_terminal.color(Colors::Reset),
                                );
                                std::process::exit(1);
                            }
                        },
                        cli_settings.show_debug_lines,
                        |path| {
                            path.replace(
                                &starter_name,
                                Path::new(target_path)
                                    .absolutize()
                                    .unwrap()
                                    .parent()
                                    .unwrap()
                                    .to_str()
                                    .unwrap(),
                            )
                            .to_string()
                        },
                        color_terminal
                    )
                );
                for message in exit_messages.lock().unwrap().iter() {
                    (message)();
                }
            }
        }
    }
}
