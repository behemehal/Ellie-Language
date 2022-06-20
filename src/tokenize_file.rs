use crate::cli_outputs;
use crate::cli_utils;
use ellie_tokenizer::tokenizer::{self, ResolvedImport};
use path_absolutize::Absolutize;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

pub struct TokenizerSettings {
    pub json_log: bool,
    pub name: String,
    pub file_name: String,
    pub show_debug_lines: bool,
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

pub fn tokenize(target_path: &Path, output_path: &Path, tokenizer_settings: TokenizerSettings) {
    let starter_name = format!("<ellie_module_{}>", tokenizer_settings.name);
    match cli_utils::read_file(target_path) {
        Ok(main_file_content) => {
            let mut main_file_hasher = DefaultHasher::new();
            main_file_content.hash(&mut main_file_hasher);
            let first_page_hash = main_file_hasher.finish();
            let mut pager = tokenizer::Pager::new(
                main_file_content,
                tokenizer_settings.file_name,
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
                    let tokenize_end =
                        (tokenize_start.elapsed().as_nanos() as f64 / 1000000_f64) as f64;
                    let json = serde_json::to_string(&pager.pages).unwrap();

                    let output_path =
                        &get_output_path(target_path, output_path, cli_utils::OutputTypes::Json);

                    if let Err(write_error) = fs::write(&output_path, json) {
                        if tokenizer_settings.json_log {
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
                        if tokenizer_settings.json_log {
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

                    if !tokenizer_settings.json_log {
                        println!(
                            "{}[?]{}: Ellie v{}",
                            cli_utils::Colors::Green,
                            cli_utils::Colors::Reset,
                            crate::engine_constants::ELLIE_ENGINE_VERSION
                        );
                        println!(
                            "{}[?]{}: Tokenizing took {}{}{}ms",
                            cli_utils::Colors::Yellow,
                            cli_utils::Colors::Reset,
                            cli_utils::Colors::Yellow,
                            tokenize_end,
                            cli_utils::Colors::Reset,
                        );
                        println!(
                            "{}[!]{}: Ellie is on development and may not be stable.",
                            cli_utils::Colors::Red,
                            cli_utils::Colors::Reset,
                        );
                    }
                }
                Err(pager_errors) => {
                    if tokenizer_settings.json_log {
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
                            tokenizer_settings.show_debug_lines,
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
                    }
                }
            }
        }
        Err(err) => {
            if tokenizer_settings.json_log {
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
