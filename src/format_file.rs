use ellie_core::defs::Version;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;

use crate::cli_outputs;
use crate::cli_utils;
use ellie_parser::parser;
use ellie_tokenizer::tokenizer::{self, ResolvedImport};
use path_absolutize::Absolutize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct FormatterSettings {
    pub json_log: bool,
}

pub fn get_output_path(
    target_path: &Path,
    output_path: &Path,
    output_type: cli_utils::OutputTypes,
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
                    cli_utils::OutputTypes::Bin => ".eib",
                    cli_utils::OutputTypes::ByteCode => ".eic",
                    cli_utils::OutputTypes::ByteCodeAsm => ".eia",
                    _ => ".json",
                }),
        )
        .to_owned()
    } else {
        output_path.to_owned()
    }
}

pub fn format(
    target_path: &Path,
    modules: Vec<(parser::Module, Option<String>)>,
    compiler_settings: CompilerSettings,
) {
    let starter_name = format!("<ellie_module_{}>", compiler_settings.name);
    match cli_utils::read_file(target_path) {
        Ok(main_file_content) => {
            let mut main_file_hasher = DefaultHasher::new();
            main_file_content.hash(&mut main_file_hasher);
            let first_page_hash = main_file_hasher.finish();
            //Auto import 'ellieStd'
            let used_modules = Mutex::new(vec!["ellieStd".to_string()]);
            let mut pager = tokenizer::Pager::new(
                main_file_content,
                compiler_settings.file_name,
                format!("{}/", starter_name),
                |path, module_identifier| {
                    if module_identifier.starts_with("@") {
                        panic!("Link module not ready");
                    } else {
                        match ellie_core::module_path::parse_module_import(
                            &path,
                            &module_identifier,
                        ) {
                            Ok(path) => {
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
                                if Path::new(&real_path).exists() {
                                    match cli_utils::read_file(real_path) {
                                        Ok(data) => {
                                            let mut hasher = DefaultHasher::new();
                                            data.hash(&mut hasher);
                                            ResolvedImport {
                                                found: true,
                                                matched:
                                                    ellie_tokenizer::tokenizer::ImportType::Code(
                                                        data,
                                                    ),
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
                                        resolve_error: "Path is not exists".to_string(),
                                        ..Default::default()
                                    }
                                }
                            }
                            Err(e) => {
                                if e == 1 {
                                    ResolvedImport {
                                        found: false,
                                        resolve_error: "Cannot access outside of workspace"
                                            .to_string(),
                                        ..Default::default()
                                    }
                                } else {
                                    unreachable!()
                                }
                            }
                        }
                    }
                },
                first_page_hash.clone().try_into().unwrap(),
            );

            let tokenize_start = Instant::now();
            match pager.run() {
                Ok(_) => {
                    let mut parser = parser::Parser::new(
                        pager.pages.clone(),
                        first_page_hash.try_into().unwrap(),
                        compiler_settings.version,
                        compiler_settings.name,
                        compiler_settings.description,
                        compiler_settings.is_lib,
                        ellie_core::defs::Version::build_from_string(
                            crate::engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                        ),
                    );

                    if compiler_settings.exclude_stdlib {
                        println!(
                            "\n{}[!]{}: {}'exclude_stdlib'{} option is deprecated",
                            cli_utils::Colors::Yellow,
                            cli_utils::Colors::Reset,
                            cli_utils::Colors::Cyan,
                            cli_utils::Colors::Reset,
                        );
                    }

                    for (module, _) in modules.iter() {
                        if used_modules.lock().unwrap().contains(&(&module.name)) {
                            parser.import_module(module.clone());
                        }
                    }

                    let tokenize_end =
                        (tokenize_start.elapsed().as_nanos() as f64 / 1000000_f64) as f64;
                    let compile_start = Instant::now();
                    let workspace = parser.parse();
                    let compile_end =
                        (compile_start.elapsed().as_nanos() as f64 / 1000000_f64) as f64;
                    let mut bytecode_end: f64 = 0.0;

                    if !parser.informations.has_no_warnings() && compiler_settings.warnings {
                        if compiler_settings.json_log {
                            let mut output = cli_outputs::COMPILER_WARNINGS.clone();
                            output.extra.push(cli_outputs::CliOuputExtraData {
                                key: "warnings".to_string(),
                                value: parser.informations.warnings.clone(),
                            });
                            println!("{}", serde_json::to_string(&output).unwrap());
                        } else {
                            cli_utils::print_warnings(
                                &parser.informations.warnings,
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
                                        match cli_utils::read_file(real_path) {
                                            Ok(e) => e,
                                            Err(err) => {
                                                panic!(
                                                "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                                path,
                                                cli_utils::Colors::Red,
                                                err,
                                                cli_utils::Colors::Reset
                                            );
                                            }
                                        }
                                    } else if let Some((_, module_path)) = modules
                                        .iter()
                                        .find(|(module, _)| module.name == virtual_path_identifier)
                                    {
                                        let module_path = module_path.clone().unwrap();
                                        let real_path =
                                            path.replace(&path_starter, &module_path).clone();
                                        match cli_utils::read_file(real_path) {
                                            Ok(e) => e,
                                            Err(err) => {
                                                panic!(
                                                "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                                path,
                                                cli_utils::Colors::Red,
                                                err,
                                                cli_utils::Colors::Reset
                                            );
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
                                    } else if let Some((_, module_path)) = modules
                                        .iter()
                                        .find(|(module, _)| module.name == virtual_path_identifier)
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
                            );
                        }
                    }

                    if !parser.informations.has_no_errors() {
                        if compiler_settings.json_log {
                            let mut output = cli_outputs::COMPILER_ERRORS.clone();
                            output.extra.push(cli_outputs::CliOuputExtraData {
                                key: "errors".to_string(),
                                value: parser.informations.errors.clone(),
                            });
                            println!("{}", serde_json::to_string(&output).unwrap());
                        } else {
                            cli_utils::print_errors(
                                &parser.informations.errors,
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
                                        match cli_utils::read_file(real_path) {
                                            Ok(e) => e,
                                            Err(err) => {
                                                panic!(
                                                "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                                path,
                                                cli_utils::Colors::Red,
                                                err,
                                                cli_utils::Colors::Reset
                                            );
                                            }
                                        }
                                    } else if let Some((_, module_path)) = modules
                                        .iter()
                                        .find(|(module, _)| module.name == virtual_path_identifier)
                                    {
                                        let module_path = module_path.clone().unwrap();
                                        let real_path =
                                            path.replace(&path_starter, &module_path).clone();
                                        match cli_utils::read_file(real_path) {
                                            Ok(e) => e,
                                            Err(err) => {
                                                panic!(
                                                "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                                path,
                                                cli_utils::Colors::Red,
                                                err,
                                                cli_utils::Colors::Reset
                                            );
                                            }
                                        }
                                    } else {
                                        panic!(
                                            "Failed to ouput error. Cannot identify module '{}'",
                                            path,
                                        );
                                    }
                                },
                                compiler_settings.show_debug_lines,
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
                                        let module_path = module_path.clone().unwrap();
                                        path.replace(&path_starter, &module_path).clone()
                                    } else {
                                        panic!(
                                            "Failed to ouput error. Cannot identify module '{}'",
                                            path,
                                        );
                                    }
                                },
                            );
                        }

                        if parser.informations.warnings.len() == 0 {
                            if compiler_settings.json_log {
                                let mut output =
                                    cli_outputs::COMPILE_FAILED_WITH_ERRORS_WITH_NO_WARNINGS
                                        .clone();
                                output.extra.push(cli_outputs::CliOuputExtraData {
                                    key: "errors".to_string(),
                                    value: parser.informations.errors.len().clone().to_string(),
                                });
                                println!("{}", serde_json::to_string(&output).unwrap())
                            } else {
                                println!(
                                    "\n{}[!]{}: Compiling {}failed{} with {}{} errors{}",
                                    cli_utils::Colors::Red,
                                    cli_utils::Colors::Reset,
                                    cli_utils::Colors::Red,
                                    cli_utils::Colors::Reset,
                                    cli_utils::Colors::Red,
                                    parser.informations.errors.len(),
                                    cli_utils::Colors::Reset,
                                );
                            }
                        } else {
                            if compiler_settings.json_log {
                                let mut output =
                                    cli_outputs::COMPILE_FAILED_WITH_ERRORS_WITH_WARNINGS.clone();
                                output.extra.push(cli_outputs::CliOuputExtraData {
                                    key: "errors".to_string(),
                                    value: parser.informations.errors.len().clone().to_string(),
                                });
                                output.extra.push(cli_outputs::CliOuputExtraData {
                                    key: "warnings".to_string(),
                                    value: parser.informations.warnings.len().to_string(),
                                });
                                println!("{}", serde_json::to_string(&output).unwrap())
                            } else {
                                println!("\n{}[!]{}: Compiling {}failed{} with {}{} errors{} and {}{} warnings{}.",
                                    cli_utils::Colors::Red,
                                    cli_utils::Colors::Reset,
                                    cli_utils::Colors::Red,
                                    cli_utils::Colors::Reset,
                                    cli_utils::Colors::Red,
                                    parser.informations.errors.len(),
                                    cli_utils::Colors::Reset,
                                    cli_utils::Colors::Yellow,
                                    parser.informations.warnings.len(),
                                    cli_utils::Colors::Reset,
                                );
                            }
                        }
                        std::process::exit(1)
                    } else {
                        if parser.informations.warnings.len() == 0 {
                            if compiler_settings.json_log {
                                println!(
                                    "{}",
                                    serde_json::to_string(
                                        &cli_outputs::COMPILE_SUCCESS_WITH_NO_WARNINGS.clone()
                                    )
                                    .unwrap()
                                )
                            } else {
                                println!(
                                    "\n{}[!]{}: Compiling {}succeeded{}.",
                                    cli_utils::Colors::Green,
                                    cli_utils::Colors::Reset,
                                    cli_utils::Colors::Green,
                                    cli_utils::Colors::Reset,
                                );
                            }
                        } else {
                            if compiler_settings.json_log {
                                let mut output = cli_outputs::COMPILE_SUCCESS_WITH_WARNINGS.clone();
                                output.extra.push(cli_outputs::CliOuputExtraData {
                                    key: "warnings".to_string(),
                                    value: parser.informations.warnings.len().to_string(),
                                });
                                println!("{}", serde_json::to_string(&output).unwrap())
                            } else {
                                println!(
                                    "\n{}[!]{}: Compiling {}succeeded{} with {}{} warnings{}.",
                                    cli_utils::Colors::Yellow,
                                    cli_utils::Colors::Reset,
                                    cli_utils::Colors::Green,
                                    cli_utils::Colors::Reset,
                                    cli_utils::Colors::Yellow,
                                    parser.informations.warnings.len(),
                                    cli_utils::Colors::Reset,
                                );
                            }
                        }

                        let output_path = &get_output_path(
                            target_path,
                            output_path,
                            compiler_settings.output_type.clone(),
                        );
                        match compiler_settings.output_type {
                            cli_utils::OutputTypes::Bin => {
                                let bytes = bincode::serialize(&workspace).unwrap();
                                if let Err(write_error) = fs::write(output_path, bytes) {
                                    if compiler_settings.json_log {
                                        let mut output = cli_outputs::WRITE_FILE_ERROR.clone();

                                        output.extra.push(cli_outputs::CliOuputExtraData {
                                            key: "path".to_string(),
                                            value: format!("{:?}", write_error),
                                        });
                                        println!("{}", serde_json::to_string(&output).unwrap())
                                    } else {
                                        println!(
                                            "\nFailed to write output. [{}{:?}{}]",
                                            cli_utils::Colors::Red,
                                            write_error,
                                            cli_utils::Colors::Reset,
                                        );
                                    }
                                } else {
                                    if compiler_settings.json_log {
                                        let mut output = cli_outputs::WRITE_BINARY_SUCCEDED.clone();
                                        output.extra.push(cli_outputs::CliOuputExtraData {
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
                                            cli_utils::Colors::Green,
                                            cli_utils::Colors::Reset,
                                            cli_utils::Colors::Yellow,
                                            output_path.absolutize().unwrap().to_str().unwrap(),
                                            cli_utils::Colors::Reset
                                        );
                                    }
                                }
                            }
                            cli_utils::OutputTypes::DependencyAnalysis => {
                                println!(
                                    "{}[!]{}: Dependency analysis output is not yet implemented.",
                                    cli_utils::Colors::Red,
                                    cli_utils::Colors::Reset,
                                );
                            }
                            cli_utils::OutputTypes::ByteCodeAsm => {
                                if !compiler_settings.json_log {
                                    println!(
                                        "{}[?]{}: ByteCode compiling to {} bit architecture",
                                        cli_utils::Colors::Green,
                                        cli_utils::Colors::Reset,
                                        match compiler_settings.byte_code_architecture {
                                            ellie_core::defs::PlatformArchitecture::B16 => "16",
                                            ellie_core::defs::PlatformArchitecture::B32 => "32",
                                            ellie_core::defs::PlatformArchitecture::B64 => "64",
                                        }
                                    );
                                }
                                let bytecode_start = Instant::now();
                                let mut assembler = ellie_bytecode::assembler::Assembler::new(
                                    workspace,
                                    ellie_bytecode::assembler::PlatformAttributes {
                                        architecture: ellie_core::defs::PlatformArchitecture::B64, //64 Bit Limit
                                        memory_size: 512000, //512kb memory limit
                                    },
                                );
                                let assembler_result = assembler.assemble();
                                bytecode_end = (bytecode_start.elapsed().as_nanos() as f64
                                    / 1000000_f64)
                                    as f64;
                                match File::create(output_path) {
                                    Ok(file) => {
                                        assembler_result.alternate_render(file);
                                        if compiler_settings.json_log {
                                            let mut output =
                                                cli_outputs::WRITE_BYTE_CODE_SUCCEDED.clone();
                                            output.extra.push(cli_outputs::CliOuputExtraData {
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
                                                cli_utils::Colors::Green,
                                                cli_utils::Colors::Reset,
                                                cli_utils::Colors::Yellow,
                                                output_path.absolutize().unwrap().to_str().unwrap(),
                                                cli_utils::Colors::Reset,
                                            );
                                        }
                                    }
                                    Err(write_error) => {
                                        if compiler_settings.json_log {
                                            let mut output = cli_outputs::WRITE_FILE_ERROR.clone();
                                            output.extra.push(cli_outputs::CliOuputExtraData {
                                                key: "path".to_string(),
                                                value: format!("{:?}", write_error),
                                            });

                                            println!("{}", serde_json::to_string(&output).unwrap())
                                        } else {
                                            println!(
                                                "\nFailed to write output. [{}{:?}{}]",
                                                cli_utils::Colors::Red,
                                                write_error,
                                                cli_utils::Colors::Reset,
                                            );
                                        }
                                    }
                                }
                            }
                            cli_utils::OutputTypes::Json => {
                                let json = serde_json::to_string(&workspace).unwrap();
                                if let Err(write_error) = fs::write(&output_path, json) {
                                    if compiler_settings.json_log {
                                        let mut output = cli_outputs::WRITE_FILE_ERROR.clone();
                                        output.extra.push(cli_outputs::CliOuputExtraData {
                                            key: "path".to_string(),
                                            value: format!("{:?}", write_error),
                                        });

                                        println!("{}", serde_json::to_string(&output).unwrap())
                                    } else {
                                        println!(
                                            "\nFailed to write output. [{}{:?}{}]",
                                            cli_utils::Colors::Red,
                                            write_error,
                                            cli_utils::Colors::Reset,
                                        );
                                    }
                                } else {
                                    if compiler_settings.json_log {
                                        let mut output = cli_outputs::WRITE_JSON_SUCCEDED.clone();
                                        output.extra.push(cli_outputs::CliOuputExtraData {
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
                                            cli_utils::Colors::Green,
                                            cli_utils::Colors::Reset,
                                            cli_utils::Colors::Yellow,
                                            output_path.absolutize().unwrap().to_str().unwrap(),
                                            cli_utils::Colors::Reset,
                                        );
                                    }
                                }
                            }
                            cli_utils::OutputTypes::Nop => (),
                            cli_utils::OutputTypes::ByteCode => {
                                if !compiler_settings.json_log {
                                    println!(
                                        "{}[?]{}: ByteCode compiling to {} bit architecture",
                                        cli_utils::Colors::Green,
                                        cli_utils::Colors::Reset,
                                        match compiler_settings.byte_code_architecture {
                                            ellie_core::defs::PlatformArchitecture::B16 => "16",
                                            ellie_core::defs::PlatformArchitecture::B32 => "32",
                                            ellie_core::defs::PlatformArchitecture::B64 => "64",
                                        }
                                    );
                                }

                                let bytecode_start = Instant::now();
                                let mut assembler = ellie_bytecode::assembler::Assembler::new(
                                    workspace,
                                    ellie_bytecode::assembler::PlatformAttributes {
                                        architecture: ellie_core::defs::PlatformArchitecture::B64, //64 Bit Limit
                                        memory_size: 512000, //512kb memory limit
                                    },
                                );
                                let assembler_result = assembler.assemble();
                                bytecode_end = (bytecode_start.elapsed().as_nanos() as f64
                                    / 1000000_f64)
                                    as f64;
                                match File::create(output_path) {
                                    Ok(mut file) => {
                                        assembler_result.render_binary(&mut file, None);
                                        if compiler_settings.json_log {
                                            let mut output =
                                                cli_outputs::WRITE_BYTE_CODE_SUCCEDED.clone();
                                            output.extra.push(cli_outputs::CliOuputExtraData {
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
                                                cli_utils::Colors::Green,
                                                cli_utils::Colors::Reset,
                                                cli_utils::Colors::Yellow,
                                                output_path.absolutize().unwrap().to_str().unwrap(),
                                                cli_utils::Colors::Reset,
                                            );
                                        }
                                    }
                                    Err(write_error) => {
                                        if compiler_settings.json_log {
                                            let mut output = cli_outputs::WRITE_FILE_ERROR.clone();
                                            output.extra.push(cli_outputs::CliOuputExtraData {
                                                key: "path".to_string(),
                                                value: format!("{:?}", write_error),
                                            });

                                            println!("{}", serde_json::to_string(&output).unwrap())
                                        } else {
                                            println!(
                                                "\nFailed to write output. [{}{:?}{}]",
                                                cli_utils::Colors::Red,
                                                write_error,
                                                cli_utils::Colors::Reset,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if !compiler_settings.json_log {
                        println!(
                            "{}[?]{}: Ellie v{}",
                            cli_utils::Colors::Green,
                            cli_utils::Colors::Reset,
                            crate::engine_constants::ELLIE_ENGINE_VERSION
                        );

                        if (compiler_settings.output_type == cli_utils::OutputTypes::ByteCode
                            || compiler_settings.output_type == cli_utils::OutputTypes::ByteCodeAsm)
                            && compiler_settings.performance_info
                        {
                            println!(
                                "{}[?]{}: Tokenizing took {}{}{}ms, Parsing took {}{}{}ms, ByteCode compile took {}{}{}ms. Total time: {}{}{}ms",
                                cli_utils::Colors::Yellow,
                                cli_utils::Colors::Reset,
                                cli_utils::Colors::Yellow,
                                tokenize_end,
                                cli_utils::Colors::Reset,
                                cli_utils::Colors::Yellow,
                                compile_end,
                                cli_utils::Colors::Reset,
                                cli_utils::Colors::Yellow,
                                bytecode_end,
                                cli_utils::Colors::Reset,
                                cli_utils::Colors::Yellow,
                                compile_end + tokenize_end + bytecode_end,
                                cli_utils::Colors::Reset,
                            );
                        } else if compiler_settings.performance_info {
                            println!(
                                "{}[?]{}: Tokenizing took {}{}{}ms, Parsing took {}{}{}ms. Total time: {}{}{}ms",
                                cli_utils::Colors::Yellow,
                                cli_utils::Colors::Reset,
                                cli_utils::Colors::Yellow,
                                tokenize_end,
                                cli_utils::Colors::Reset,
                                cli_utils::Colors::Yellow,
                                compile_end,
                                cli_utils::Colors::Reset,
                                cli_utils::Colors::Yellow,
                                compile_end + tokenize_end,
                                cli_utils::Colors::Reset,
                            );
                        }

                        println!(
                            "{}[!]{}: Ellie is on development and may not be stable.",
                            cli_utils::Colors::Red,
                            cli_utils::Colors::Reset,
                        );
                    }
                }
                Err(pager_errors) => {
                    if compiler_settings.json_log {
                        let mut output = cli_outputs::COMPILER_ERRORS.clone();
                        output.extra.push(cli_outputs::CliOuputExtraData {
                            key: "errors".to_string(),
                            value: pager_errors,
                        });
                        println!("{}", serde_json::to_string(&output).unwrap());
                    } else {
                        cli_utils::print_errors(
                            &pager_errors,
                            |path| match cli_utils::read_file(
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
                                    panic!(
                                        "{}[Internal Error]{} Cannot build error, read file failed '{}' {}[{}]{}",
                                        cli_utils::Colors::Red,
                                        cli_utils::Colors::Reset,
                                        path,
                                        cli_utils::Colors::Red,
                                        err,
                                        cli_utils::Colors::Reset
                                    );
                                }
                            },
                            compiler_settings.show_debug_lines,
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
                        );
                        println!(
                            "\n{}[!]{}: Compiling {}failed{} with {}{} errors{}",
                            cli_utils::Colors::Red,
                            cli_utils::Colors::Reset,
                            cli_utils::Colors::Red,
                            cli_utils::Colors::Reset,
                            cli_utils::Colors::Red,
                            pager_errors.len(),
                            cli_utils::Colors::Reset,
                        );
                    }
                }
            }
        }
        Err(err) => {
            if compiler_settings.json_log {
                let mut cli_module_output = cli_outputs::READ_FILE_ERROR.clone();
                cli_module_output
                    .extra
                    .push(cli_outputs::CliOuputExtraData { key: 0, value: err });
                cli_module_output
                    .extra
                    .push(cli_outputs::CliOuputExtraData {
                        key: 1,
                        value: target_path.to_str().unwrap().to_string(),
                    });
                println!(
                    "{}",
                    serde_json::to_string_pretty(&cli_module_output).unwrap()
                );
            } else {
                println!(
                    "Unable to read file ~{} [{}]",
                    target_path.to_str().unwrap().to_string(),
                    err
                );
                std::process::exit(1);
            }
        }
    }
}
