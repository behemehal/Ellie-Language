use crate::cli_outputs;
use crate::cli_utils;
use crate::terminal_utils;
use crate::terminal_utils::ColorDisplay;
use crate::terminal_utils::MainProgram;
use crate::terminal_utils::ProgramRepository;
use ellie_tokenizer::tokenizer::ResolvedImport;
use path_absolutize::Absolutize;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Clone)]
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
    #[derive(Clone)]
    struct Repository {
        target_path: String,
        tokenizer_settings: TokenizerSettings,
    }

    #[derive(Copy, Clone)]
    struct ColorTerminal;

    let color_terminal = ColorTerminal;
    let mut program_repisotory = Repository {
        target_path: target_path.to_str().unwrap().to_string(),
        tokenizer_settings: tokenizer_settings.clone(),
    };

    impl ProgramRepository for Repository {
        fn read_main(&mut self) -> terminal_utils::MainProgram {
            match cli_utils::read_file(self.target_path.clone()) {
                Ok(main_file_content) => {
                    let mut main_file_hasher = DefaultHasher::new();
                    main_file_content.hash(&mut main_file_hasher);
                    let first_page_hash = main_file_hasher.finish();
                    MainProgram {
                        file_content: main_file_content,
                        file_name: self.tokenizer_settings.file_name.clone(),
                        file_hash: first_page_hash as usize,
                        start_directory: format!("<ellie_module_{}>", self.tokenizer_settings.name),
                    }
                }
                Err(err) => {
                    if self.tokenizer_settings.json_log {
                        let mut cli_module_output = cli_outputs::READ_FILE_ERROR.clone();
                        cli_module_output
                            .extra
                            .push(cli_outputs::CliOuputExtraData { key: 0, value: err });
                        cli_module_output
                            .extra
                            .push(cli_outputs::CliOuputExtraData {
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
            let starter_name = format!("<ellie_module_{}>", self.tokenizer_settings.name);

            if link_module {
                ResolvedImport {
                    found: false,
                    resolve_error: "Cannot use modules in tokenizer".to_owned(),
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
                            match cli_utils::read_file(real_path) {
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

    impl ColorDisplay for ColorTerminal {
        fn color(&self, color: terminal_utils::Colors) -> String {
            let color_id = match color {
                terminal_utils::Colors::Black => "[30m",
                terminal_utils::Colors::Red => "[31m",
                terminal_utils::Colors::Green => "[32m",
                terminal_utils::Colors::Yellow => "[33m",
                terminal_utils::Colors::Blue => "[34m",
                terminal_utils::Colors::Magenta => "[35m",
                terminal_utils::Colors::Cyan => "[36m",
                terminal_utils::Colors::White => "[37m",
                terminal_utils::Colors::Reset => "[0m",
            };
            format!("{}{}", '\u{001b}', color_id)
        }

        fn text_style(&self, text_style: terminal_utils::TextStyles) -> String {
            let type_id = match text_style {
                terminal_utils::TextStyles::Bold => "[1m",
                terminal_utils::TextStyles::Dim => "[2m",
                terminal_utils::TextStyles::Italic => "[3m",
                terminal_utils::TextStyles::Underline => "[4m",
            };
            format!("{}{}", '\u{001b}', type_id)
        }
    }

    let tokenize_start = Instant::now();

    let starter_name = format!("<ellie_module_{}>", tokenizer_settings.name);

    match crate::tokenizer::tokenize_file(&mut program_repisotory) {
        Ok(pages) => {
            let tokenize_end = (tokenize_start.elapsed().as_nanos() as f64 / 1000000_f64) as f64;
            let json = serde_json::to_string(&pages).unwrap();
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
                println!(
                    "{}",
                    terminal_utils::print_errors(
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
                                    cli_utils::Colors::Reset,
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
                        color_terminal
                    )
                );
            }
        }
    }
}
