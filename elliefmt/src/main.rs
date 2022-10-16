use ellie_engine::{
    ellie_renderer_utils::{
        options, outputs,
        utils::{CliColor, ColorDisplay, Colors, TextStyles},
    },
    engine_constants,
};
use std::path::Path;
mod format_file;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let app = options::generate_elliefmt_options();
    let matches = app.get_matches();
    let cli_color = &CliColor;

    match matches.subcommand() {
        Some(("format", matches)) => {
            if !matches.is_present("allowPanics") {
                std::panic::set_hook(Box::new(|e| {
                    if e.to_string().contains("@Halt") {
                        println!(
                            "\n\n{}-----------------{}\n",
                            cli_color.color(Colors::Blue),
                            cli_color.color(Colors::Reset)
                        );
                        println!(
                            "{}{}Compiler halted{}\n",
                            cli_color.color(Colors::Yellow),
                            cli_color.text_style(TextStyles::Bold),
                            cli_color.color(Colors::Reset)
                        );
                        println!(
                            "{}{}{}",
                            cli_color.color(Colors::Blue),
                            e.to_string().split("@Halt:").collect::<Vec<&str>>()[1]
                                .split("@")
                                .collect::<Vec<&str>>()[0]
                                .trim(),
                            cli_color.color(Colors::Red)
                        );
                        println!(
                            "\n{}-----------------{}\n\n",
                            cli_color.color(Colors::Blue),
                            cli_color.color(Colors::Reset)
                        );
                        return;
                    }

                    println!(
                        "\n\n{}-----------------{}\n",
                        cli_color.color(Colors::Blue),
                        cli_color.color(Colors::Reset)
                    );
                    println!(
                        "{}{}Oh no! A internal error occured;{}",
                        cli_color.color(Colors::Red),
                        cli_color.text_style(TextStyles::Bold),
                        cli_color.color(Colors::Red)
                    );
                    println!(
                        "{}Can you please share this error with us? This can help us solve issue faster. All you have to do is follow the link below {}[{}CTRL + Mouse Left Click might help{}]",
                        cli_color.color(Colors::Green),
                        cli_color.color(Colors::Reset),
                        cli_color.color(Colors::Yellow),
                        cli_color.color(Colors::Reset),
                    );

                    let line_and_col = if let Some(real_loc) = e.location() {
                        format!("{}:{}", real_loc.line(), real_loc.column())
                    } else {
                        "?:?".to_string()
                    };
                    println!("\n{}{}https://github.com/behemehal/Ellie-Language/issues/new?labels=compiler,bug,Internal%20Error&title=Ellie%20Internal%20Error-{}&body=%23%20Ellie%20Internal%20Error%0AGenerated%20by%20elliec%20located%20at%20{}%0AEllieVersion:{}%0A{}", cli_color.text_style(TextStyles::Underline),cli_color.color(Colors::Green),line_and_col, line_and_col, engine_constants::ELLIE_ENGINE_VERSION, cli_color.color(Colors::Reset));
                    println!(
                        "\n{}-----------------{}\n\n",
                        cli_color.color(Colors::Blue),
                        cli_color.color(Colors::Reset)
                    );
                    std::process::exit(1);
                }));
            }

            let target_path = {
                let path = Path::new(matches.value_of("target").unwrap().clone());
                if path.exists() {
                    matches.value_of("target").unwrap().to_string()
                } else {
                    println!(
                        "{}Error:{} Target path does not exist",
                        cli_color.color(Colors::Red),
                        cli_color.color(Colors::Reset)
                    );
                    std::process::exit(1);
                }
            };

            let project_name = {
                let file_name = Path::new(&target_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();

                if file_name.contains(".") {
                    file_name.split(".").next().unwrap().to_string()
                } else {
                    file_name.to_string()
                }
            };

            let formatter_settings = format_file::FormatterSettings {
                json_log: matches.is_present("jsonLog"),
                name: project_name,
                file_name: Path::new(&target_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                show_debug_lines: matches.is_present("showDebugLines"),
                format_all: true,
            };

            format_file::format_file(
                Path::new(&target_path),
                Path::new(
                    &Path::new(&target_path)
                        .parent()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                ),
                formatter_settings,
            )
        }
        Some(("analyze", matches)) => {
            todo!()
        }
        Some(("version", matches)) => {
            let mut output = outputs::VERSION_DETAILED.clone();
            if matches.is_present("detailed") {
                if matches.is_present("jsonLog") {
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: version.to_string(),
                    });
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "git_hash".to_string(),
                        value: engine_constants::ELLIE_BUILD_GIT_HASH.to_owned(),
                    });
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "build_date".to_string(),
                        value: engine_constants::ELLIE_BUILD_DATE.to_owned(),
                    });
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "engine_version".to_string(),
                        value: engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "code".to_string(),
                        value: engine_constants::ELLIE_ENGINE_VERSION_NAME.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "tokenizer_version".to_string(),
                        value: engine_constants::ELLIE_TOKENIZER_VERSION.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "parser_version".to_string(),
                        value: engine_constants::ELLIE_PARSER_VERSION.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "bytecode_version".to_string(),
                        value: engine_constants::ELLIE_BYTECODE_VERSION.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "core_version".to_string(),
                        value: engine_constants::ELLIE_CORE_VERSION.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "EllieFMT v{} ({}: {})\nEllie v{} - Code: {}\nTokenizer Version: v{}\nCore version: v{}\n",
                        version,
                        engine_constants::ELLIE_BUILD_GIT_HASH,
                        engine_constants::ELLIE_BUILD_DATE,
                        engine_constants::ELLIE_ENGINE_VERSION,
                        engine_constants::ELLIE_ENGINE_VERSION_NAME,
                        engine_constants::ELLIE_TOKENIZER_VERSION,
                        engine_constants::ELLIE_CORE_VERSION,
                    );
                }
            } else {
                if matches.is_present("jsonLog") {
                    let mut output = outputs::VERSION.clone();
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: version.to_string(),
                    });
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "git_hash".to_string(),
                        value: engine_constants::ELLIE_BUILD_GIT_HASH.to_owned(),
                    });
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "build_date".to_string(),
                        value: engine_constants::ELLIE_BUILD_DATE.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "EllieFMT v{}({} : {}) ",
                        version,
                        engine_constants::ELLIE_BUILD_GIT_HASH,
                        engine_constants::ELLIE_BUILD_DATE
                    );
                }
            }
        }
        _ => unreachable!(),
    }
}
