use day05_01_02::parse;
use day05_01_02::filter_points_with_same_x_or_y;
use day05_01_02::get_count_of_points_with_at_least_2_overlaps;

pub fn main() {
    use std::io::Read;
    let mut fd = std::fs::File::open("input").unwrap();
    let mut contens = String::new();
    fd.read_to_string(&mut contens).unwrap();
    let mut point_pairs = parse::<500>(&contens);
    let points_between = get_count_of_points_with_at_least_2_overlaps(point_pairs);
    println!("part2 {}", points_between);

    filter_points_with_same_x_or_y(&mut point_pairs);
    let points_between = get_count_of_points_with_at_least_2_overlaps(point_pairs);
    println!("part1 {}", points_between);
}
