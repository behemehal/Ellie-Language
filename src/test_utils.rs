use ellie_core::defs;
use ellie_parser::parser::{Parser, ResolvedImport};

pub fn emulate_value_processor(
    code: &str,
) -> (
    ellie_parser::syntax::types::Types,
    Vec<ellie_core::error::Error>,
) {
    let mut emulated_parser = Parser::new(
        "".to_string(),
        |_, _, _| ResolvedImport::default(),
        |_| {},
        defs::ParserOptions::default(),
    );
    let mut emulated_collector_data = ellie_parser::syntax::variable::VariableCollector {
        ignore_existence: true,
        ..Default::default()
    };
    let mut syntax_errors = vec![];
    emulated_collector_data.data.dynamic = true;
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
            syntax_errors.push(error);
        }
        emulated_collector_data = itered.itered_data;
        emulated_parser.pos.1 += 1;
    }
    (emulated_collector_data.data.value, syntax_errors)
}

pub fn has_no_error_and_correct(
    found: (
        ellie_parser::syntax::types::Types,
        Vec<ellie_core::error::Error>,
    ),
    expected: ellie_parser::syntax::types::Types,
) -> bool {
    found.0 == expected && found.1.len() == 0
}

/*

    Operators are so complicated so I created special emulator and expector

*/

pub fn emulate_value_processor_operator(
    code: &str,
) -> (
    ellie_parser::syntax::types::operator_type::OperatorType,
    Vec<ellie_core::error::Error>,
) {
    let emulated_parser = Parser::new(
        "".to_string(),
        |_, _, _| ResolvedImport::default(),
        |_| {},
        defs::ParserOptions::default(),
    );
    let mut emulated_collector_data = ellie_parser::syntax::variable::VariableCollector::default();
    let mut syntax_errors = vec![];
    emulated_collector_data.data.dynamic = true;
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
            syntax_errors.push(error);
        }
        emulated_collector_data = itered.itered_data;
    }

    let mut found = ellie_parser::syntax::types::operator_type::OperatorType::default();

    if let ellie_parser::syntax::types::Types::Operator(operator_type) =
        emulated_collector_data.data.value
    {
        found = operator_type.data;
    } else {
        //We should give a error so it fails
        syntax_errors.push(ellie_core::error::Error {
            message: "This is a custom error that indicates catched value is not a operator"
                .to_string(),
            ..Default::default()
        })
    }
    (found, syntax_errors)
}

pub fn has_no_error_and_correct_operator(
    found: (
        ellie_parser::syntax::types::operator_type::OperatorType,
        Vec<ellie_core::error::Error>,
    ),
    expected: ellie_parser::syntax::types::operator_type::OperatorType,
) -> bool {
    found.0 == expected && found.1.len() == 0
}
