use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ellie_engine::{compiler, parseText};

use crate::codes::BENCH_FIBONACCI_RECURSION;

pub mod codes;
pub mod utils;

fn tokenize(c: &mut Criterion) {
    c.bench_function("class_tokenize", |b| {
        b.iter(|| {
            let _ = black_box(utils::tokenize_code(black_box(codes::BENCH_CLASS)));
        })
    });
    c.bench_function("loop_tokenize", |b| {
        b.iter(|| {
            let _ = black_box(utils::tokenize_code(black_box(codes::BENCH_LOOP)));
        })
    });
    c.bench_function("fibonacci_loop_tokenize", |b| {
        b.iter(|| {
            let _ = black_box(utils::tokenize_code(black_box(codes::BENCH_FIBONACCI_LOOP)));
        })
    });
    c.bench_function("fibonacci_recursion_tokenize", |b| {
        b.iter(|| {
            let _ = black_box(utils::tokenize_code(black_box(
                codes::BENCH_FIBONACCI_RECURSION,
            )));
        })
    });
}

fn parse(c: &mut Criterion) {
    let class_tokenized = utils::tokenize_code(codes::BENCH_CLASS);
    let loop_tokenized = utils::tokenize_code(codes::BENCH_LOOP);
    let fibonacci_loop_tokenized = utils::tokenize_code(codes::BENCH_FIBONACCI_LOOP);
    let fibonacci_recursion_tokenized = utils::tokenize_code(codes::BENCH_FIBONACCI_RECURSION);

    c.bench_function("class_parse", |b| {
        b.iter(|| {
            let _ = black_box(utils::parse_code(class_tokenized.clone()));
        })
    });

    c.bench_function("loop_parse", |b| {
        b.iter(|| {
            let _ = black_box(utils::parse_code(loop_tokenized.clone()));
        })
    });

    c.bench_function("fibonacci_loop_parse", |b| {
        b.iter(|| {
            let _ = black_box(utils::parse_code(fibonacci_loop_tokenized.clone()));
        })
    });

    c.bench_function("fibonacci_recursion_parse", |b| {
        b.iter(|| {
            let _ = black_box(utils::parse_code(fibonacci_recursion_tokenized.clone()));
        })
    });
}

fn byte_code_assembler(c: &mut Criterion) {
    let class_parse = parseText!(codes::BENCH_CLASS).expect("should've been successful");
    let loop_parse = parseText!(codes::BENCH_LOOP).expect("should've been successful");
    let fibonacci_loop_parse =
        parseText!(codes::BENCH_FIBONACCI_LOOP).expect("should've been successful");
    let fibonacci_recursion_parse =
        parseText!(codes::BENCH_FIBONACCI_RECURSION).expect("should've been successful");

    c.bench_function("class_byte_code_assembler", |b| {
        b.iter(|| {
            let _ = black_box(compiler::byte_code_assembler(class_parse.clone()));
        })
    });
}

criterion_group!(benches, tokenize, parse);
criterion_main!(benches);
