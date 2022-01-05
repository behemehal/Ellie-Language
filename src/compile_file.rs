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
    pub description: String,
    pub version: Version,
    pub output_type: String,
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
    modules: Vec<parser::Module>,
    compiler_settings: CompilerSettings,
) {
    let prefered_output_type = match compiler_settings.output_type.as_str() {
        "bin" => cli_utils::OutputTypes::Bin,
        "json" => cli_utils::OutputTypes::Json,
        "depA" => cli_utils::OutputTypes::DependencyAnalysis,
        _ => unreachable!(),
    };

    match cli_utils::read_file(target_path) {
        Ok(main_file_content) => {
            //Auto import 'ellieStd'
            let used_modules = Mutex::new(vec!["ellieStd".to_string()]);
            let mut pager = tokenizer::Pager::new(
                main_file_content,
                target_path.to_str().unwrap().to_string(),
                |path, file_name| {
                    let path = Path::new(&path)
                        .parent()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    let file = if cli_utils::file_exists(path.clone() + "/" + &file_name.clone()) {
                        Some(path.clone() + "/" + &file_name.clone())
                    } else if cli_utils::file_exists(
                        path.clone() + "/" + &file_name.clone() + ".ei",
                    ) {
                        Some(path.clone() + "/" + &file_name.clone() + ".ei")
                    } else {
                        None
                    };

                    if !compiler_settings.json_log && compiler_settings.warnings {
                        println!(
                            "{}[Warning]{}: A import used but resolver is not 100% ready. Be aware of {}errors{}",
                            cli_utils::Colors::Yellow,
                            cli_utils::Colors::Reset,
                            cli_utils::Colors::Red,
                            cli_utils::Colors::Reset
                        );
                    }

                    if let Some(module) = modules.iter().find(|x| x.name == file_name) {
                        if module.name != "ellieStd" {
                            used_modules.lock().unwrap().push(module.name.clone());
                        }
                        ResolvedImport {
                            found: true,
                            matched: tokenizer::ImportType::Module(
                                ellie_tokenizer::tokenizer::Module {
                                    hash: module.hash,
                                    initial_page: module.initial_page,
                                    version: module.version.clone(),
                                    name: module.name.clone(),
                                },
                            ),
                            hash: module.hash,
                            path: "<lia_virtual>".to_string(),
                            ..Default::default()
                        }
                    } else {
                        match file {
                            Some(file) => {
                                let file = Path::new(&file).absolutize().unwrap();
                                match cli_utils::read_file(&file.to_str().unwrap().to_string()) {
                                    Ok(ext) => {
                                        let mut hasher = DefaultHasher::new();
                                        ext.hash(&mut hasher);
                                        ResolvedImport {
                                            found: true,
                                            matched: ellie_tokenizer::tokenizer::ImportType::Code(
                                                ext,
                                            ),
                                            hash: hasher.finish(),
                                            path: file.to_str().unwrap().to_string(),
                                            ..Default::default()
                                        }
                                    }
                                    Err(err) => ResolvedImport {
                                        found: false,
                                        resolve_error: err,
                                        ..Default::default()
                                    },
                                }
                            }
                            None => ResolvedImport {
                                found: false,
                                ..Default::default()
                            },
                        }
                    }
                },
                None,
            );

            match pager.run() {
                Ok(_) => {
                    let mut parser =
                        parser::Parser::new(pager.pages.clone(), None, compiler_settings.version);
                    for i in modules.iter() {
                        if used_modules.lock().unwrap().contains(&(&i.name)) {
                            parser.import_module(i.clone());
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
                                        println!(
                                            "Cannot read file '{}' {}[{}]{}",
                                            path,
                                            cli_utils::Colors::Red,
                                            err,
                                            cli_utils::Colors::Reset
                                        );
                                        std::process::exit(1);
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
                                match cli_utils::read_file(&path) {
                                    Ok(e) => e,
                                    Err(err) => {
                                        println!(
                                            "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                            path,
                                            cli_utils::Colors::Red,
                                            err,
                                            cli_utils::Colors::Reset
                                        );
                                        std::process::exit(1);
                                    }
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
                            prefered_output_type.clone(),
                        );
                        match prefered_output_type {
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
                            match cli_utils::read_file(&path) {
                                Ok(e) => e,
                                Err(err) => {
                                    println!(
                                        "Cannot read file '{}' {}[{}]{}",
                                        path,
                                        cli_utils::Colors::Red,
                                        err,
                                        cli_utils::Colors::Reset
                                    );
                                    std::process::exit(1);
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
