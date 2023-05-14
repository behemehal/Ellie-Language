use crate::OutputTypesSelector;
use bincode::Options;
use ellie_engine::{
    compiler::parse_pages,
    ellie_bytecode::assembler::{Assembler, PlatformAttributes},
    ellie_core::{
        defs::{ModuleMap, PlatformArchitecture},
        module_path::parse_module_import,
    },
    ellie_parser::parser,
    //ellie_renderer_utils::*,
    ellie_renderer_utils::outputs,
    ellie_renderer_utils::utils::{
        print_errors, print_warnings, read_file, CliColor, ColorDisplay, Colors,
    },
    ellie_tokenizer::tokenizer::ImportType,
    ellie_tokenizer::tokenizer::ResolvedImport,
    tokenizer,
    utils::{CompilerSettings, MainProgram, ProgramRepository},
};
use path_absolutize::Absolutize;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[derive(Clone)]
pub struct CliCompilerSettings {
    pub json_log: bool,
    pub output_type: OutputTypesSelector,
    pub warnings: bool,
    pub performance_info: bool,
    pub show_debug_lines: bool,
    pub exclude_std: bool,
    pub compiler_settings: CompilerSettings,
}

pub fn get_output_path(
    target_path: &Path,
    output_path: &Path,
    output_type: OutputTypesSelector,
) -> PathBuf {
    if output_path.is_dir() {
        let path = output_path
            .absolutize()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let mut file_name = target_path.file_name().unwrap().to_str().unwrap();

        if file_name.contains(".") {
            file_name = file_name.split(".").nth(0).unwrap();
        }

        Path::new(
            &(path
                + "/"
                + file_name
                + match output_type {
                    OutputTypesSelector::Bin => ".eib",
                    OutputTypesSelector::ByteCode => ".eic",
                    OutputTypesSelector::ByteCodeAsm => ".eia",
                    OutputTypesSelector::ByteCodeDebug => ".eig",
                    _ => ".json",
                }),
        )
        .to_owned()
    } else {
        output_path.to_owned()
    }
}

