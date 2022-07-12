#![allow(dead_code)]
#![allow(unused)]
use crate::cli_outputs;
use crate::cli_utils;
use crate::cli_utils::generate_blank;
use crate::cli_utils::Colors;
use ellie_core::definite::Converter;
use ellie_tokenizer::processors::items::Processors;
use ellie_tokenizer::tokenizer::{self, ResolvedImport};
use path_absolutize::Absolutize;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

pub struct ColorizerSettings {
    pub name: String,
    pub file_name: String,
    pub show_debug_lines: bool,
}

pub fn get_output_path(
    target_path: &Path,
    output_path: &Path,
    output_type: cli_utils::OutputTypes,
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
                    cli_utils::OutputTypes::Bin => ".bin",
                    _ => ".json",
                }),
        )
        .to_owned()
    } else {
        output_path.to_owned()
    }
}

pub fn color_code(target_path: &Path, output_path: &Path, colorizer_settings: ColorizerSettings) {
    let starter_name = format!("<ellie_module_{}>", colorizer_settings.name);
    match cli_utils::read_file(target_path) {
        Ok(main_file_content) => {
            let mut main_file_hasher = DefaultHasher::new();
            main_file_content.hash(&mut main_file_hasher);
            let first_page_hash: usize = main_file_hasher.finish().try_into().unwrap();
            let mut pager = tokenizer::Pager::new(
                main_file_content,
                colorizer_settings.file_name,
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
                                        resolve_error: "Path is not exists".to_string(),
                                        ..Default::default()
                                    }
                                }
                            }
                            Err(e) => {
                                if e == 1 {
                                    ResolvedImport {
                                        found: false,
                                        resolve_error: "Cannot access outside of workspace"
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

            let tokenize_start = Instant::now();
            match pager.run() {
                Ok(_) => {
                    let tokenize_end =
                        (tokenize_start.elapsed().as_nanos() as f64 / 1000000_f64) as f64;

                    //Empty char ·
                    //Tab        |→
                    //╞
                    //    3 │
                    //    ├──

                    fn type_liner(item: ellie_tokenizer::processors::types::Processors) -> String {
                        match item {
                            ellie_tokenizer::processors::types::Processors::Integer(e) => {
                                format!(
                                    "{}{}{}",
                                    Colors::Cyan,
                                    e.data.value.to_string(),
                                    Colors::Reset
                                )
                            }
                            ellie_tokenizer::processors::types::Processors::Byte(e) => {
                                format!("{}{}{}", Colors::Cyan, e.value.to_string(), Colors::Reset)
                            }
                            ellie_tokenizer::processors::types::Processors::Float(e) => {
                                format!(
                                    "{}{}{}",
                                    Colors::Cyan,
                                    e.data.value.to_string(),
                                    Colors::Reset
                                )
                            }
                            ellie_tokenizer::processors::types::Processors::Char(e) => {
                                format!(
                                    "{}'{}'{}",
                                    Colors::Cyan,
                                    e.value.to_string(),
                                    Colors::Reset
                                )
                            }
                            ellie_tokenizer::processors::types::Processors::String(e) => {
                                format!(
                                    "{}\"{}\"{}",
                                    Colors::Cyan,
                                    e.data.value.to_string(),
                                    Colors::Reset
                                )
                            }
                            ellie_tokenizer::processors::types::Processors::Variable(e) => {
                                format!(
                                    "{}{}{}",
                                    Colors::Cyan,
                                    e.data.value.to_string(),
                                    Colors::Reset
                                )
                            }
                            ellie_tokenizer::processors::types::Processors::Negative(e) => {
                                format!("!{}", type_liner(*e.value))
                            }
                            ellie_tokenizer::processors::types::Processors::Array(_) => todo!(),
                            ellie_tokenizer::processors::types::Processors::Operator(_) => todo!(),
                            ellie_tokenizer::processors::types::Processors::Reference(_) => todo!(),
                            ellie_tokenizer::processors::types::Processors::BraceReference(_) => {
                                todo!()
                            }
                            ellie_tokenizer::processors::types::Processors::NullResolver(_) => {
                                todo!()
                            }
                            ellie_tokenizer::processors::types::Processors::FunctionCall(_) => {
                                todo!()
                            }
                            ellie_tokenizer::processors::types::Processors::ClassCall(_) => todo!(),
                            ellie_tokenizer::processors::types::Processors::Cloak(_) => todo!(),
                            ellie_tokenizer::processors::types::Processors::Collective(_) => {
                                todo!()
                            }
                            ellie_tokenizer::processors::types::Processors::AsKeyword(_) => todo!(),
                            ellie_tokenizer::processors::types::Processors::EnumData(_) => todo!(),
                        }
                    }

                    fn liner(item: Processors) -> Vec<String> {
                        //{
                        let brace_open = format!("{}{}{}", Colors::Magenta, "{", Colors::Reset);
                        //}
                        let brace_close = format!("{}{}{}", Colors::Magenta, "}", Colors::Reset);
                        //,
                        let comma = format!("{}{}{}", Colors::Magenta, ",", Colors::Reset);
                        //:
                        let colon = format!("{}{}{}", Colors::Magenta, ";", Colors::Reset);
                        //(
                        let bracket_open = format!("{}{}{}", Colors::Magenta, "(", Colors::Reset);
                        //)
                        let bracket_close = format!("{}{}{}", Colors::Magenta, ")", Colors::Reset);
                        //:
                        let two_dot = format!("{}{}{}", Colors::Magenta, ":", Colors::Reset);

                        //fn
                        let fn_keyword = format!("{}{}{}", Colors::Red, "fn", Colors::Reset);

                        let mut lines = Vec::new();
                        match item {
                            Processors::Variable(_) => todo!(),
                            Processors::GetterCall(_) => todo!(),
                            Processors::SetterCall(_) => todo!(),
                            Processors::Getter(_) => todo!(),
                            Processors::Setter(_) => todo!(),
                            Processors::Function(f) => {
                                lines.push(format!(
                                    "{} {}fn{} {}{}{}{bracket_open}{}{bracket_close}{}",
                                    if f.data.public {
                                        format!("{}pub{}", Colors::Red, Colors::Reset)
                                    } else {
                                        format!("{}pri{}", Colors::Red, Colors::Reset)
                                    },
                                    Colors::Red,
                                    Colors::Reset,
                                    Colors::Cyan,
                                    f.data.name,
                                    Colors::Reset,
                                    f.data
                                        .parameters
                                        .iter()
                                        .map(|param| {
                                            format!(
                                                "{}{}{} {} {}{}{}",
                                                Colors::Cyan,
                                                param.name,
                                                Colors::Reset,
                                                two_dot,
                                                Colors::Blue,
                                                param
                                                    .rtype
                                                    .definer_type
                                                    .clone()
                                                    .to_definite()
                                                    .to_string(),
                                                Colors::Reset,
                                            )
                                        })
                                        .collect::<Vec<String>>()
                                        .join(&format!("{comma} ")),
                                    if f.data.no_return {
                                        if f.data.defining {
                                            colon
                                        } else {
                                            format!(" {brace_open}")
                                        }
                                    } else {
                                        format!(
                                            " {} {}{}{}{}",
                                            two_dot,
                                            Colors::Blue,
                                            f.data
                                                .return_type
                                                .clone()
                                                .definer_type
                                                .to_definite()
                                                .to_string(),
                                            Colors::Blue,
                                            if f.data.defining {
                                                colon
                                            } else {
                                                format!(" {}", brace_open)
                                            }
                                        )
                                    }
                                ));
                                if !f.data.defining {
                                    for item in f.data.body {
                                        lines.extend(
                                            liner(item).iter().map(|x| format!("    {}", x)),
                                        );
                                    }
                                    lines.push(brace_close);
                                }
                            }
                            Processors::FileKey(_) => todo!(),
                            Processors::Import(_) => todo!(),
                            Processors::ForLoop(_) => todo!(),
                            Processors::Condition(_) => todo!(),
                            Processors::Constructor(_) => todo!(),
                            Processors::Class(_) => todo!(),
                            Processors::Ret(r) => lines.push(format!(
                                "{}ret{} {}{colon}",
                                Colors::Red,
                                Colors::Reset,
                                type_liner(r.value.current)
                            )),
                            Processors::SelfItem(_) => todo!(),
                            Processors::GenericItem(_) => todo!(),
                            Processors::FunctionParameter(_) => todo!(),
                            Processors::ConstructorParameter(_) => todo!(),
                            Processors::Brk(_) => todo!(),
                            Processors::Go(_) => todo!(),
                            Processors::Enum(_) => todo!(),
                        }
                        lines
                    }

                    for page in pager.pages.iter() {
                        let file_path = &page.path.replace(
                            &starter_name,
                            Path::new(target_path)
                                .absolutize()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .to_str()
                                .unwrap(),
                        );
                        panic!("{:?}", file_path);

                        let let_lines = page
                            .items
                            .iter()
                            .map(|item| liner(item.clone()))
                            .collect::<Vec<Vec<String>>>();
                        let page_line_len: usize = let_lines.iter().map(|x| x.len()).sum();
                        println!("{}│", generate_blank(2));
                        println!(
                            "{}╞ {}{}{}",
                            generate_blank(2),
                            Colors::Green,
                            page.path,
                            Colors::Reset
                        );
                        println!("{}├──", generate_blank(2));
                        println!("{}│", generate_blank(2));
                        let mut qline = 1;
                        for (inner_index, lines) in let_lines.iter().enumerate() {
                            for (line_index, line) in lines.iter().enumerate() {
                                let line_i = inner_index + (line_index);
                                println!(
                                    "{}│{}{}{}{} : {}",
                                    generate_blank(2),
                                    generate_blank(
                                        (page_line_len.to_string().len() - qline.to_string().len())
                                    ),
                                    Colors::Yellow,
                                    qline,
                                    Colors::Reset,
                                    line
                                );
                                qline += 1;
                            }
                        }
                    }

                    println!(
                        "\n{}[?]{}: Ellie v{}",
                        cli_utils::Colors::Green,
                        cli_utils::Colors::Reset,
                        crate::engine_constants::ELLIE_ENGINE_VERSION
                    );
                    println!(
                        "{}[?]{}: Tokenizing took {}{}{}ms",
                        cli_utils::Colors::Yellow,
                        cli_utils::Colors::Reset,
                        cli_utils::Colors::Yellow,
                        tokenize_end,
                        cli_utils::Colors::Reset,
                    );
                    println!(
                        "{}[!]{}: If colors are correct we're good",
                        cli_utils::Colors::Red,
                        cli_utils::Colors::Reset,
                    );
                }
                Err(pager_errors) => {
                    cli_utils::print_errors(
                        &pager_errors,
                        |path| match cli_utils::read_file(
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
                                panic!(
                                    "{}[Internal Error]{} Cannot build error, read file failed '{}' {}[{}]{}",
                                    cli_utils::Colors::Red,
                                    cli_utils::Colors::Reset,
                                    path,
                                    cli_utils::Colors::Red,
                                    err,
                                    cli_utils::Colors::Reset
                                );
                            }
                        },
                        colorizer_settings.show_debug_lines,
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
                    );
                }
            }
        }
        Err(err) => {
            println!(
                "Unable to read file ~{} [{}]",
                target_path.to_str().unwrap().to_string(),
                err
            );
            std::process::exit(1);
        }
    }
}
