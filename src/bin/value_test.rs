use ellie_parser;
use ellie_core;
//use std::fs;


fn main() {
    let pos = ellie_core::defs::CursorPosition(0, 0);
    let mut emulated_collector_data = ellie_parser::syntax::variable::VariableCollector::default();
    emulated_collector_data.data.dynamic = true;
    //We should set to dynamic for catching type
    /*
        emulated_collector_data.r#type = ellie_parser::syntax::r#type::Collecting::Generic(
            ellie_parser::syntax::r#type::GenericType {
                r#type: "f32".to_string()
            }
        );
    */

    let code = "

        v test: i8 = 1;
    
    ";

    for (index, char) in code.chars().enumerate() {
        if char == '\n' || char == '\r' {
            continue;
        }
        let letter_char = &char.to_string();
        let last_char = &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
        let next_char = &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
        let itered = ellie_parser::processors::value_processor::collect(
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
                    ellie_core::utils::terminal_colors::get_color(ellie_core::utils::terminal_colors::Colors::Yellow),
                    error.debug_message,
                    ellie_core::utils::terminal_colors::get_color(ellie_core::utils::terminal_colors::Colors::Reset)
                ),
                ellie_core::utils::terminal_colors::get_color(ellie_core::utils::terminal_colors::Colors::Red),
                &error.code,
                ellie_core::utils::terminal_colors::get_color(ellie_core::utils::terminal_colors::Colors::Reset),
                ellie_core::utils::terminal_colors::get_color(ellie_core::utils::terminal_colors::Colors::Cyan),
                error.title,
                ellie_core::utils::terminal_colors::get_color(ellie_core::utils::terminal_colors::Colors::Reset),
                error.builded_message
            );
        }
        emulated_collector_data = itered.itered_data;
    }
    //let j = serde_json::to_string(&emulated_collector_data).unwrap();
    //fs::write("data.json", j).unwrap();
    println!("{:#?}", emulated_collector_data.clone());
}
