use ellie_core;
use ellie_parser::parser;
use regex::Regex;
use toml::Value;

#[path = "src/terminal_colors.rs"]
mod terminal_colors;
//use serde_json;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    env,
    fs::{self, File},
    hash::{Hash, Hasher},
    io::Read,
};

fn read_file(file_dir: &str) -> Result<String, String> {
    let file_read = File::open(file_dir);
    match file_read {
        Err(r) => Err(r.to_string()),
        Ok(mut file) => {
            let mut file_content = Vec::new();
            file.read_to_end(&mut file_content).expect("Unable to read");
            let code_string = String::from_utf8(file_content);
            Ok(code_string.unwrap())
        }
    }
}

fn resolve_import(
    _: ellie_core::defs::ParserOptions,
    lib_name: String,
    native_header: bool,
) -> ellie_parser::parser::ResolvedImport {
    std::eprintln!(
        "{}[ReadingFile]{}: {}~./lib/{}.ei{}",
        terminal_colors::get_color(terminal_colors::Colors::Magenta),
        terminal_colors::get_color(terminal_colors::Colors::Reset),
        terminal_colors::get_color(terminal_colors::Colors::Yellow),
        lib_name,
        terminal_colors::get_color(terminal_colors::Colors::Reset),
    );
    let file_extension = if native_header {
        ".eih".to_owned()
    } else {
        ".ei".to_owned()
    };
    match read_file(&("./lib/".to_owned() + &lib_name + &file_extension)) {
        Ok(e) => {
            let mut id_hasher = DefaultHasher::new();
            let mut resolution_id_hasher = DefaultHasher::new();
            e.hash(&mut resolution_id_hasher);
            ellie_core::utils::generate_hash().hash(&mut id_hasher);

            ellie_parser::parser::ResolvedImport {
                found: true,
                resolved_path: ("./lib/".to_owned() + &lib_name + &file_extension),
                file_content: ellie_parser::parser::ResolvedFileContent::Raw(e),
                resolution_id: resolution_id_hasher.finish(),
                id: id_hasher.finish(),
                ..Default::default()
            }
        }
        Err(err) => {
            panic!(
                "{}[Fail]{}: Cannot read file {}~./lib/{}{}{}\n{:#?}",
                terminal_colors::get_color(terminal_colors::Colors::Red),
                terminal_colors::get_color(terminal_colors::Colors::Reset),
                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                lib_name,
                file_extension,
                terminal_colors::get_color(terminal_colors::Colors::Reset),
                err
            );
        }
    }
}

fn parse(contents: String, file_name: String) -> ellie_parser::parser::ParserResponse {
    std::eprintln!(
        "{}[ParsingFile]{}: {}~./lib/{}.ei{}",
        terminal_colors::get_color(terminal_colors::Colors::Cyan),
        terminal_colors::get_color(terminal_colors::Colors::Reset),
        terminal_colors::get_color(terminal_colors::Colors::Yellow),
        file_name,
        terminal_colors::get_color(terminal_colors::Colors::Reset),
    );
    let mut builded: Vec<(String, ellie_parser::parser::ResolvedImport)> = Vec::new();

    let parser = parser::Parser::new(
        contents.clone(),
        |x, y, z| {
            let found = builded.into_iter().find(|x| x.0 == y);

            if let Some(pre_built) = found {
                pre_built.1
            } else {
                let built = resolve_import(x, y, z);
                builded.push((y, built.clone()));
                built
            }
        },
        |_| {},
        ellie_core::defs::ParserOptions {
            path: "./lib/".to_owned() + &file_name.to_string(),
            functions: true,
            break_on_error: false,
            loops: true,
            conditions: true,
            classes: true,
            import_std: false,
            enums: true,
            dynamics: true,
            global_variables: true,
            getters: true,
            setters: true,
            line_ending: "\\n".to_owned(),
            collectives: true,
            variables: true,
            constants: true,
            parser_type: ellie_core::defs::ParserType::RawParser,
            allow_import: true,
        },
    );
    let parsed = parser.map();

    if parsed.syntax_errors.len() == 0 {
        std::eprintln!(
            "{}[ParsingSuccess]{}: {}~./lib/{}.ei{}",
            terminal_colors::get_color(terminal_colors::Colors::Green),
            terminal_colors::get_color(terminal_colors::Colors::Reset),
            terminal_colors::get_color(terminal_colors::Colors::Yellow),
            file_name,
            terminal_colors::get_color(terminal_colors::Colors::Reset),
        );
    } else {
        panic!(
            "{}[ParsingFailed]{}: {}~./lib/{}.ei{}\n{:#?}",
            terminal_colors::get_color(terminal_colors::Colors::Red),
            terminal_colors::get_color(terminal_colors::Colors::Reset),
            terminal_colors::get_color(terminal_colors::Colors::Yellow),
            file_name,
            terminal_colors::get_color(terminal_colors::Colors::Reset),
            parsed.syntax_errors
        );
    }
    parsed
}

