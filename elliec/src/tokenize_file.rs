use crate::{get_output_path, OutputTypesSelector};
use ellie_engine::{
    ellie_core,
    ellie_renderer_utils::{
        outputs,
        utils::{self, print_errors, CliColor, ColorDisplay, Colors, TextStyles},
    },
    ellie_tokenizer::tokenizer::{ImportType, ResolvedImport},
    tokenizer::tokenize_file,
    utils::{MainProgram, ProgramRepository},
};
use path_absolutize::Absolutize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::Instant;
use std::{fs, println};

#[derive(Clone)]
pub struct TokenizerSettings {
    pub json_log: bool,
    pub name: String,
    pub file_name: String,
    pub show_debug_lines: bool,
}

pub fn tokenize(target_path: &Path, output_path: &Path, tokenizer_settings: TokenizerSettings) {
    let cli_color = CliColor;

    #[derive(Clone)]
    struct Repository {
        target_path: String,
        tokenizer_settings: TokenizerSettings,
    }

    #[derive(Copy, Clone)]
    struct ColorTerminal;

    let color_terminal = ColorTerminal;
    let mut program_repository = Repository {
        target_path: target_path.to_str().unwrap().to_string(),
        tokenizer_settings: tokenizer_settings.clone(),
    };

    impl ProgramRepository for Repository {
        fn read_main(&mut self) -> MainProgram {
            match utils::read_file(self.target_path.clone()) {
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
                            match utils::read_file(real_path) {
                                Ok(data) => {
                                    let mut hasher = DefaultHasher::new();
                                    data.hash(&mut hasher);
                                    ResolvedImport {
                                        found: true,
                                        matched: ImportType::Code(data),
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

    let tokenize_start = Instant::now();

    let starter_name = format!("<ellie_module_{}>", tokenizer_settings.name);

    match tokenize_file(&mut program_repository) {
        Ok(pages) => {
            let tokenize_end = (tokenize_start.elapsed().as_nanos() as f64 / 1000000_f64) as f64;
            let json = serde_json::to_string(&pages).unwrap();
            let output_path = &get_output_path(target_path, output_path, OutputTypesSelector::Json);

            if let Err(write_error) = fs::write(&output_path, json) {
                if tokenizer_settings.json_log {
                    let mut output = outputs::WRITE_FILE_ERROR.clone();
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "path".to_string(),
                        value: format!("{:?}", write_error),
                    });

                    println!("{}", serde_json::to_string(&output).unwrap())
                } else {
                    println!(
                        "\nFailed to write output. [{}{:?}{}]",
                        cli_color.color(Colors::Red),
                        write_error,
                        cli_color.color(Colors::Reset),
                    );
                }
            } else {
                if tokenizer_settings.json_log {
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
                        cli_color.color(Colors::Green),
                        cli_color.color(Colors::Reset),
                        cli_color.color(Colors::Yellow),
                        output_path.absolutize().unwrap().to_str().unwrap(),
                        cli_color.color(Colors::Reset),
                    );
                }
            }

            if !tokenizer_settings.json_log {
                println!(
                    "{}[?]{}: Ellie v{}",
                    cli_color.color(Colors::Green),
                    cli_color.color(Colors::Reset),
                    crate::engine_constants::ELLIE_ENGINE_VERSION
                );
                println!(
                    "{}[?]{}: Tokenizing took {}{}{}ms",
                    cli_color.color(Colors::Yellow),
                    cli_color.color(Colors::Reset),
                    cli_color.color(Colors::Yellow),
                    tokenize_end,
                    cli_color.color(Colors::Reset),
                );
                println!(
                    "{}[!]{}: Ellie is on development and may not be stable.",
                    cli_color.color(Colors::Red),
                    cli_color.color(Colors::Reset),
                );
            }
        }
        Err(pager_errors) => {
            if tokenizer_settings.json_log {
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
                                panic!(
                                    "{}[Internal Error]{} Cannot build error, read file failed '{}' {}[{}]{}",
                                    cli_color.color(Colors::Red),
                                    cli_color.color(Colors::Reset),
                                    path,
                                    cli_color.color(Colors::Red),
                                    err,
                                    cli_color.color(Colors::Reset),
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
