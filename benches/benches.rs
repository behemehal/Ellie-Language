use criterion::{criterion_group, criterion_main, Criterion};
use ellie_parser::parser;
use std::env;

fn variable_benchmark(c: &mut Criterion) {
    let parser = parser::Parser::new(
        "
            v test: string = \"string\";
            v test2: char = 's';
            v test3: integer = 123;
            v test4: float = 1.2;
            v test5: bool = true;
            v test6: cloak(string, char, integer, float, bool) = (\"string\", 'e', 123, 2.1, true);
            v test7: array(char, 5 ) = [
                'e',
                'l',
                'l',
                'i',
                'e'
            ];
        "
        .to_owned(),
        |_, _, _| ellie_parser::parser::ResolvedImport::default(),
        |_| {},
        ellie_core::defs::ParserOptions {
            path: "<benchmark>".to_owned(),
            functions: true,
            break_on_error: false,
            loops: true,
            conditions: true,
            classes: true,
            enums: true,
            dynamics: true,
            global_variables: true,
            getters: true,
            setters: true,
            line_ending: if env::consts::OS == "windows" {
                "\\r\\n".to_owned()
            } else {
                "\\n".to_owned()
            },
            collectives: true,
            variables: true,
            constants: true,
            parser_type: ellie_core::defs::ParserType::RawParser,
            allow_import: true,
        },
    );
    c.bench_function("parse variables", |b| b.iter(|| parser.clone().map()));
}

criterion_group!(benches, variable_benchmark);
criterion_main!(benches);
