use std::collections::HashSet;
pub fn parse<const HEIGHT: usize, const WIDTH: usize>(input: &str) -> [[u32; WIDTH]; HEIGHT] {
    let mut ret = [[0u32; WIDTH]; HEIGHT];
    let lines = input.split('\n').filter(|ll| !ll.is_empty());
    for (yy, line) in lines.enumerate() {
        let chars = line.chars();
        for (xx, cc) in chars.enumerate() {
            ret[yy][xx] = cc.to_digit(10).unwrap() as u32;
        }
    }
    ret
}

#[derive(Debug, Default)]
struct AdjacentPoints {
    left: Option<(u32, (usize, usize))>,
    right: Option<(u32, (usize, usize))>,
    above: Option<(u32, (usize, usize))>,
    below: Option<(u32, (usize, usize))>,
    top_left: Option<(u32, (usize, usize))>,
    top_right: Option<(u32, (usize, usize))>,
    bottom_left: Option<(u32, (usize, usize))>,
    bottom_right: Option<(u32, (usize, usize))>,
}

fn get_adjacent_points<const HEIGHT: usize, const WIDTH: usize>(
    map: &[[u32; WIDTH]; HEIGHT],
    from: (usize, usize),
) -> AdjacentPoints {
    let mut ret = AdjacentPoints::default();
    let (xx, yy) = from;

    if xx > 0 {
        ret.left = Some((map[yy][xx - 1], (xx - 1, yy)));
    }

    if xx < WIDTH - 1 {
        ret.right = Some((map[yy][xx + 1], (xx + 1, yy)));
    }

    if yy > 0 {
        ret.above = Some((map[yy - 1][xx], (xx, yy - 1)));
    }

    if yy < HEIGHT - 1 {
        ret.below = Some((map[yy + 1][xx], (xx, yy + 1)));
    }

    if yy > 0 && xx > 0 {
        ret.top_left = Some((map[yy - 1][xx - 1], (xx - 1, yy - 1)))
    }

    if yy > 0 && xx < WIDTH - 1 {
        ret.top_right = Some((map[yy - 1][xx + 1], (xx + 1, yy - 1)))
    }

    if yy < HEIGHT - 1 && xx > 0 {
        ret.bottom_left = Some((map[yy + 1][xx - 1], (xx - 1, yy + 1)));
    }

    if yy < HEIGHT - 1 && xx < WIDTH - 1 {
        ret.bottom_right = Some((map[yy + 1][xx + 1], (xx + 1, yy + 1)));
    }

    ret
}

#[allow(unused)]
fn print_matrix<const HEIGHT: usize, const WIDTH: usize>(matrix: &[[u32; WIDTH]; HEIGHT]) {
    use termion::color;
    for yy in 0..matrix.len() {
        for xx in 0..matrix[0].len() {
            let val = matrix[yy][xx];
            if val < 10 {
                print!("{:02}", val);
            } else {
                print!(
                    "{}{}{}",
                    color::Fg(color::Red),
                    val,
                    color::Fg(color::Reset)
                );
            }
        }
        println!();
    }
    println!();
}

pub fn part1<const HEIGHT: usize, const WIDTH: usize>(matrix: &mut [[u32; WIDTH]; HEIGHT]) -> u128 {
    let mut count = 0_u128;
    for _ in 0..100 {
        let mut found = HashSet::new();
        for yy in 0..HEIGHT {
            for xx in 0..WIDTH {
                matrix[yy][xx] += 1;
                if matrix[yy][xx] > 9 && !found.contains(&(xx, yy)) {
                    recurse_flashes_from_point(matrix, (xx, yy), &mut found);
                }
            }
        }
        for (xx, yy) in &found {
            matrix[*yy][*xx] = 0;
        }
        count += found.len() as u128;
    }
    count
}

pub fn part2<const HEIGHT: usize, const WIDTH: usize>(matrix: &mut [[u32; WIDTH]; HEIGHT]) -> u128 {
    for step in 0..500 {
        let mut found = HashSet::with_capacity(HEIGHT * WIDTH);
        for yy in 0..HEIGHT {
            for xx in 0..WIDTH {
                matrix[yy][xx] += 1;
                if matrix[yy][xx] > 9 && !found.contains(&(xx, yy)) {
                    recurse_flashes_from_point(matrix, (xx, yy), &mut found);
                }
            }
        }
        if found.len() == WIDTH * HEIGHT {
            return step + 1;
        }
        for (xx, yy) in &found {
            matrix[*yy][*xx] = 0;
        }
    }
    panic!("all did not flash bud");
}

fn recurse_flashes_from_point<const HEIGHT: usize, const WIDTH: usize>(
    map: &mut [[u32; WIDTH]; HEIGHT],
    start: (usize, usize),
    found: &mut HashSet<(usize, usize)>,
) {
    found.insert(start);

    print_matrix(map);
    let AdjacentPoints {
        left,
        right,
        above,
        below,
        top_left,
        top_right,
        bottom_left,
        bottom_right,
    } = get_adjacent_points(map, start);

    let func = |x: Option<(u32, (usize, usize))>| {
        if let Some((_, next_pos)) = x {
            let y = map[next_pos.1][next_pos.0];
            map[next_pos.1][next_pos.0] = y + 1;
            if y + 1 > 9 && !found.contains(&next_pos) {
                recurse_flashes_from_point(map, next_pos, found);
            }
        }
    };
    [
        left,
        right,
        above,
        below,
        top_left,
        top_right,
        bottom_left,
        bottom_right,
    ]
    .into_iter()
    .for_each(func);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut matrix = parse::<10, 10>(&input);
        print_matrix(&matrix);
        let count = part1(&mut matrix);
        assert_eq!(1656, count);
    }

    #[test]
    fn test_part2() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut matrix = parse::<10, 10>(&input);
        print_matrix(&matrix);
        let count = part2(&mut matrix);
        assert_eq!(195, count);
    }
}
