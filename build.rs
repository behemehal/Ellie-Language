#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use clap_complete::generate;
use ellie_core;
use ellie_parser::parser;
use ellie_tokenizer::tokenizer::{self, ResolvedImport};
use path_absolutize::Absolutize;
use regex::Regex;
use toml::Value;

#[path = "src/cli_utils.rs"]
mod cli_utils;

#[path = "src/engine_constants.rs"]
mod engine_constants;

use std::{
    collections::hash_map::DefaultHasher,
    env,
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
    match cli_utils::read_file(env!("CARGO_MANIFEST_DIR").to_owned() + &"/Cargo.toml".to_owned()) {
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

    let mut bash =
        File::create(env::var("OUT_DIR").unwrap().to_owned() + "/elliec_completion_bash").unwrap();
    let mut fish =
        File::create(env::var("OUT_DIR").unwrap().to_owned() + "/elliec_completion_fish").unwrap();
    let mut zsh =
        File::create(env::var("OUT_DIR").unwrap().to_owned() + "/elliec_completion_zsh").unwrap();
    let mut powershell =
        File::create(env::var("OUT_DIR").unwrap().to_owned() + "/elliec_completion_powershell")
            .unwrap();

    let cmd = cli_utils::generate_elliec_options();
    generate(
        clap_complete::shells::Bash,
        &mut cmd.clone(),
        cmd.get_name().to_string(),
        &mut bash,
    );

    generate(
        clap_complete::shells::Fish,
        &mut cmd.clone(),
        cmd.get_name().to_string(),
        &mut fish,
    );

    generate(
        clap_complete::shells::Fish,
        &mut cmd.clone(),
        cmd.get_name().to_string(),
        &mut zsh,
    );

    generate(
        clap_complete::shells::Fish,
        &mut cmd.clone(),
        cmd.get_name().to_string(),
        &mut powershell,
    );

    match cli_utils::read_file(
        &(env!("CARGO_MANIFEST_DIR").to_owned() + &"/Ellie-Standard-Library/ellie.ei".to_owned()),
    ) {
        Ok(ellie_lib) => {
            let version_line_regex = Regex::new(
                "(@(\\s)*version(\\s)*=)(\\s)*(\")*(?P<version>\"\\^|\\~?(\\d|x|\\*)+\\.(\\d|x|\\*)+\\.(\\d|x|\\*))*(\"|()*;)",
            ).unwrap();
            let lib_version_number = &version_line_regex.captures(&ellie_lib).unwrap();
            let lib_version = lib_version_number["version"].to_owned();

            match cli_utils::read_file(
                env!("CARGO_MANIFEST_DIR").to_owned()
                    + &"/core/src/builded_libraries.rs".to_owned(),
            ) {
                Ok(builded_libraries) => {
                    let current_version_number =
                        &version_line_regex.captures(&builded_libraries).unwrap();
                    let current_lib_version = current_version_number["version"].to_owned();

                    if current_lib_version != lib_version
                        || ellie_core::builded_libraries::BUILDED_ELLIE_VERSION
                            != engine_constants::ELLIE_VERSION
                    {
                        let mut main_hasher = DefaultHasher::new();
                        ellie_lib.hash(&mut main_hasher);

                        let first_page_hash = main_hasher.finish();

                        let starter_name = "<ellie_module_std>".to_string();
                        let mut pager = tokenizer::Pager::new(
                            ellie_lib,
                            "/ellie.ei".to_string(),
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
                                                .replace(&starter_name, "./Ellie-Standard-Library/")
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
                                                            hash: hasher.finish(),
                                                            path,
                                                            ..Default::default()
                                                        }
                                                    }
                                                    Err(e) => ResolvedImport {
                                                        found: false,
                                                        resolve_error: "Cannot find file"
                                                            .to_string(),
                                                        ..Default::default()
                                                    },
                                                }
                                            } else {
                                                panic!(
                                                    "Cannot find file {}",
                                                    Path::new(&real_path)
                                                        .absolutize()
                                                        .unwrap()
                                                        .display()
                                                );
                                            }
                                        }
                                        Err(e) => {
                                            if e == 1 {
                                                ResolvedImport {
                                                    found: false,
                                                    resolve_error:
                                                        "Cannot access outside of workspace"
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
                            first_page_hash.clone(),
                        );

                        match pager.run() {
                            Err(e) => {
                                cli_utils::print_errors(
                                    &e,
                                    |path| match cli_utils::read_file(
                                        &path.replace(&starter_name, "./Ellie-Standard-Library/"),
                                    ) {
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
                                    true,
                                    |path| {
                                        path.replace(&starter_name, "./Ellie-Standard-Library/")
                                            .to_string()
                                    },
                                );
                                panic!("Build failed");
                            }
                            Ok(_) => {
                                let mut parser = parser::Parser::new(
                                    pager.pages.clone(),
                                    first_page_hash,
                                    ellie_core::defs::Version::build_from_string(
                                        lib_version.clone(),
                                    ),
                                );
                                let workspace = parser.parse(
                                    "ellie_std".to_owned(),
                                    "Ellie Standard Types".to_owned(),
                                    ellie_core::defs::Version::build_from_string(
                                        engine_constants::ELLIE_VERSION.to_owned(),
                                    ),
                                );

                                if !parser.informations.has_no_warnings() {
                                    cli_utils::print_warnings(
                                        &parser.informations.warnings,
                                        |path| match cli_utils::read_file(&path.replace(
                                            &starter_name.clone(),
                                            "./Ellie-Standard-Library/",
                                        )) {
                                            Ok(e) => e,
                                            Err(err) => {
                                                panic!(
                                                    "{}[Internal Error]{} Cannot build warning, read file failed '{}' {}[{}]{}",
                                                    cli_utils::Colors::Red,
                                                    cli_utils::Colors::Reset,
                                                    path,
                                                    cli_utils::Colors::Red,
                                                    err,
                                                    cli_utils::Colors::Reset
                                                );
                                            }
                                        },
                                        |path: String| {
                                            path.replace(&starter_name, "./Ellie-Standard-Library/")
                                                .to_string()
                                        },
                                    );
                                }

                                if !parser.informations.has_no_errors() {
                                    cli_utils::print_errors(
                                        &parser.informations.errors,
                                        |path| match cli_utils::read_file(
                                            &path.replace(
                                                &starter_name,
                                                "./Ellie-Standard-Library/",
                                            ),
                                        ) {
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
                                        },
                                        true,
                                        |path| {
                                            path.replace(&starter_name, "./Ellie-Standard-Library/")
                                                .to_string()
                                        },
                                    );
                                    panic!("\nCompiling {}failed{} with {}{} errors{} and {}{} warnings{}.",
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
                                    let json = serde_json::to_string(&workspace).unwrap();
                                    fs::write(
                                        env!("CARGO_MANIFEST_DIR").to_owned() + &"/core/src/builded_libraries.rs",
                                        format!("//NEVER EDIT THIS FILE WHILE LANGUAGE SERVER IS RUNNING\n//@version = \"{}\";\npub static BUILDED_ELLIE_VERSION: &'static str = \"{}\";\npub static ELLIE_STD_VERSION : crate::defs::Version = crate::defs::Version {{minor: {}, major: {}, bug: {} }};\npub static ELLIE_STANDARD_LIBRARY : &str = {:#?};\n", lib_version, engine_constants::ELLIE_VERSION, lib_version.split(".").nth(0).unwrap(), lib_version.split(".").nth(1).unwrap(),lib_version.split(".").nth(2).unwrap(), json),
                                    )
                                    .unwrap();
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    panic!(
                        "{}[Fail]{}: Cannot read file {}~./Ellie-Standard-Library/{}.ei{}\n{:#?}",
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
                env!("CARGO_MANIFEST_DIR").to_owned() + &"/src/engine_constants.rs",
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
                "{}[Fail]{}: Cannot read file {}~{}/Ellie-Standard-Library/{}.ei{}\n{:#?}",
                cli_utils::Colors::Red,
                cli_utils::Colors::Reset,
                cli_utils::Colors::Yellow,
                &(env!("CARGO_MANIFEST_DIR").to_owned() + &"/Ellie-Standard-Library/ellie.ei".to_owned()),
                "ellie",
                cli_utils::Colors::Reset,
                err
            );
        }
    }
}
