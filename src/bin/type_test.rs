use ellie_core;
use ellie_parser;

fn main() {
    let mut pos = ellie_core::defs::CursorPosition(0, 0);
    let mut errors: Vec<ellie_core::error::Error> = vec![];
    let mut emulated_collector_data = ellie_parser::syntax::definers::DefinerCollecting::default();
    let code = "
        array(16,test)
    ";

    for (index, char) in code.chars().enumerate() {
        if char == '\n' || char == '\r' {
            continue;
        }
        let letter_char = &char.to_string();
        let last_char = &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
        let next_char = &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
        ellie_parser::processors::definer_processor::collect_definer(
            &mut emulated_collector_data,
            &mut errors,
            letter_char.to_string(),
            pos,
            next_char.to_string(),
            last_char.to_string(),
            ellie_core::defs::ParserOptions::default(),
        );
        pos.0 += 1;
    }
    for error in errors {
        println!(
            "{}{}Error[{:#04x}]{} - {}{}{}: {} |  {}-{}",
            format!(
                "{}[{}]{} ",
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
            error.builded_message.builded,
            error.pos.range_start.0,
            error.pos.range_end.0
        );
    }
    println!("Emulated data: {:#?}", emulated_collector_data);
}
