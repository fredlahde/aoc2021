use std::io::Read;
fn main() {
    let mut fd = std::fs::File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let map = parse::<100, 100>(&contents);
    let part1 = part1(&map);
    println!("{}", part1);
}

fn parse<const HEIGHT: usize, const WIDTH: usize>(input: &str) -> [[i32; WIDTH]; HEIGHT] {
    let mut map = [[0i32; WIDTH]; HEIGHT];
    let lines = input.split('\n');
    for (ii, line) in lines.enumerate() {
        if line.is_empty() {
            continue;
        }
        let numbers = line.chars();
        for (jj, num) in numbers.enumerate() {
            let num: i32 = num.to_digit(10).unwrap() as i32;
            map[ii][jj] = num;
        }
    }
    map
}

fn part1<const HEIGHT: usize, const WIDTH: usize>(map: &[[i32; WIDTH]; HEIGHT]) -> i32 {
    let mut low_points = Vec::new();
    for yy in 0..HEIGHT {
        for xx in 0..WIDTH {
            let num_at_pos = map[yy][xx];
            let left = if xx > 0 { Some(map[yy][xx - 1]) } else { None };
            let right = if xx < WIDTH - 1 {
                Some(map[yy][xx + 1])
            } else {
                None
            };
            let above = if yy > 0 { Some(map[yy - 1][xx]) } else { None };
            let below = if yy < HEIGHT - 1 {
                Some(map[yy + 1][xx])
            } else {
                None
            };
            let mut is_smaller = true;
            if let Some(left) = left {
                if num_at_pos >= left {
                    is_smaller = false;
                }
            }
            if let Some(right) = right {
                if num_at_pos >= right {
                    is_smaller = false;
                }
            }
            if let Some(above) = above {
                if num_at_pos >= above {
                    is_smaller = false;
                }
            }
            if let Some(below) = below {
                if num_at_pos >= below {
                    is_smaller = false;
                }
            }
            if is_smaller {
                low_points.push(num_at_pos + 1);
            }
        }
    }
    return low_points.into_iter().sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let map = parse::<5, 10>(&input);
        let danger = part1(&map);
    }
}