fn main() {
    let rebuild_std = env::args().any(|x| x == "-rstd" || x == "--rebuild-std");
    let ellie_version;
    let ellie_version_name;
    let parser_version;
    let runtime_version;
    let core_version;
    match read_file(&("./Cargo.toml".to_owned())) {
        Ok(cargo_toml) => {
            let ellie_lang_toml = cargo_toml.parse::<Value>().unwrap();
            ellie_version = ellie_lang_toml["package"]["version"].clone();
            ellie_version_name = ellie_lang_toml["package"]["version_code"].clone();
            parser_version = ellie_lang_toml["dependencies"]["ellie_parser"]["version"].clone();
            runtime_version = ellie_lang_toml["dependencies"]["ellie_runtime"]["version"].clone();
            core_version = ellie_lang_toml["dependencies"]["ellie_core"]["version"].clone();
        }
        Err(_) => {
            panic!(
                "Failed to build ellie constants, cannot read {}Cargo.toml{}",
                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                terminal_colors::get_color(terminal_colors::Colors::Reset),
            )
        }
    }

    match read_file(&("./lib/ellie.ei".to_owned())) {
        Ok(ellie_lib) => {
            match read_file(&("./core/src/builded_libraries.rs".to_owned())) {
                Ok(current_lib) => {
                    //@version *=\\s*\"\\^|\\~?(\\d|x|\\*)+\\.(\\d|x|\\*)+\\.(\\d|x|\\*)
                    let version_line_regex = Regex::new(
                        "(@(\\s)*version(\\s)*=)(\\s)*(\")*(?P<version>\"\\^|\\~?(\\d|x|\\*)+\\.(\\d|x|\\*)+\\.(\\d|x|\\*))*(\"|()*;)",
                    )
                    .unwrap();

                    if version_line_regex.is_match(&ellie_lib.clone())
                        && version_line_regex.is_match(&current_lib.clone())
                    {
                        let lib_version_number = &version_line_regex.captures(&ellie_lib).unwrap();
                        let current_version_number =
                            &version_line_regex.captures(&current_lib).unwrap();

                        if lib_version_number.name("version").is_some()
                            && current_version_number.name("version").is_some()
                        {
                            let lib_version = &lib_version_number["version"];
                            let current_version = &current_version_number["version"];

                            let mut ellie_std_types_version_hasher = DefaultHasher::new();
                            ellie_lib.hash(&mut ellie_std_types_version_hasher);

                            let version_hash: u64 = vec![
                                ellie_version.as_str().unwrap(),
                                parser_version.as_str().unwrap(),
                                runtime_version.as_str().unwrap(),
                                core_version.as_str().unwrap(),
                                lib_version,
                            ]
                            .join(".")
                            .split(".")
                            .fold(0, |acc, x| acc + x.parse::<u64>().unwrap());

                            fs::write(
                                "./src/cli_constants.rs",
                                format!(
                                    "pub static ELLIE_VERSION: &'static str = &{};\npub static ELLIE_VERSION_NAME: &'static str = &{};\npub static ELLIE_PARSER_VERSION: &'static str = &{};\npub static ELLIE_RUNTIME_VERSION: &'static str = &{};\npub static ELLIE_CORE_VERSION: &'static str = &{};\npub static ELLIE_STD_VERSION: &'static str = &\"{}\";\npub static ELLIE_COMPATIBILITY_HASH: u64 = {:#04x};",
                                    ellie_version,
                                    ellie_version_name,
                                    parser_version,
                                    runtime_version,
                                    core_version,
                                    lib_version,
                                    version_hash + ellie_std_types_version_hasher.finish()
                                ),
                            )
                            .unwrap();

                            if lib_version == current_version && rebuild_std {
                                eprintln!(
                                    "\nCompiling Ellie standard library {}v{}{} is not required",
                                    terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                    lib_version,
                                    terminal_colors::get_color(terminal_colors::Colors::Reset),
                                );
                            } else {
                                let ellie_lib = parse(ellie_lib.clone(), "ellie".to_owned())
                                    .parsed
                                    .to_definite();

                                eprintln!(
                                    "\nCompiling Ellie standard library {}v{}{} complete",
                                    terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                    lib_version,
                                    terminal_colors::get_color(terminal_colors::Colors::Reset),
                                );
                                let j = serde_json::to_string(&ellie_lib).unwrap();
                                fs::write(
                                    "./core/src/builded_libraries.rs",
                                    format!("//@version = \"{}\";\npub static ELLIE_STANDARD_LIBRARY : &str = {:#?};", lib_version, j),
                                )
                                .unwrap();
                            }
                        } else if lib_version_number.name("version").is_some() {
                            panic!(
                                "\nCompiling Ellie Failed, No version key found in {}~./core/src/builded_libraries.rs{}",
                                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                terminal_colors::get_color(terminal_colors::Colors::Reset),
                            );
                        } else {
                            panic!(
                                "\nCompiling Ellie Failed, No version key found in {}~./lib/ellie.ei{}",
                                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                terminal_colors::get_color(terminal_colors::Colors::Reset),
                            );
                        }
                    } else {
                        if version_line_regex.is_match(&ellie_lib.clone()) {
                            panic!(
                                "\nCompiling Ellie Failed, No version key found in {}~./core/src/builded_libraries.rs{}",
                                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                terminal_colors::get_color(terminal_colors::Colors::Reset),
                            );
                        } else {
                            panic!(
                                "\nCompiling Ellie Failed, No version key found in {}~./lib/ellie.ei{}",
                                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                                terminal_colors::get_color(terminal_colors::Colors::Reset),
                            );
                        }
                    }
                }
                Err(err) => {
                    panic!(
                        "{}[Fail]{}: Cannot read file {}~./core/{}.rs{}\n{:#?}",
                        terminal_colors::get_color(terminal_colors::Colors::Red),
                        terminal_colors::get_color(terminal_colors::Colors::Reset),
                        terminal_colors::get_color(terminal_colors::Colors::Yellow),
                        "builded_libraries",
                        terminal_colors::get_color(terminal_colors::Colors::Reset),
                        err
                    );
                }
            }
        }
        Err(err) => {
            panic!(
                "{}[Fail]{}: Cannot read file {}~./lib/{}.ei{}\n{:#?}",
                terminal_colors::get_color(terminal_colors::Colors::Red),
                terminal_colors::get_color(terminal_colors::Colors::Reset),
                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                "ellie",
                terminal_colors::get_color(terminal_colors::Colors::Reset),
                err
            );
        }
    }
}
