use ellie_parser::parser;
use ellie_raw::converter;

fn main() {
    let code = "

        class test {
            co test();
        }

    "
    .to_string();

    let parser = parser::Parser::new(
        code.clone(),
        |e| {
            if e == "ellie" {
                let lib: ellie_parser::parser::Parsed =
                    serde_json::from_str(ellie_core::builded_libraries::ELLIE_STANDARD_LIBRARY)
                        .unwrap();
                ellie_parser::parser::ResolvedImport {
                    found: true,
                    file_content: lib,
                }
            } else {
                ellie_parser::parser::ResolvedImport::default()
            }
        },
        ellie_core::defs::ParserOptions {
            functions: true,
            break_on_error: false,
            loops: true,
            conditions: true,
            classes: true,
            dynamics: true,
            global_variables: true,
            line_ending: "\\n".to_string(),
            collectives: true,
            variables: true,
            constants: true,
            parser_type: ellie_core::defs::ParserType::RawParser,
            allow_import: true,
        },
    );
    let mapped = parser.map();

    if mapped.syntax_errors.len() != 0 {
        for error in &ellie_lang::cli_utils::zip_errors(mapped.syntax_errors) {
            if error.pos.range_start.0 != error.pos.range_end.0 {
                println!(
                    "{}[Experimental]{}: Multi line error listing",
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Magenta
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Reset
                    ),
                );
                println!(
                    "{}{}Error[{:#04x}]{} - {}{}{}: {}",
                    format!(
                        "{}({}) {}[{}]{} ",
                        ellie_lang::terminal_colors::get_color(
                            ellie_lang::terminal_colors::Colors::Magenta
                        ),
                        error.scope,
                        ellie_lang::terminal_colors::get_color(
                            ellie_lang::terminal_colors::Colors::Yellow
                        ),
                        error.debug_message,
                        ellie_lang::terminal_colors::get_color(
                            ellie_lang::terminal_colors::Colors::Reset
                        )
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Red
                    ),
                    &error.code,
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Reset
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Cyan
                    ),
                    error.title,
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Reset
                    ),
                    error.builded_message.builded
                );
                println!(
                    "{}:[{} ~ {}]:?",
                    "eval",
                    error.pos.range_start.0 + 1,
                    error.pos.range_end.0 + 1
                );
                let mut pos = vec![error.pos.range_start];

                for _ in 1..error.pos.range_end.0 {
                    pos.push(error.pos.range_end)
                }

                println!(
                    "{}",
                    ellie_lang::cli_utils::get_lines(code.clone(), error.pos)
                )
            } else {
                println!(
                    "{}{}Error[{:#04x}]{} - {}{}{}: {}",
                    format!(
                        "{}({}) {}[{}]{} ",
                        ellie_lang::terminal_colors::get_color(
                            ellie_lang::terminal_colors::Colors::Magenta
                        ),
                        error.scope,
                        ellie_lang::terminal_colors::get_color(
                            ellie_lang::terminal_colors::Colors::Yellow
                        ),
                        error.debug_message,
                        ellie_lang::terminal_colors::get_color(
                            ellie_lang::terminal_colors::Colors::Reset
                        )
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Red
                    ),
                    &error.code,
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Reset
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Cyan
                    ),
                    error.title,
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Reset
                    ),
                    error.builded_message.builded
                );
                println!(
                    "{}:{}:{}",
                    "eval",
                    error.pos.range_start.0 + 1,
                    error.pos.range_start.1 + 1
                );
                let line: Vec<&str> = code.split("\\n").collect();
                println!(
                    "{}\n{}{}{}",
                    ellie_lang::cli_utils::get_line(code.clone(), error.pos.range_start.0 as usize),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Red
                    ),
                    ellie_lang::cli_utils::arrow(
                        (error.pos.range_start.1 + 1) as usize,
                        if error.pos.range_end.1 > error.pos.range_start.1 {
                            ((error.pos.range_end.1) - (error.pos.range_start.1)) as usize
                        } else {
                            error.pos.range_start.1 as usize - (line[error.pos.range_start.1]).len()
                        }
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Reset
                    )
                );
            }
        }
    } else {
        let raw_converter = converter::Converter::new("main".to_string(), "./main.ei".to_string(), 0);
        raw_converter.convert(mapped.parsed);
    }
}