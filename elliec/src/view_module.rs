use bincode::Options;
use ellie_engine::{
    ellie_core::defs::{PlatformArchitecture, Version},
    ellie_parser::parser,
    ellie_renderer_utils::{
        outputs,
        utils::{read_file_bin, CliColor, ColorDisplay, Colors},
    },
    engine_constants,
};
use std::path::Path;

pub fn parse(target_path: &Path, json_log: bool, target_arch: PlatformArchitecture) {
    let cli_color = &CliColor;
    match read_file_bin(target_path) {
        Ok(file_content) => {
            let config = bincode::options()
                .with_big_endian()
                .with_fixint_encoding()
                .with_limit(match target_arch {
                    PlatformArchitecture::B16 => 65535,
                    PlatformArchitecture::B32 => 4294967295,
                    PlatformArchitecture::B64 => 18446744073709551615,
                });
            match config.deserialize::<parser::Module>(file_content.as_slice()) {
                Ok(module) => {
                    let current_ellie_version = Version::build_from_string(
                        engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                    );
                    if current_ellie_version != module.ellie_version {
                        if json_log {
                            let mut cli_module_output = outputs::LEGACY_MODULE.clone();
                            cli_module_output.extra.push(outputs::CliOuputExtraData {
                                key: 0,
                                value: module.ellie_version.clone(),
                            })
                        } else {
                            println!(
                            "\n{}Info{}: This module is legacy, used ellie_version: {}{}.{}.{}{} current ellie_version: {}{}.{}.{}{}",
                            cli_color.color(Colors::Cyan),
                            cli_color.color(Colors::Reset),
                            cli_color.color(Colors::Yellow),
                            module.ellie_version.major,
                            module.ellie_version.minor,
                            module.ellie_version.bug,
                            cli_color.color(Colors::Reset),
                            cli_color.color(Colors::Yellow),
                            current_ellie_version.major,
                            current_ellie_version.minor,
                            current_ellie_version.bug,
                            cli_color.color(Colors::Reset),
                        );
                        }
                    }

                    if json_log {
                        let mut cli_module_output = outputs::READ_BINARY_MODULE_SUCCEDED.clone();

                        cli_module_output.extra.push(outputs::CliOuputExtraData {
                            key: 0,
                            value: outputs::CliModuleOutput {
                                name: module.name,
                                description: module.description,
                                version: format!(
                                    "{}.{}.{}",
                                    module.version.minor, module.version.major, module.version.bug
                                ),
                                modules: module
                                    .modules
                                    .iter()
                                    .map(|x| outputs::CliInnerModuleOutput {
                                        name: x.name.clone(),
                                        version: format!(
                                            "{}.{}.{}",
                                            x.version.minor, x.version.minor, x.version.bug
                                        ),
                                    })
                                    .collect(),
                            },
                        });

                        println!(
                            "{}",
                            serde_json::to_string_pretty(&cli_module_output).unwrap()
                        );
                    } else {
                        let mut output = format!("ModuleName        = {}{}\nModuleDescription = {}\nModuleVersion     = {}.{}.{}\nEllieVersion      = {}.{}.{}", module.name, if module.is_library {" (Library)"} else {""}, module.description, module.version.major, module.version.minor, module.version.bug, module.ellie_version.major, module.ellie_version.minor, module.ellie_version.bug);
                        if !module.modules.is_empty() {
                            output.push_str("\nInnerModules      =\n");
                        }

                        for inner_module in module.modules {
                            output += format!(
                                "\tModuleName    =\t{}\n\tModuleVersion =\t{}.{}.{}\n",
                                inner_module.name,
                                inner_module.version.major,
                                inner_module.version.minor,
                                inner_module.version.bug
                            )
                            .as_str();
                        }
                        println!("{}", output);
                        println!(
                            "\n{}Success{}: {}Decoding complete{}",
                            cli_color.color(Colors::Green),
                            cli_color.color(Colors::Reset),
                            cli_color.color(Colors::Cyan),
                            cli_color.color(Colors::Reset)
                        );
                    }
                }
                Err(e) => {
                    if json_log {
                        let mut cli_module_output = outputs::READ_BINARY_MODULE_ERROR.clone();
                        cli_module_output.extra.push(outputs::CliOuputExtraData {
                            key: "file".to_owned(),
                            value: ".".to_owned(),
                        });
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&cli_module_output).unwrap()
                        );
                    } else {
                        println!(
                            "{}Error{} 0x1: {}Failed to decode module [{}]].{}",
                            cli_color.color(Colors::Red),
                            cli_color.color(Colors::Reset),
                            cli_color.color(Colors::Cyan),
                            e,
                            cli_color.color(Colors::Reset)
                        );
                    }
                    std::process::exit(1);
                }
            }
        }
        Err(err) => {
            if json_log {
                let mut cli_module_output = outputs::READ_BINARY_MODULE_ERROR.clone();
                cli_module_output.extra.push(outputs::CliOuputExtraData {
                    key: "file".to_owned(),
                    value: ".".to_owned(),
                });
                println!(
                    "{}",
                    serde_json::to_string_pretty(&cli_module_output).unwrap()
                );
            } else {
                println!(
                    "Unable to read file ~{} [{}]",
                    target_path.to_str().unwrap().to_owned(),
                    err
                );
            }
            std::process::exit(1);
        }
    }
}
