use crate::engine_constants;
use alloc::{borrow::ToOwned, format};
use ellie_cli_utils::{outputs, utils};
use ellie_parser::parser;
use std::{path::Path, println};

pub fn parse(target_path: &Path, json_log: bool) {
    match utils::read_file_bin(target_path) {
        Ok(file_content) => match bincode::deserialize::<parser::Module>(file_content.as_slice()) {
            Ok(module) => {
                let current_ellie_version = ellie_core::defs::Version::build_from_string(
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
                            utils::Colors::Cyan,
                            utils::Colors::Reset,
                            utils::Colors::Yellow,
                            module.ellie_version.major,
                            module.ellie_version.minor,
                            module.ellie_version.bug,
                            utils::Colors::Reset,
                            utils::Colors::Yellow,
                            current_ellie_version.major,
                            current_ellie_version.minor,
                            current_ellie_version.bug,
                            utils::Colors::Reset,
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
                    if module.modules.len() > 0 {
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
                        utils::Colors::Green,
                        utils::Colors::Reset,
                        utils::Colors::Cyan,
                        utils::Colors::Reset
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
                        utils::Colors::Red,
                        utils::Colors::Reset,
                        utils::Colors::Cyan,
                        e,
                        utils::Colors::Reset
                    );
                }
                std::process::exit(1);
            }
        },
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
