#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::Read;
use day07_02::solve;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut numbers = [0i32; 1000];
    let mut fd = std::fs::File::open("../input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let mut split = contents
        .split(',')
        .filter(|s| *s != "")
        .map(|s| s.replace('\n', ""));
    for ii in 0..numbers.len() {
        let s = split.next().unwrap();
        numbers[ii] = s.parse().unwrap();
    }
    let numbers: Vec<i32> = numbers.to_vec();
    c.bench_function("fuel", |b| b.iter(|| solve(black_box(&numbers))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