pub fn compile(
    target_path: &Path,
    output_path: &Path,
    modules: Vec<(parser::Module, Option<String>)>,
    cli_settings: CliCompilerSettings,
) {
    let cli_color = &CliColor;
    let exit_messages: Mutex<Vec<Box<dyn Fn()>>> = Mutex::new(vec![Box::new(|| {
        println!(
            "{}[?]{}: Ellie v{}",
            cli_color.color(Colors::Green),
            cli_color.color(Colors::Reset),
            crate::engine_constants::ELLIE_ENGINE_VERSION
        );
    })]);

    if cli_settings.compiler_settings.experimental_features {
        println!(
            "{}[!]{}: Experimental features are enabled.",
            cli_color.color(Colors::Red),
            cli_color.color(Colors::Reset),
        );
    }

    #[derive(Clone)]
    struct Repository {
        used_modules: Vec<String>,
        main_hash: usize,
        target_path: String,
        cli_compiler_settings: CliCompilerSettings,
    }

    let mut _used_modules = vec![];

    if !cli_settings.exclude_std {
        _used_modules.push("ellieStd".to_string());
    }

    let mut program_repisotory = Repository {
        main_hash: 0,
        used_modules: _used_modules,
        target_path: target_path.to_str().unwrap().to_string(),
        cli_compiler_settings: cli_settings.clone(),
    };

    impl ProgramRepository for Repository {
        fn read_main(&mut self) -> MainProgram {
            match read_file(self.target_path.clone()) {
                Ok(main_file_content) => {
                    let mut main_file_hasher = DefaultHasher::new();
                    main_file_content.hash(&mut main_file_hasher);
                    let first_page_hash = main_file_hasher.finish();
                    self.main_hash = first_page_hash as usize;
                    MainProgram {
                        file_content: main_file_content,
                        file_name: self
                            .cli_compiler_settings
                            .compiler_settings
                            .file_name
                            .clone(),
                        file_hash: first_page_hash as usize,
                        start_directory: format!(
                            "<ellie_module_{}>",
                            self.cli_compiler_settings.compiler_settings.name
                        ),
                    }
                }
                Err(err) => {
                    if self.cli_compiler_settings.json_log {
                        let mut cli_module_output = outputs::READ_FILE_ERROR.clone();
                        cli_module_output
                            .extra
                            .push(outputs::CliOuputExtraData { key: 0, value: err });
                        cli_module_output.extra.push(outputs::CliOuputExtraData {
                            key: 1,
                            value: self.target_path.clone(),
                        });
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&cli_module_output).unwrap()
                        );
                    } else {
                        println!("Unable to read file ~{} [{}]", self.target_path, err);
                    }
                    std::process::exit(1);
                }
            }
        }

        fn read_module(
            &mut self,
            link_module: bool,
            current_path: String,
            requested_path: String,
        ) -> ResolvedImport {
            let starter_name = format!(
                "<ellie_module_{}>",
                self.cli_compiler_settings.compiler_settings.name
            );

            if link_module {
                self.used_modules.push(requested_path);
                ResolvedImport {
                    found: true,
                    ..Default::default()
                }
            } else {
                match parse_module_import(&current_path, &requested_path) {
                    Ok(path) => {
                        let real_path = path
                            .replace(
                                &starter_name,
                                Path::new(&self.target_path)
                                    .absolutize()
                                    .unwrap()
                                    .parent()
                                    .unwrap()
                                    .to_str()
                                    .unwrap(),
                            )
                            .clone();
                        if Path::new(&real_path).exists() {
                            match read_file(real_path) {
                                Ok(data) => {
                                    let mut hasher = DefaultHasher::new();
                                    data.hash(&mut hasher);
                                    ResolvedImport {
                                        found: true,
                                        matched: ImportType::Code(data),
                                        hash: hasher.finish().try_into().unwrap(),
                                        path,
                                        ..Default::default()
                                    }
                                }
                                Err(_) => ResolvedImport {
                                    found: false,
                                    resolve_error: "Cannot find file".to_string(),
                                    ..Default::default()
                                },
                            }
                        } else {
                            ResolvedImport {
                                found: false,
                                resolve_error: "Path does not exist".to_string(),
                                ..Default::default()
                            }
                        }
                    }
                    Err(e) => {
                        if e == 1 {
                            ResolvedImport {
                                found: false,
                                resolve_error: "Cannot access outside of workspace".to_string(),
                                ..Default::default()
                            }
                        } else {
                            unreachable!()
                        }
                    }
                }
            }
        }
    }

    let mut used_modules = Vec::new();
    for module_name in &program_repisotory.used_modules {
        if let Some(module) = modules
            .iter()
            .find(|(module, _)| module.name == *module_name)
        {
            used_modules.push(module.clone());
        } else {
            if program_repisotory.cli_compiler_settings.json_log {
                let mut cli_module_output = outputs::FAILED_TO_FIND_MODULE.clone();
                cli_module_output.extra.push(outputs::CliOuputExtraData {
                    key: 0,
                    value: module_name.to_string(),
                });
                println!(
                    "{}",
                    serde_json::to_string_pretty(&cli_module_output).unwrap()
                );
            } else {
                println!(
                    "{}[Internal Error]{}: Could not find imported module {}'{}'{}",
                    cli_color.color(Colors::Red),
                    cli_color.color(Colors::Reset),
                    cli_color.color(Colors::Cyan),
                    module_name,
                    cli_color.color(Colors::Reset),
                );
            }
            std::process::exit(1);
        }
    }
    let starter_name = format!("<ellie_module_{}>", cli_settings.compiler_settings.name);

    match tokenizer::tokenize_file(&mut program_repisotory) {
        Ok(pages) => {
            match parse_pages(
                program_repisotory.main_hash,
                used_modules,
                pages,
                cli_settings.compiler_settings.clone(),
            ) {
                Ok(compile_output) => {
                    if !compile_output.warnings.is_empty() {
                        if cli_settings.json_log {
                            let mut output = outputs::COMPILER_WARNINGS.clone();
                            output.extra.push(outputs::CliOuputExtraData {
                                key: "warnings".to_string(),
                                value: compile_output.warnings.clone(),
                            });
                            println!("{}", serde_json::to_string(&output).unwrap());
                        } else {
                            println!(
                                "{}",
                                print_warnings(
                                    &compile_output.warnings,
                                    |path| {
                                        let path_starter = path.split("/").next().unwrap();
                                        let virtual_path_identifier =
                                            match path_starter.split("<ellie_module_").last() {
                                                Some(e) => e.split(">").next().unwrap(),
                                                None => "",
                                            };
                                        if path_starter == starter_name {
                                            let real_path = path
                                                .replace(
                                                    &starter_name,
                                                    Path::new(target_path)
                                                        .absolutize()
                                                        .unwrap()
                                                        .parent()
                                                        .unwrap()
                                                        .to_str()
                                                        .unwrap(),
                                                )
                                                .clone();
                                            match read_file(real_path) {
                                                Ok(e) => e,
                                                Err(err) => {
                                                    println!(
                                                    "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                                    path,
                                                    cli_color.color(Colors::Red),
                                                    err,
                                                    cli_color.color(Colors::Reset)
                                                );
                                                    std::process::exit(1);
                                                }
                                            }
                                        } else if let Some((_, module_path)) =
                                            modules.iter().find(|(module, _)| {
                                                module.name == virtual_path_identifier
                                            })
                                        {
                                            let module_path = module_path.clone().unwrap();
                                            let real_path =
                                                path.replace(&path_starter, &module_path).clone();
                                            match read_file(real_path) {
                                                Ok(e) => e,
                                                Err(err) => {
                                                    println!(
                                                    "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                                    path,
                                                    cli_color.color(Colors::Red),
                                                    err,
                                                    cli_color.color(Colors::Reset)
                                                );
                                                    std::process::exit(1);
                                                }
                                            }
                                        } else {
                                            panic!(
                                            "Failed to ouput error. Cannot identify module '{}'",
                                            path,
                                        );
                                        }
                                    },
                                    |path| {
                                        let path_starter = path.split("/").next().unwrap();
                                        let virtual_path_identifier =
                                            match path_starter.split("<ellie_module_").last() {
                                                Some(e) => e.split(">").next().unwrap(),
                                                None => "",
                                            };
                                        if path_starter == starter_name {
                                            path.replace(
                                                &starter_name,
                                                Path::new(target_path)
                                                    .absolutize()
                                                    .unwrap()
                                                    .parent()
                                                    .unwrap()
                                                    .to_str()
                                                    .unwrap(),
                                            )
                                            .clone()
                                        } else if let Some((_, module_path)) =
                                            modules.iter().find(|(module, _)| {
                                                module.name == virtual_path_identifier
                                            })
                                        {
                                            let module_path = module_path.clone().unwrap();
                                            path.replace(&path_starter, &module_path).clone()
                                        } else {
                                            panic!(
                                            "Failed to ouput error. Cannot identify module '{}'",
                                            path,
                                        );
                                        }
                                    },
                                    cli_color.clone(),
                                )
                            );
                        }
                    }

                    let output_path = &get_output_path(
                        target_path,
                        output_path,
                        cli_settings.output_type.clone(),
                    );

                    let dbg_output_path = output_path.file_name().unwrap().to_str().unwrap();

                    let dbg_output_path = &get_output_path(
                        target_path,
                        Path::new(&output_path.to_str().unwrap().replace(dbg_output_path, "")),
                        OutputTypesSelector::ByteCodeDebug,
                    );

                    let mut module_maps = vec![ModuleMap {
                        module_name: compile_output.module.name.clone(),
                        module_path: Some(
                            Path::new(target_path)
                                .absolutize()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string(),
                        ),
                    }];

                    module_maps.extend(
                        modules
                            .iter()
                            .map(|(module, path)| ModuleMap {
                                module_name: module.name.clone(),
                                module_path: path.clone(),
                            })
                            .collect::<Vec<_>>(),
                    );

                    match cli_settings.output_type {
                        OutputTypesSelector::Bin => {
                            let config = bincode::options().clone();

                            let bytes = config.serialize(&compile_output.module).unwrap();
                            if let Err(write_error) = fs::write(output_path, bytes) {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_FILE_ERROR.clone();

                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: "path".to_string(),
                                        value: format!("{:?}", write_error),
                                    });
                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "\nFailed to write output. [{}{:?}{}]",
                                        cli_color.color(Colors::Red),
                                        write_error,
                                        cli_color.color(Colors::Reset),
                                    );
                                }
                            } else {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_BINARY_SUCCEDED.clone();
                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: 0,
                                        value: output_path
                                            .absolutize()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_owned(),
                                    });
                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "{}[!]{}: Binary output written to {}{}{}",
                                        cli_color.color(Colors::Green),
                                        cli_color.color(Colors::Reset),
                                        cli_color.color(Colors::Yellow),
                                        output_path.absolutize().unwrap().to_str().unwrap(),
                                        cli_color.color(Colors::Reset)
                                    );
                                }
                            }
                        }
                        OutputTypesSelector::DependencyAnalysis => {
                            println!(
                                "{}[Error]{}: Dependency analysis output is not supported yet.",
                                cli_color.color(Colors::Red),
                                cli_color.color(Colors::Reset),
                            );
                            std::process::exit(1);
                        }
                        OutputTypesSelector::Json => {
                            let json = serde_json::to_string(&compile_output.module).unwrap();
                            if let Err(write_error) = fs::write(&output_path, json) {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_FILE_ERROR.clone();
                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: "path".to_string(),
                                        value: format!("{:?}", write_error),
                                    });

                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "\nFailed to write output. [{}{:?}{}]",
                                        cli_color.color(Colors::Red),
                                        write_error,
                                        cli_color.color(Colors::Reset),
                                    );
                                }
                            } else {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_JSON_SUCCEDED.clone();
                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: 0,
                                        value: output_path
                                            .absolutize()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_owned(),
                                    });
                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "{}[!]{}: JSON output written to {}{}{}",
                                        cli_color.color(Colors::Green),
                                        cli_color.color(Colors::Reset),
                                        cli_color.color(Colors::Yellow),
                                        output_path.absolutize().unwrap().to_str().unwrap(),
                                        cli_color.color(Colors::Reset),
                                    );
                                }
                            }
                        }
                        OutputTypesSelector::ByteCode => {
                            if !cli_settings.json_log {
                                println!(
                                    "{}[?]{}: ByteCode compiling to {} bit architecture",
                                    cli_color.color(Colors::Green),
                                    cli_color.color(Colors::Reset),
                                    match cli_settings.compiler_settings.byte_code_architecture {
                                        PlatformArchitecture::B16 => "16",
                                        PlatformArchitecture::B32 => "32",
                                        PlatformArchitecture::B64 => "64",
                                    }
                                );
                            }
                            let mut assembler = Assembler::new(
                                compile_output.module,
                                PlatformAttributes {
                                    architecture: PlatformArchitecture::B64, //64 Bit Limit
                                    memory_size: 512000,                     //512kb memory limit
                                },
                            );
                            let assembler_result = assembler.assemble(module_maps);
                            let mut output_file = File::create(output_path).unwrap_or_else(|err| {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_FILE_ERROR.clone();
                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: "path".to_string(),
                                        value: format!("{:?}", err),
                                    });
                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "\nFailed to create file {}{}{}. [{}{:?}{}]",
                                        cli_color.color(Colors::Cyan),
                                        output_path.absolutize().unwrap().to_str().unwrap(),
                                        cli_color.color(Colors::Reset),
                                        cli_color.color(Colors::Red),
                                        err,
                                        cli_color.color(Colors::Reset),
                                    );
                                }
                                std::process::exit(1);
                            });
                            let mut dbg_file =
                                File::create(dbg_output_path).unwrap_or_else(|err| {
                                    if cli_settings.json_log {
                                        let mut output = outputs::WRITE_FILE_ERROR.clone();
                                        output.extra.push(outputs::CliOuputExtraData {
                                            key: "path".to_string(),
                                            value: format!("{:?}", err),
                                        });
                                        println!("{}", serde_json::to_string(&output).unwrap())
                                    } else {
                                        println!(
                                            "\nFailed to create file {}{}{}. [{}{:?}{}]",
                                            cli_color.color(Colors::Cyan),
                                            dbg_output_path.absolutize().unwrap().to_str().unwrap(),
                                            cli_color.color(Colors::Reset),
                                            cli_color.color(Colors::Red),
                                            err,
                                            cli_color.color(Colors::Reset),
                                        );
                                    }
                                    std::process::exit(1);
                                });
                            assembler_result.render_binary(&mut output_file, &mut dbg_file);
                            if cli_settings.json_log {
                                let mut output = outputs::WRITE_BYTE_CODE_SUCCEDED.clone();
                                output.extra.push(outputs::CliOuputExtraData {
                                    key: 0,
                                    value: output_path
                                        .absolutize()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_owned(),
                                });
                                println!("{}", serde_json::to_string(&output).unwrap())
                            } else {
                                println!(
                                    "{}[!]{}: ByteCode output written to {}{}{}",
                                    cli_color.color(Colors::Green),
                                    cli_color.color(Colors::Reset),
                                    cli_color.color(Colors::Yellow),
                                    output_path.absolutize().unwrap().to_str().unwrap(),
                                    cli_color.color(Colors::Reset),
                                );
                                println!(
                                    "{}[!]{}: ByteCode debug file written to {}{}{}",
                                    cli_color.color(Colors::Green),
                                    cli_color.color(Colors::Reset),
                                    cli_color.color(Colors::Yellow),
                                    dbg_output_path.absolutize().unwrap().to_str().unwrap(),
                                    cli_color.color(Colors::Reset),
                                );
                            }
                        }
                        OutputTypesSelector::ByteCodeAsm => {
                            if !cli_settings.json_log {
                                println!(
                                    "{}[?]{}: ByteCode compiling to {} bit architecture",
                                    cli_color.color(Colors::Green),
                                    cli_color.color(Colors::Reset),
                                    match cli_settings.compiler_settings.byte_code_architecture {
                                        PlatformArchitecture::B16 => "16",
                                        PlatformArchitecture::B32 => "32",
                                        PlatformArchitecture::B64 => "64",
                                    }
                                );
                            }
                            let mut assembler = Assembler::new(
                                compile_output.module,
                                PlatformAttributes {
                                    architecture: PlatformArchitecture::B64, //64 Bit Limit
                                    memory_size: 512000,                     //512kb memory limit
                                },
                            );
                            let assembler_result = assembler.assemble(module_maps);
                            let mut output_file = File::create(output_path).unwrap_or_else(|err| {
                                if cli_settings.json_log {
                                    let mut output = outputs::WRITE_FILE_ERROR.clone();
                                    output.extra.push(outputs::CliOuputExtraData {
                                        key: "path".to_string(),
                                        value: format!("{:?}", err),
                                    });
                                    println!("{}", serde_json::to_string(&output).unwrap())
                                } else {
                                    println!(
                                        "\nFailed to create file {}{}{}. [{}{:?}{}]",
                                        cli_color.color(Colors::Cyan),
                                        output_path.absolutize().unwrap().to_str().unwrap(),
                                        cli_color.color(Colors::Reset),
                                        cli_color.color(Colors::Red),
                                        err,
                                        cli_color.color(Colors::Reset),
                                    );
                                }
                                std::process::exit(1);
                            });
                            assembler_result.alternate_render(&mut output_file);

                            if cli_settings.json_log {
                                let mut output = outputs::WRITE_BYTE_CODE_ASM_SUCCEDED.clone();
                                output.extra.push(outputs::CliOuputExtraData {
                                    key: 0,
                                    value: output_path
                                        .absolutize()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_owned(),
                                });
                                println!("{}", serde_json::to_string(&output).unwrap())
                            } else {
                                println!(
                                    "{}[!]{}: ByteCodeAsm output written to {}{}{}",
                                    cli_color.color(Colors::Green),
                                    cli_color.color(Colors::Reset),
                                    cli_color.color(Colors::Yellow),
                                    output_path.absolutize().unwrap().to_str().unwrap(),
                                    cli_color.color(Colors::Reset),
                                );
                            }
                        }
                        OutputTypesSelector::ByteCodeDebug => unreachable!(),
                        OutputTypesSelector::Nop => (),
                    }
                    if !cli_settings.json_log {
                        for message in exit_messages.lock().unwrap().iter() {
                            (message)();
                        }
                    }
                }
                Err(errors) => {
                    if cli_settings.json_log {
                        let mut output = outputs::COMPILER_ERRORS.clone();
                        output.extra.push(outputs::CliOuputExtraData {
                            key: "errors".to_string(),
                            value: errors,
                        });
                        println!("{}", serde_json::to_string(&output).unwrap());
                    } else {
                        println!(
                            "{}",
                            print_errors(
                                &errors,
                                |path| {
                                    let path_starter = path.split("/").next().unwrap();
                                    let virtual_path_identifier =
                                        match path_starter.split("<ellie_module_").last() {
                                            Some(e) => e.split(">").next().unwrap(),
                                            None => "",
                                        };
                                    if path_starter == starter_name {
                                        let real_path = path
                                            .replace(
                                                &starter_name,
                                                Path::new(target_path)
                                                    .absolutize()
                                                    .unwrap()
                                                    .parent()
                                                    .unwrap()
                                                    .to_str()
                                                    .unwrap(),
                                            )
                                            .clone();
                                        match read_file(real_path) {
                                            Ok(e) => e,
                                            Err(err) => {
                                                panic!(
                                            "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                            path,
                                            cli_color.color(Colors::Red),
                                            err,
                                            cli_color.color(Colors::Reset)
                                        );
                                            }
                                        }
                                    } else if let Some((_, module_path)) = modules
                                        .iter()
                                        .find(|(module, _)| module.name == virtual_path_identifier)
                                    {
                                        if let Some(module_path) = module_path.clone() {
                                            let real_path =
                                                path.replace(&path_starter, &module_path).clone();
                                            match read_file(real_path.clone()) {
                                                Ok(e) => e,
                                                Err(err) => {
                                                    exit_messages.lock().unwrap().push(Box::new(move || {
                                                    println!(
                                                        "{}[!]{}: Failed to read module targeted code director y: {}{}{} - [{}]",
                                                        cli_color.color(Colors::Red),
                                                        cli_color.color(Colors::Reset),
                                                        cli_color.color(Colors::Yellow),
                                                        real_path.clone(),
                                                        cli_color.color(Colors::Reset),
                                                        err,
                                                    );
                                                }));
                                                    module_path
                                                }
                                            }
                                        } else {
                                            exit_messages.lock().unwrap().push(Box::new(move || {
                                                println!(
                                                    "{}[!]{}: Path not provided no output will be shown: {}{}{}",
                                                    cli_color.color(Colors::Red),
                                                    cli_color.color(Colors::Reset),
                                                    cli_color.color(Colors::Yellow),
                                                    path,
                                                    cli_color.color(Colors::Reset),
                                                );
                                            }));
                                            "No output path provided".to_string()
                                        }
                                    } else {
                                        panic!(
                                            "Failed to ouput error. Cannot identify module '{}'",
                                            path,
                                        );
                                    }
                                },
                                cli_settings.show_debug_lines,
                                |path| {
                                    let path_starter = path.split("/").next().unwrap();
                                    let virtual_path_identifier =
                                        match path_starter.split("<ellie_module_").last() {
                                            Some(e) => e.split(">").next().unwrap(),
                                            None => "",
                                        };
                                    if path_starter == starter_name {
                                        path.replace(
                                            &starter_name,
                                            Path::new(target_path)
                                                .absolutize()
                                                .unwrap()
                                                .parent()
                                                .unwrap()
                                                .to_str()
                                                .unwrap(),
                                        )
                                        .clone()
                                    } else if let Some((_, module_path)) = modules
                                        .iter()
                                        .find(|(module, _)| module.name == virtual_path_identifier)
                                    {
                                        if let Some(module_path) = module_path.clone() {
                                            path.replace(&path_starter, &module_path).clone()
                                        } else {
                                            exit_messages.lock().unwrap().push(Box::new(move || {
                                                println!(
                                                    "{}[!]{}: Path not provided no output will be shown: {}{}{}",
                                                    cli_color.color(Colors::Red),
                                                    cli_color.color(Colors::Reset),
                                                    cli_color.color(Colors::Yellow),
                                                    path,
                                                    cli_color.color(Colors::Reset),
                                                );
                                            }));
                                            "[No output path provided]".to_string()
                                        }
                                    } else {
                                        panic!(
                                            "Failed to ouput error. Cannot identify module '{}'",
                                            path,
                                        );
                                    }
                                },
                                cli_color.clone(),
                            )
                        );
                        for message in exit_messages.lock().unwrap().iter() {
                            (message)();
                        }
                    }
                }
            }
        }
        Err(pager_errors) => {
            if cli_settings.json_log {
                let mut output = outputs::COMPILER_ERRORS.clone();
                output.extra.push(outputs::CliOuputExtraData {
                    key: "errors".to_string(),
                    value: pager_errors,
                });
                println!("{}", serde_json::to_string(&output).unwrap());
            } else {
                println!(
                    "{}",
                    print_errors(
                        &pager_errors,
                        |path| match read_file(
                            &path.replace(
                                &starter_name,
                                Path::new(target_path)
                                    .absolutize()
                                    .unwrap()
                                    .parent()
                                    .unwrap()
                                    .to_str()
                                    .unwrap(),
                            ),
                        ) {
                            Ok(e) => e,
                            Err(err) => {
                                println!(
                                    "{}[Internal Error]{} Cannot build error, read file failed '{}' {}[{}]{}",
                                    cli_color.color(Colors::Red),
                                    cli_color.color(Colors::Reset),
                                    path,
                                    cli_color.color(Colors::Red),
                                    err,
                                    cli_color.color(Colors::Reset),
                                );
                                std::process::exit(1);
                            }
                        },
                        cli_settings.show_debug_lines,
                        |path| {
                            path.replace(
                                &starter_name,
                                Path::new(target_path)
                                    .absolutize()
                                    .unwrap()
                                    .parent()
                                    .unwrap()
                                    .to_str()
                                    .unwrap(),
                            )
                            .to_string()
                        },
                        cli_color.clone()
                    )
                );
                for message in exit_messages.lock().unwrap().iter() {
                    (message)();
                }
            }
        }
    }
}
