use ellie_core;
use ellie_parser;
//use std::fs;

fn main() {
    let pos = ellie_core::defs::CursorPosition(0, 0);
    let mut emulated_collector_data = ellie_parser::syntax::variable::VariableCollector::default();
    emulated_collector_data.data.dynamic = true;
    //We should set to dynamic for catching type
    /*
        emulated_collector_data.rtype = ellie_parser::syntax::definers::Collecting::Generic(
            ellie_parser::syntax::definers::GenericType {
                rtype: "f32".to_string()
            }
        );
    */

    let code = "

        @(test: array(i8, 8), test2: i8) {}

    ";

    for (index, char) in code.chars().enumerate() {
        if char == '\n' || char == '\r' {
            continue;
        }
        let letter_char = &char.to_string();
        let last_char = &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
        let next_char = &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
        let itered = ellie_parser::processors::value_processor::collect_value(
            &mut emulated_collector_data,
            letter_char,
            next_char.to_string(),
            last_char.to_string(),
            pos,
<<<<<<< HEAD
<<<<<<< HEAD
            ellie_core::defs::ParserOptions::default(),
=======
<<<<<<< HEAD
            ellie_core::defs::ParserOptions::default(),
=======
            ellie_core::defs::ParserOptions::default()
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
            ellie_core::defs::ParserOptions::default(),
>>>>>>> FFI
        );

        for error in itered.errors {
            println!(
                "{}{}Error[{:#04x}]{} - {}{}{}: {}",
                format!(
                    "{}[{}]{} ",
                    ellie_core::utils::terminal_colors::get_color(
                        ellie_core::utils::terminal_colors::Colors::Yellow
                    ),
                    error.debug_message,
                    ellie_core::utils::terminal_colors::get_color(
                        ellie_core::utils::terminal_colors::Colors::Reset
                    )
                ),
                ellie_core::utils::terminal_colors::get_color(
                    ellie_core::utils::terminal_colors::Colors::Red
                ),
                &error.code,
                ellie_core::utils::terminal_colors::get_color(
                    ellie_core::utils::terminal_colors::Colors::Reset
                ),
                ellie_core::utils::terminal_colors::get_color(
                    ellie_core::utils::terminal_colors::Colors::Cyan
                ),
                error.title,
                ellie_core::utils::terminal_colors::get_color(
                    ellie_core::utils::terminal_colors::Colors::Reset
                ),
                error.builded_message.builded
            );
        }
        emulated_collector_data = itered.itered_data;
    }
    //let j = serde_json::to_string(&emulated_collector_data).unwrap();
    //fs::write("data.json", j).unwrap();
    println!("{:#?}", emulated_collector_data.clone());
}
