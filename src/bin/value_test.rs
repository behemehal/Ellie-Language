use ellie_core;
use ellie_parser;
//use std::fs;

/*

    This is a development tool for collecting values

*/

fn main() {
    let emulated_parser = ellie_parser::parser::Parser::new(
        "".to_string(),
        |_, _| ellie_parser::parser::ResolvedImport::default(),
        |_| {},
        ellie_core::defs::ParserOptions::default(),
    );
    let mut emulated_collector_data = ellie_parser::syntax::variable::VariableCollector::default();
    emulated_collector_data.data.dynamic = true;
    let code = "

    {
        0: 'e',
        1: 'l',
        2: 'l',
        3: 'i',
        4: {
            0: 'e',
            1: 'l',
            2: 'l',
            3: 'i',
            4: 'e'
        }
    }
    
    ";

    for (index, char) in code.chars().enumerate() {
        if char == '\n' || char == '\r' {
            continue;
        }
        let letter_char = &char.to_string();
        let last_char = &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
        let next_char = &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
        let itered = ellie_parser::processors::value_processor::collect_value(
            emulated_parser.clone(),
            &mut emulated_collector_data,
            letter_char,
            next_char.to_string(),
            last_char.to_string(),
        );

        for error in itered.errors {
            println!(
                "{}{}Error[{:#04x}]{} - {}{}{}: {}",
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
                ellie_lang::terminal_colors::get_color(ellie_lang::terminal_colors::Colors::Red),
                &error.code,
                ellie_lang::terminal_colors::get_color(ellie_lang::terminal_colors::Colors::Reset),
                ellie_lang::terminal_colors::get_color(ellie_lang::terminal_colors::Colors::Cyan),
                error.title,
                ellie_lang::terminal_colors::get_color(ellie_lang::terminal_colors::Colors::Reset),
                error.builded_message.builded
            );
        }
        emulated_collector_data = itered.itered_data;
    }
    //let j = serde_json::to_string(&emulated_collector_data).unwrap();
    //fs::write("data.json", j).unwrap();
    println!("{:#?}", emulated_collector_data.data.clone());
}
