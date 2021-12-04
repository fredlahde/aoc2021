use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day04_part2::solve;

const INPUT_FN: &str = "input";
use std::io::Read;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut fd = std::fs::File::open(INPUT_FN).unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();

    c.bench_function("bingo", |b| b.iter(|| solve(black_box(&contents))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
