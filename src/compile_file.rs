use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use ellie_core::defs::Version;

use crate::cli_outputs;
use crate::cli_utils;
use ellie_parser::parser;
use ellie_tokenizer::tokenizer::{self, ResolvedImport};
use path_absolutize::Absolutize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct CompilerSettings {
    pub json_log: bool,
    pub name: String,
    pub file_name: String,
    pub description: String,
    pub version: Version,
    pub output_type: cli_utils::OutputTypes,
    pub warnings: bool,
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
                    cli_utils::OutputTypes::Bin => ".bin",
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
    modules: Vec<(parser::Module, String)>,
    compiler_settings: CompilerSettings,
) {
    let starter_name = format!("<ellie_module_{}>", compiler_settings.name);
    match cli_utils::read_file(target_path) {
        Ok(main_file_content) => {
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
                                                hash: hasher.finish(),
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
                None,
            );

            match pager.run() {
                Ok(_) => {
                    let mut parser =
                        parser::Parser::new(pager.pages.clone(), None, compiler_settings.version);

                    for (module, path) in modules.iter() {
                        if module.name == "ellie" {
                            parser.import_module(module.clone());
                        } else if used_modules.lock().unwrap().contains(&(&module.name)) {
                            parser.import_module(module.clone());
                        }
                    }

                    let workspace = parser.parse(
                        compiler_settings.name,
                        compiler_settings.description,
                        ellie_core::defs::Version::build_from_string(
                            crate::engine_constants::ELLIE_VERSION.to_owned(),
                        ),
                    );

                    if !parser.informations.has_no_warnings() && compiler_settings.warnings {
                        if compiler_settings.json_log {
                            let mut output = cli_outputs::COMPILER_WARNINGS.clone();
                            output.extra.push(cli_outputs::CliOuputExtraData {
                                key: "warnings".to_string(),
                                value: parser.informations.warnings.clone(),
                            });
                            println!("{}", serde_json::to_string(&output).unwrap());
                        } else {
                            cli_utils::print_warnings(&parser.informations.warnings, |path| {
                                match cli_utils::read_file(&path) {
                                    Ok(e) => e,
                                    Err(err) => {
                                        panic!(
                                            "{}[Internal Error]{} Cannot build warning, read file failed '{}' {}[{}]{}",
                                            cli_utils::Colors::Red,
                                            cli_utils::Colors::Reset,
                                            path,
                                            cli_utils::Colors::Red,
                                            err,
                                            cli_utils::Colors::Reset
                                        );
                                    }
                                }
                            });
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
                            cli_utils::print_errors(&parser.informations.errors, |path| {
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
                                } else if let Some((module, module_path)) = modules
                                    .iter()
                                    .find(|(module, path)| module.name == virtual_path_identifier)
                                {
                                    let real_path =
                                        path.replace(&path_starter, module_path).clone();
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
                            });
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
                                    "\nCompiling {}failed{} with {}{} errors{}",
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
                                println!("\nCompiling {}failed{} with {}{} errors{} and {}{} warnings{}.",
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
                                    "\nCompiling {}succeeded{}.",
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
                                    "\nCompiling {}succeeded{} with {}{} warnings{}.",
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
                                            "\nBinary output written to {}{}{}",
                                            cli_utils::Colors::Yellow,
                                            output_path.absolutize().unwrap().to_str().unwrap(),
                                            cli_utils::Colors::Reset
                                        );
                                    }
                                }
                            }
                            cli_utils::OutputTypes::DependencyAnalysis => todo!(),
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
                                            "\nJSON output written to {}{}{}",
                                            cli_utils::Colors::Yellow,
                                            output_path.absolutize().unwrap().to_str().unwrap(),
                                            cli_utils::Colors::Reset,
                                        );
                                    }
                                }
                            }
                        }
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
                        cli_utils::print_errors(&pager_errors, |path| {
                            match cli_utils::read_file(
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
                            }
                        });
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
