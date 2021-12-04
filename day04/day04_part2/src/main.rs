use day04_part2::solve;

const INPUT_FN: &str = "input";
use std::io::Read;
fn main() {
    let mut fd = std::fs::File::open(INPUT_FN).unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let last_score = crate::solve(&contents);

    println!("{}", last_score);
}
