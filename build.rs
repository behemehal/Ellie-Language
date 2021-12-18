#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use ellie_core;
use ellie_parser::parser;
use ellie_tokenizer::tokenizer::{self, ResolvedImport};
use path_absolutize::Absolutize;
use regex::Regex;
use toml::Value;

#[path = "src/cli_utils.rs"]
mod cli_utils;

use std::{
    collections::hash_map::DefaultHasher,
    fs::{self, File},
    hash::{Hash, Hasher},
    io::Read,
    path::Path,
};

fn main() {
    let ellie_version;
    let ellie_version_name;
    let tokenizer_version;
    let parser_version;
    let runtime_version;
    let core_version;
    match cli_utils::read_file(&("./Cargo.toml".to_owned())) {
        Ok(cargo_toml) => {
            let ellie_lang_toml = cargo_toml.parse::<Value>().unwrap();
            ellie_version = ellie_lang_toml["package"]["version"].clone();
            ellie_version_name = ellie_lang_toml["package"]["version_code"].clone();
            tokenizer_version =
                ellie_lang_toml["dependencies"]["ellie_tokenizer"]["version"].clone();
            parser_version = ellie_lang_toml["dependencies"]["ellie_parser"]["version"].clone();
            runtime_version = ellie_lang_toml["dependencies"]["ellie_runtime"]["version"].clone();
            core_version = ellie_lang_toml["dependencies"]["ellie_core"]["version"].clone();
        }
        Err(_) => {
            panic!(
                "Failed to build ellie constants, cannot read {}Cargo.toml{}",
                cli_utils::Colors::Yellow,
                cli_utils::Colors::Reset,
            )
        }
    }

    match cli_utils::read_file(&("./lib/ellie.ei".to_owned())) {
        Ok(ellie_lib) => {
            let version_line_regex = Regex::new(
                "(@(\\s)*version(\\s)*=)(\\s)*(\")*(?P<version>\"\\^|\\~?(\\d|x|\\*)+\\.(\\d|x|\\*)+\\.(\\d|x|\\*))*(\"|()*;)",
            ).unwrap();
            let lib_version_number = &version_line_regex.captures(&ellie_lib).unwrap();
            let lib_version = lib_version_number["version"].to_owned();

            match cli_utils::read_file(&("./core/src/builded_libraries.rs".to_owned())) {
                Ok(builded_libraries) => {
                    let current_version_number =
                        &version_line_regex.captures(&builded_libraries).unwrap();
                    let current_lib_version = current_version_number["version"].to_owned();

                    if current_lib_version != lib_version {
                        let mut pager = tokenizer::Pager::new(
                            ellie_lib,
                            Path::new("./lib/ellie.ei").to_str().unwrap().to_string(),
                            |path, file_name| {
                                let path = Path::new(&path)
                                    .parent()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                                let file = if cli_utils::file_exists(
                                    path.clone() + "/" + &file_name.clone(),
                                ) {
                                    Some(path.clone() + "/" + &file_name.clone())
                                } else if cli_utils::file_exists(
                                    path.clone() + "/" + &file_name.clone() + ".ei",
                                ) {
                                    Some(path.clone() + "/" + &file_name.clone() + ".ei")
                                } else {
                                    None
                                };

                                match file {
                                    Some(file) => {
                                        let file = Path::new(&file).absolutize().unwrap();
                                        match cli_utils::read_file(
                                            &file.to_str().unwrap().to_string(),
                                        ) {
                                            Ok(ext) => {
                                                let mut hasher = DefaultHasher::new();
                                                ext.hash(&mut hasher);
                                                ResolvedImport {
                                                    found: true,
                                                    code: ext,
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
                            },
                            Some(343),
                        );

                        match pager.run() {
                            Err(e) => {
                                cli_utils::print_errors(&e, |path| {
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
                            Ok(_) => {
                                let mut parser = parser::Parser::new(pager.pages.clone(), Some(343));
                                parser.parse();

                                if !parser.informations.has_no_warnings() {
                                    cli_utils::print_warnings(
                                        &parser.informations.warnings,
                                        |path| match cli_utils::read_file(&path) {
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
                                        },
                                    );
                                }

                                if !parser.informations.has_no_errors() {
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
                                } else {
                                    println!(
                                        "\nCompiling {}succeeded{} with {}{} warnings{}.",
                                        cli_utils::Colors::Green,
                                        cli_utils::Colors::Reset,
                                        cli_utils::Colors::Yellow,
                                        parser.informations.warnings.len(),
                                        cli_utils::Colors::Reset,
                                    );

                                    let json =
                                        serde_json::to_string(&parser.processed_pages).unwrap();
                                    fs::write(
                                        "./core/src/builded_libraries.rs",
                                        format!("//@version = \"{}\";\npub static ELLIE_STANDARD_LIBRARY : &str = {:#?};\n", lib_version, json),
                                    )
                                    .unwrap();
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    panic!(
                        "{}[Fail]{}: Cannot read file {}~./lib/{}.ei{}\n{:#?}",
                        cli_utils::Colors::Red,
                        cli_utils::Colors::Reset,
                        cli_utils::Colors::Yellow,
                        "builded_libraries.rs",
                        cli_utils::Colors::Reset,
                        err
                    )
                }
            }

            fs::write(
                "./src/cli_constants.rs",
                format!(
                    "pub static ELLIE_VERSION: &'static str = &{};\npub static ELLIE_VERSION_NAME: &'static str = &{};\npub static ELLIE_TOKENIZER_VERSION: &'static str = &{};\npub static ELLIE_PARSER_VERSION: &'static str = &{};\npub static ELLIE_RUNTIME_VERSION: &'static str = &{};\npub static ELLIE_CORE_VERSION: &'static str = &{};\npub static ELLIE_STD_VERSION: &'static str = &\"{}\";\n",
                    ellie_version,
                    ellie_version_name,
                    tokenizer_version,
                    parser_version,
                    runtime_version,
                    core_version,
                    lib_version
                ),
            ).unwrap();
        }
        Err(err) => {
            panic!(
                "{}[Fail]{}: Cannot read file {}~./lib/{}.ei{}\n{:#?}",
                cli_utils::Colors::Red,
                cli_utils::Colors::Reset,
                cli_utils::Colors::Yellow,
                "ellie",
                cli_utils::Colors::Reset,
                err
            );
        }
    }
}
