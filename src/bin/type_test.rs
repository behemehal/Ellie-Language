use ellie_core;
use ellie_parser;

fn main() {
    let mut pos = ellie_core::defs::CursorPosition(0, 0);
    let mut errors: Vec<ellie_core::error::Error> = vec![];
    let emulated_parser = ellie_parser::parser::Parser::new(
        "".to_owned(),
        |_, _, _| ellie_parser::parser::ResolvedImport::default(),
        |_| {},
        ellie_core::defs::ParserOptions::default(),
    );
    let mut emulated_collector_data = ellie_parser::syntax::definers::DefinerCollecting::default();
    let code = "
    collective(string, dyn)
    ";

    let mut content = code.split("").collect::<Vec<_>>();
    content.remove(0);
    content.remove(content.len() - 1);
    for i in 0..content.len() {
        let char = content[i].chars().next().unwrap();
        if char == '\n' || char == '\r' {
            continue;
        }
        let letter_char = char.to_string();
        let last_char = if i == 0 { "" } else { content[i - 1] };
        let next_char = if i + 1 > content.len() - 1 {
            ""
        } else {
            content[i + 1]
        };
        ellie_parser::processors::definer_processor::collect_definer(
            emulated_parser.clone(),
            &mut emulated_collector_data,
            &mut errors,
            letter_char.to_string(),
            next_char,
            last_char,
        );
        pos.0 += 1;
    }
    for error in errors {
        println!(
            "{}{}Error[{:#04x}]{} - {}{}{}: {} |  {}-{}",
            format!(
                "{}[{}]{} ",
                ellie_engine::terminal_colors::get_color(
                    ellie_engine::terminal_colors::Colors::Yellow
                ),
                error.debug_message,
                ellie_engine::terminal_colors::get_color(
                    ellie_engine::terminal_colors::Colors::Reset
                )
            ),
            ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Red),
            &error.code,
            ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Reset),
            ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Cyan),
            error.title,
            ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Reset),
            error.builded_message.builded,
            error.pos.range_start.0,
            error.pos.range_end.0
        );
    }
    println!("Emulated data: {:#?}", emulated_collector_data);
}
