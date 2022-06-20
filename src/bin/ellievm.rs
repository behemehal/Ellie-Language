use ellie_engine::{cli_options, cli_outputs, cli_utils};
use std::path::Path;

fn main() {
    let app = cli_options::generate_ellievm_options();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("run", matches)) => {
            if !matches.is_present("allowPanics") {
                std::panic::set_hook(Box::new(|e| {
                    if e.to_string().contains("@Halt") {
                        println!(
                            "\n\n{}-----------------{}\n",
                            cli_utils::Colors::Blue,
                            cli_utils::Colors::Reset
                        );
                        println!(
                            "{}{}VM halted{}\n",
                            cli_utils::Colors::Yellow,
                            cli_utils::TextStyles::Bold,
                            cli_utils::Colors::Reset
                        );
                        println!(
                            "{}{}{}",
                            cli_utils::Colors::Blue,
                            e.to_string().split("@Halt:").collect::<Vec<&str>>()[1]
                                .split("@")
                                .collect::<Vec<&str>>()[0]
                                .trim(),
                            cli_utils::Colors::Red
                        );
                        println!(
                            "\n{}-----------------{}\n\n",
                            cli_utils::Colors::Blue,
                            cli_utils::Colors::Reset
                        );
                        return;
                    }

                    println!(
                        "\n\n{}-----------------{}\n",
                        cli_utils::Colors::Blue,
                        cli_utils::Colors::Reset
                    );
                    println!(
                        "{}{}Oh no! A internal error occured;{}",
                        cli_utils::Colors::Red,
                        cli_utils::TextStyles::Bold,
                        cli_utils::Colors::Red
                    );
                    println!(
                        "{}Can you please share this error with us? This can help us solve issue faster. All you have to do is follow the link below {}[{}CTRL + Mouse Left Click might help{}]",
                        cli_utils::Colors::Green,
                        cli_utils::Colors::Reset,
                        cli_utils::Colors::Yellow,
                        cli_utils::Colors::Reset,
                    );

                    let line_and_col = if let Some(real_loc) = e.location() {
                        format!("{}:{}", real_loc.line(), real_loc.column())
                    } else {
                        "?:?".to_string()
                    };
                    println!("\n{}{}https://github.com/behemehal/Ellie-Language/issues/new?labels=vm,bug,Internal%20Error&title=Ellie%20Internal%20Error-{}&body=%23%20Ellie%20Internal%20Error%0AGenerated%20by%20ellievm%20located%20at%20{}%0AEllieVersion:{}%0A{}", cli_utils::TextStyles::Underline,cli_utils::Colors::Green,line_and_col, line_and_col, ellie_engine::engine_constants::ELLIE_ENGINE_VERSION, cli_utils::Colors::Reset);
                    println!(
                        "\n{}-----------------{}\n\n",
                        cli_utils::Colors::Blue,
                        cli_utils::Colors::Reset
                    );
                    std::process::exit(1);
                }));
            }

            let vm_settings = ellie_engine::run_vm::VmSettings {
                json_log: matches.is_present("jsonLog"),
                warnings: true,
                architecture: match matches.value_of("targetArchitecture") {
                    Some(e) => {
                        if e == "64" {
                            ellie_core::defs::PlatformArchitecture::B64
                        } else if e == "32" {
                            ellie_core::defs::PlatformArchitecture::B32
                        } else if e == "16" {
                            ellie_core::defs::PlatformArchitecture::B16
                        } else {
                            println!(
                                "{}Error:{} Unknown architecture '{}{}{}'",
                                cli_utils::Colors::Red,
                                cli_utils::Colors::Reset,
                                cli_utils::Colors::Yellow,
                                e,
                                cli_utils::Colors::Reset,
                            );
                            std::process::exit(1);
                        }
                    }
                    None => unreachable!(),
                },
            };

            let target_path = {
                let path = Path::new(matches.value_of("target").unwrap().clone());
                if path.exists() {
                    if path.is_file() {
                        matches.value_of("target").unwrap().to_string()
                    } else {
                        println!(
                            "{}Error:{} Given path is not a file",
                            cli_utils::Colors::Red,
                            cli_utils::Colors::Reset
                        );
                        std::process::exit(1);
                    }
                } else {
                    println!(
                        "{}Error:{} Target path does not exist",
                        cli_utils::Colors::Red,
                        cli_utils::Colors::Reset
                    );
                    std::process::exit(1);
                }
            };

            ellie_engine::run_vm::run(Path::new(&target_path), vm_settings);
        }
        Some(("version", matches)) => {
            if matches.is_present("detailed") {
                if matches.is_present("jsonLog") {
                    let mut output = cli_outputs::VERSION_DETAILED.clone();
                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "code".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_ENGINE_VERSION_NAME.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "vm_version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_VM_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "core_version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_CORE_VERSION.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "Ellie v{} - Code: {}\nVM Version: v{}\nCore version: v{}\n",
                        ellie_engine::engine_constants::ELLIE_ENGINE_VERSION,
                        ellie_engine::engine_constants::ELLIE_ENGINE_VERSION_NAME,
                        ellie_engine::engine_constants::ELLIE_VM_VERSION,
                        ellie_engine::engine_constants::ELLIE_CORE_VERSION,
                    );
                }
            } else {
                if matches.is_present("jsonLog") {
                    let mut output = cli_outputs::VERSION.clone();
                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "code".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_ENGINE_VERSION_NAME.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "Ellie v{} - Code: {}",
                        ellie_engine::engine_constants::ELLIE_ENGINE_VERSION,
                        ellie_engine::engine_constants::ELLIE_ENGINE_VERSION_NAME
                    );
                }
            }
        }
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
