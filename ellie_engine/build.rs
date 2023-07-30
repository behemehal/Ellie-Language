#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use chrono::Local;
use path_absolutize::Absolutize;
use regex::Regex;
use toml::Value;

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
    process::Command,
};

fn main() {
    let ellie_engine_version: Value;
    let ellie_engine_version_name = "\"BeautifulTropicalFish\"".to_string();
    let ellie_core_version: Value;
    #[cfg(any(feature = "compiler", feature = "fmt"))]
    let ellie_tokenizer_version: Value;
    #[cfg(feature = "compiler")]
    let ellie_parser_version: Value;
    #[cfg(feature = "compiler")]
    let ellie_bytecode_version: Value;
    #[cfg(feature = "vm")]
    let ellie_vm_version: Value;
    #[cfg(feature = "fmt")]
    let ellie_fmt_version: Value;

    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + "/Cargo.toml") {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_engine_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            return;
        }
    }

    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + "/core/Cargo.toml") {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_core_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            return;
        }
    }

    #[cfg(any(feature = "compiler", feature = "fmt"))]
    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + "/tokenizer/Cargo.toml") {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_tokenizer_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            return;
        }
    }

    #[cfg(feature = "compiler")]
    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + "/parser/Cargo.toml") {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_parser_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            return;
        }
    }

    #[cfg(feature = "compiler")]
    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + "/bytecode/Cargo.toml") {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_bytecode_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            return;
        }
    }

    #[cfg(feature = "vm")]
    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + "/vm/Cargo.toml") {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_vm_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            return;
        }
    }

    #[cfg(feature = "fmt")]
    match read_file(env!("CARGO_MANIFEST_DIR").to_owned() + "/fmt/Cargo.toml") {
        Ok(cargo_toml) => {
            let toml = cargo_toml.parse::<Value>().unwrap();
            ellie_fmt_version = toml["package"]["version"].clone();
        }
        Err(_) => {
            return;
        }
    }

    /*

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

    */

    let mut output = format!(
        r#"pub static ELLIE_ENGINE_VERSION: &str = &{ellie_engine_version};
pub static ELLIE_ENGINE_VERSION_NAME: &str = &{ellie_engine_version_name};
pub static ELLIE_CORE_VERSION: &str = &{ellie_core_version};"#
    );

    #[cfg(feature = "compiler")]
    {
        output += &format!(
            r#"
pub static ELLIE_TOKENIZER_VERSION: &str = &{ellie_tokenizer_version};
pub static ELLIE_PARSER_VERSION: &str = &{ellie_parser_version};
pub static ELLIE_BYTECODE_VERSION: &str = &{ellie_bytecode_version};"#
        );
    }
    //git show HEAD~2 --pretty=format:"%h" --no-patch
    #[cfg(feature = "fmt")]
    {
        #[cfg(feature = "compiler")]
        {
            output += &format!("\npub static ELLIE_FMT_VERSION: &str = &{ellie_fmt_version};");
        }
        #[cfg(not(feature = "compiler"))]
        {
            output += &format!(
                r#"
pub static ELLIE_TOKENIZER_VERSION: &str = &{ellie_tokenizer_version};
pub static ELLIE_PARSER_VERSION : & str = &{ellie_parser_version};
pub static ELLIE_BYTECODE_VERSION : & str = &{ellie_bytecode_version};
pub static ELLIE_FMT_VERSION : & str = &{ellie_fmt_version};"#
            );
        }
    }

    #[cfg(feature = "vm")]
    {
        output += &format!("\npub static ELLIE_VM_VERSION: &str = &{ellie_vm_version};");
    }

    //Add date and git show HEAD~2 --pretty=format:"%h" --no-patch
    let date = Local::now().format("%Y-%m-%d").to_string();
    let git_hash = Command::new("git")
        .args(["show", "HEAD~2", "--pretty=format:%h", "--no-patch"])
        .output()
        .expect("Failed to execute 'git' command. Ellie requires git to be installed to build.")
        .stdout;
    let git_hash = String::from_utf8(git_hash).unwrap();
    let git_hash = git_hash.trim();
    let git_branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Failed to execute 'git' command. Ellie requires git to be installed to build.")
        .stdout;
    let git_branch = String::from_utf8(git_branch).unwrap().replace('\n', "");

    output += &format!("\npub static ELLIE_BUILD_DATE: &str = &\"{date}\";");
    output += &format!("\npub static ELLIE_BUILD_GIT_HASH: &str = &\"{git_hash}\";");
    output += &format!("\npub static ELLIE_BUILD_GIT_BRANCH: &str = &\"{git_branch}\";\n");

    if matches!(env::var("PROFILE"), Ok(e) if e == "release") {
        fs::write(
            env!("CARGO_MANIFEST_DIR").to_owned() + "/src/engine_constants.rs",
            output,
        )
        .unwrap();
    }
}
