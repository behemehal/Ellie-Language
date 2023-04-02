#[macro_export]
macro_rules! tokenizeCode {
    ($input:expr) => {
        struct Repository {
            target_path: String,
            tokenizer_settings: TokenizerSettings,
        }

        let mut program_repository = Repository {
            target_path: target_path.to_str().unwrap().to_string(),
            tokenizer_settings: tokenizer_settings.clone(),
        };

        impl ProgramRepository for Repository {
            fn read_main(&mut self) -> MainProgram {
                match read_file(self.target_path.clone()) {
                    Ok(main_file_content) => {
                        let mut main_file_hasher = DefaultHasher::new();
                        main_file_content.hash(&mut main_file_hasher);
                        let first_page_hash = main_file_hasher.finish();
                        MainProgram {
                            file_content: main_file_content,
                            file_name: self.tokenizer_settings.file_name.clone(),
                            file_hash: first_page_hash as usize,
                            start_directory: format!(
                                "<ellie_module_{}>",
                                self.tokenizer_settings.name
                            ),
                        }
                    }
                    Err(err) => {
                        if self.tokenizer_settings.json_log {
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
                let starter_name = format!("<ellie_module_{}>", self.tokenizer_settings.name);

                if link_module {
                    ResolvedImport {
                        found: false,
                        resolve_error: "Cannot use modules in tokenizer".to_owned(),
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
                                match utils::read_file(real_path) {
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
    };
}
