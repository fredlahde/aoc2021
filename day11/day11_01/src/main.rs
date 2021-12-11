use std::io::Read;
use day11_01_02::*;


fn main() {
    let mut fd = std::fs::File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let mut matrix = parse::<10, 10>(&contents);
    let count = part1(&mut matrix);
    println!("part1 {}", count);

    let mut matrix = parse::<10, 10>(&contents);
    let count = part2(&mut matrix);
    println!("part2 {}", count);
}
