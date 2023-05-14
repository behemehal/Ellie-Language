use std::path::Path;

use ellie_cli_utils::{options, utils as cli_utils};
use ellie_engine::engine_constants;

fn main() {
    let version = format!("0.1.0",);
    let app = options::generate_elliefmt_options();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("format", matches)) => {
            if !matches.is_present("allowPanics") {
                std::panic::set_hook(Box::new(|e| {
                    if e.to_string().contains("@Halt") {
                        println!(
                            "\n\n{}-----------------{}\n",
                            cli_utils::Colors::Blue,
                            cli_utils::Colors::Reset
                        );
                        println!(
                            "{}{}Compiler halted{}\n",
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
                    println!("\n{}{}https://github.com/behemehal/Ellie-Language/issues/new?labels=compiler,bug,Internal%20Error&title=Ellie%20Internal%20Error-{}&body=%23%20Ellie%20Internal%20Error%0AGenerated%20by%20elliec%20located%20at%20{}%0AEllieVersion:{}%0A{}", cli_utils::TextStyles::Underline,cli_utils::Colors::Green,line_and_col, line_and_col, engine_constants::ELLIE_ENGINE_VERSION, cli_utils::Colors::Reset);
                    println!(
                        "\n{}-----------------{}\n\n",
                        cli_utils::Colors::Blue,
                        cli_utils::Colors::Reset
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
                        cli_utils::Colors::Red,
                        cli_utils::Colors::Reset
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

            let formatter_settings = ellie_engine::binary_tools::format_file::FormatterSettings {
                json_log: matches.is_present("jsonLog"),
                name: project_name,
                file_name: Path::new(&target_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                show_debug_lines: matches.is_present("showDebugLines"),
                analyze: false,
            };

            ellie_engine::binary_tools::format_file::format_file(
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
        Some(("version", matches)) => {}
        _ => unreachable!(),
    }
}
