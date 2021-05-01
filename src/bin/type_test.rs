use ellie_parser;
use ellie_core;
//use std::fs;


fn main() {
    let mut errors : Vec<ellie_core::error::Error> = vec![];
    let mut emulated_collector_data = ellie_parser::syntax::r#type::TypeConstructorCollector::default();
    let code = "
        
        fn(test: string, test2: string) > string
    
    ";

    for (index, char) in code.chars().enumerate() {
        if char == '\n' || char == '\r' {
            continue;
        }
        let letter_char = &char.to_string();
        let last_char = &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
        let next_char = &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
        
        ellie_parser::processors::type_check_processor::collect(
            &mut emulated_collector_data,
            &mut errors,
            letter_char.to_string(),
            last_char.to_string(),
            next_char.to_string()
        );
    }
    for error in errors {
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
    println!("Emulated data: {:#?}", emulated_collector_data);
}
