use ellie;
use std::fs;

fn main() {
    let pos = ellie::mapper::defs::CursorPosition(0, 0);
    let mut emulated_collector_data = ellie::syntax::variable::VariableCollector::default();
    let code = "         
        true || false && true
    ";

    for (index, char) in code.chars().enumerate() {
        if char == '\n' || char == '\r' {
            continue;
        }
        let letter_char = &char.to_string();
        let last_char = &ellie::utils::get_letter(code.to_string(), index, false).to_owned();
        let next_char = &ellie::utils::get_letter(code.to_string(), index, true).to_owned();
        let itered = ellie::processors::value_processor::collect(
            &mut emulated_collector_data,
            letter_char,
            next_char.to_string(),
            last_char.to_string(),
            pos,
        );

        for error in itered.errors {
            println!(
                "{}{}Error[{:#04x}]{} - {}{}{}: {}",
                format!(
                    "{}[{}]{} ",
                    ellie::utils::terminal_colors::get_color(ellie::utils::terminal_colors::Colors::Yellow),
                    error.debug_message,
                    ellie::utils::terminal_colors::get_color(ellie::utils::terminal_colors::Colors::Reset)
                ),
                ellie::utils::terminal_colors::get_color(ellie::utils::terminal_colors::Colors::Red),
                &error.code,
                ellie::utils::terminal_colors::get_color(ellie::utils::terminal_colors::Colors::Reset),
                ellie::utils::terminal_colors::get_color(ellie::utils::terminal_colors::Colors::Cyan),
                error.title,
                ellie::utils::terminal_colors::get_color(ellie::utils::terminal_colors::Colors::Reset),
                error.builded_message
            );
        }

        emulated_collector_data = itered.itered_data;
    }
    let j = serde_json::to_string(&emulated_collector_data).unwrap();
    fs::write("data.json", j).unwrap();
    //println!("{:#?}", j)
}
