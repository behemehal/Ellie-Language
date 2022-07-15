#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[cfg(feature = "build-cli")]
#[path = "src/cli_options.rs"]
mod cli_options;

#[cfg(feature = "build-cli")]
use clap_complete::generate;

use path_absolutize::Absolutize;
use regex::Regex;
use toml::Value;

pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_id = match self {
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
        write!(f, "{}{}", '\u{001b}', color_id)
    }
}

pub fn read_file<P: AsRef<Path>>(file_dir: P) -> Result<String, String> {
    let file_read = File::open(file_dir);
    match file_read {
        Err(r) => Err(r.to_string()),
        Ok(mut file) => {
            let mut file_content = Vec::new();
            match file.read_to_end(&mut file_content) {
                Ok(_) => match String::from_utf8(file_content) {
                    Ok(code_string) => Ok(code_string),
                    Err(e) => Err(e.to_string()),
                },
                Err(e) => Err(e.to_string()),
            }
        }
    }
}

#[path = "src/engine_constants.rs"]
mod engine_constants;

use std::{
    collections::hash_map::DefaultHasher,
    env,
    fmt::Display,
    fs::{self, File},
    hash::{Hash, Hasher},
    io::Read,
    path::Path,
};

fn main() {
    let ellie_engine_version: Value;
    let ellie_engine_version_name: Value;
    let ellie_core_version: Value;
    let ellie_tokenizer_version: Value;
    let ellie_parser_version: Value;
    let ellie_bytecode_version: Value;
    let ellie_vm_version: Value;
    let ellie_fmt_version: Value;

    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + &"/Cargo.toml".to_owned()) {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_engine_version = toml["package"]["version"].clone();
            ellie_engine_version_name = toml["package"]["version_code"].clone();
        }
        Err(_) => {
            panic!(
                "Failed to build ellie constants, cannot read {}Cargo.toml{}",
                Colors::Yellow,
                Colors::Reset,
            )
        }
    }

    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + &"/core/Cargo.toml".to_owned()) {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_core_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            panic!(
                "Failed to build ellie constants, cannot read {}core/Cargo.toml{}",
                Colors::Yellow,
                Colors::Reset,
            )
        }
    }

    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + &"/tokenizer/Cargo.toml".to_owned()) {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_tokenizer_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            panic!(
                "Failed to build ellie constants, cannot read {}tokenizer/Cargo.toml{}",
                Colors::Yellow,
                Colors::Reset,
            )
        }
    }

    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + &"/parser/Cargo.toml".to_owned()) {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_parser_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            panic!(
                "Failed to build ellie constants, cannot read {}parser/Cargo.toml{}",
                Colors::Yellow,
                Colors::Reset,
            )
        }
    }

    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + &"/bytecode/Cargo.toml".to_owned()) {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_bytecode_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            panic!(
                "Failed to build ellie constants, cannot read {}bytecode/Cargo.toml{}",
                Colors::Yellow,
                Colors::Reset,
            )
        }
    }

    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + &"/vm/Cargo.toml".to_owned()) {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_vm_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            panic!(
                "Failed to build ellie constants, cannot read {}vm/Cargo.toml{}",
                Colors::Yellow,
                Colors::Reset,
            )
        }
    }

    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + &"/fmt/Cargo.toml".to_owned()) {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_fmt_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            panic!(
                "Failed to build ellie constants, cannot read {}vm/Cargo.toml{}",
                Colors::Yellow,
                Colors::Reset,
            )
        }
    }

    #[cfg(feature = "build-cli")]
    {
        let mut bash =
            File::create(env!("CARGO_MANIFEST_DIR").to_string() + "/target/elliec_completion_bash")
                .unwrap();
        let mut fish =
            File::create(env!("CARGO_MANIFEST_DIR").to_string() + "/target/elliec_completion_fish")
                .unwrap();
        let mut zsh =
            File::create(env!("CARGO_MANIFEST_DIR").to_string() + "/target/elliec_completion_zsh")
                .unwrap();
        let mut powershell = File::create(
            env!("CARGO_MANIFEST_DIR").to_string() + "/target/elliec_completion_powershell",
        )
        .unwrap();

        let cmd = cli_options::generate_elliec_options();
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
            clap_complete::shells::Zsh,
            &mut cmd.clone(),
            cmd.get_name().to_string(),
            &mut zsh,
        );

        generate(
            clap_complete::shells::PowerShell,
            &mut cmd.clone(),
            cmd.get_name().to_string(),
            &mut powershell,
        );
    }

    #[cfg(feature = "build-cli")]
    {
        let mut bash = File::create(
            env!("CARGO_MANIFEST_DIR").to_string() + "/target/ellievm_completion_bash",
        )
        .unwrap();
        let mut fish = File::create(
            env!("CARGO_MANIFEST_DIR").to_string() + "/target/ellievm_completion_fish",
        )
        .unwrap();
        let mut zsh =
            File::create(env!("CARGO_MANIFEST_DIR").to_string() + "/target/ellievm_completion_zsh")
                .unwrap();
        let mut powershell = File::create(
            env!("CARGO_MANIFEST_DIR").to_string() + "/target/ellievm_completion_powershell",
        )
        .unwrap();

        let cmd = cli_options::generate_ellievm_options();
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
            clap_complete::shells::Zsh,
            &mut cmd.clone(),
            cmd.get_name().to_string(),
            &mut zsh,
        );

        generate(
            clap_complete::shells::PowerShell,
            &mut cmd.clone(),
            cmd.get_name().to_string(),
            &mut powershell,
        );
    }

    #[cfg(feature = "build-cli")]
    {
        let mut bash = File::create(
            env!("CARGO_MANIFEST_DIR").to_string() + "/target/elliefmt_completion_bash",
        )
        .unwrap();
        let mut fish = File::create(
            env!("CARGO_MANIFEST_DIR").to_string() + "/target/elliefmt_completion_fish",
        )
        .unwrap();
        let mut zsh = File::create(
            env!("CARGO_MANIFEST_DIR").to_string() + "/target/elliefmt_completion_zsh",
        )
        .unwrap();
        let mut powershell = File::create(
            env!("CARGO_MANIFEST_DIR").to_string() + "/target/elliefmt_completion_powershell",
        )
        .unwrap();

        let cmd = cli_options::generate_elliefmt_options();
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
            clap_complete::shells::Zsh,
            &mut cmd.clone(),
            cmd.get_name().to_string(),
            &mut zsh,
        );

        generate(
            clap_complete::shells::PowerShell,
            &mut cmd.clone(),
            cmd.get_name().to_string(),
            &mut powershell,
        );
    }

    fs::write(
        env!("CARGO_MANIFEST_DIR").to_owned() + &"/src/engine_constants.rs",
        format!(
            r#"pub static ELLIE_ENGINE_VERSION : &'static str = &{ellie_engine_version};
pub static ELLIE_ENGINE_VERSION_NAME : &'static str = &{ellie_engine_version_name};
pub static ELLIE_CORE_VERSION : &'static str = &{ellie_core_version};
pub static ELLIE_TOKENIZER_VERSION : &'static str = &{ellie_tokenizer_version};
pub static ELLIE_PARSER_VERSION : &'static str = &{ellie_parser_version};
pub static ELLIE_BYTECODE_VERSION : &'static str = &{ellie_bytecode_version};
pub static ELLIE_VM_VERSION : &'static str = &{ellie_vm_version};
pub static ELLIE_FMT_VERSION : &'static str = &{ellie_fmt_version};"#,
        ),
    )
    .unwrap();
}
