use day07_02::solve;
use std::io::Read;

fn main() {
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

    let dist_low = solve(&numbers);

    println!("{}", dist_low);
}
