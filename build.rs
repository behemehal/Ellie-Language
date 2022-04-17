#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[cfg(feature = "build-cli")]
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
    let bytecode_version;
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
            bytecode_version = ellie_lang_toml["dependencies"]["ellie_bytecode"]["version"].clone();
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

    #[cfg(feature = "build-cli")]
    {
        let mut bash =
            File::create(env::var("OUT_DIR").unwrap().to_owned() + "/elliec_completion_bash")
                .unwrap();
        let mut fish =
            File::create(env::var("OUT_DIR").unwrap().to_owned() + "/elliec_completion_fish")
                .unwrap();
        let mut zsh =
            File::create(env::var("OUT_DIR").unwrap().to_owned() + "/elliec_completion_zsh")
                .unwrap();
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
    }

    fs::write(
        env!("CARGO_MANIFEST_DIR").to_owned() + &"/src/engine_constants.rs",
        format!(
            "pub static ELLIE_VERSION: &'static str = &{};\npub static ELLIE_VERSION_NAME: &'static str = &{};\npub static ELLIE_TOKENIZER_VERSION: &'static str = &{};\npub static ELLIE_PARSER_VERSION: &'static str = &{};\npub static ELLIE_BYTECODE_VERSION: &'static str = &{};\npub static ELLIE_RUNTIME_VERSION: &'static str = &{};\npub static ELLIE_CORE_VERSION: &'static str = &{};\n",
            ellie_version,
            ellie_version_name,
            tokenizer_version,
            parser_version,
            bytecode_version,
            runtime_version,
            core_version
        ),
    ).unwrap();
}
