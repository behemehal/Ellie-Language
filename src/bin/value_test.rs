use ellie_core;
use ellie_parser;
//use std::fs;

/*

    This is a development tool for collecting values

*/

fn main() {
    let mut emulated_parser = ellie_parser::parser::Parser::new(
        "".to_owned(),
        |_, _, _| ellie_parser::parser::ResolvedImport::default(),
        |_| {},
        ellie_core::defs::ParserOptions::default(),
    );
    let mut emulated_collector_data = ellie_parser::syntax::variable::VariableCollector::default();
    emulated_collector_data.data.dynamic = true;
    let code = "
        \"test string\"
    ";

    let mut content = code.split("").collect::<Vec<_>>();
    content.remove(0);
    content.remove(content.len() - 1);
    for i in 0..content.len() {
        let char = content[i].chars().next().unwrap();
        if char == '\n' || char == '\r' {
            emulated_parser.pos.1 = 0;
            emulated_parser.pos.0 += 1;
            continue;
        }
        let letter_char = &char.to_string();
        let last_char = if i == 0 { "" } else { content[i - 1] };
        let next_char = if i + 1 > content.len() - 1 {
            ""
        } else {
            content[i + 1]
        };

        let mut errors = Vec::new();

        ellie_parser::processors::value_processor::collect_value(
            emulated_parser.clone(),
            &mut emulated_collector_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        );

        for error in errors {
            println!(
                "{}{}Error[{:#04x}]{} - {}{}{}: {}",
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
                ellie_engine::terminal_colors::get_color(
                    ellie_engine::terminal_colors::Colors::Red
                ),
                &error.code,
                ellie_engine::terminal_colors::get_color(
                    ellie_engine::terminal_colors::Colors::Reset
                ),
                ellie_engine::terminal_colors::get_color(
                    ellie_engine::terminal_colors::Colors::Cyan
                ),
                error.title,
                ellie_engine::terminal_colors::get_color(
                    ellie_engine::terminal_colors::Colors::Reset
                ),
                error.builded_message.builded
            );
        }
        emulated_parser.pos.1 += 1;
    }
    //let j = serde_json::to_string(&emulated_collector_data).unwrap();
    //fs::write("data.json", j).unwrap();
    //println!("{:#?}", emulated_collector_data.data.clone());
}
