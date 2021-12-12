use ellie_core;
use ellie_parser::parser;
use regex::Regex;
use toml::Value;

#[path = "src/cli_utils.rs"]
mod cli_utils;

use std::{
    collections::hash_map::DefaultHasher,
    fs::{self, File},
    hash::{Hash, Hasher},
    io::Read,
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
            tokenizer_version = ellie_lang_toml["dependencies"]["ellie_tokenizer"]["version"].clone();
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
