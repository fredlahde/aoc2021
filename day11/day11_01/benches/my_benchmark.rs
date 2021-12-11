use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day10_02::*;
use std::io::Read;

pub fn part2_bench(c: &mut Criterion) {
    let mut fd = std::fs::File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let tokens = parse(&contents);

    let score = part2(&tokens);
    c.bench_function("part2", |b| b.iter(|| part2(black_box(&tokens))));
}

pub fn parse_bench(c: &mut Criterion) {
    let mut fd = std::fs::File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let tokens = parse(&contents);

    c.bench_function("parse", |b| b.iter(|| parse(black_box(&contents))));
}

criterion_group!(benches, part2_bench, parse_bench);
criterion_main!(benches);
