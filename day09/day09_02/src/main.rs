use std::collections::HashSet;
use std::io::Read;
fn main() {
    let mut fd = std::fs::File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let map = parse::<100, 100>(&contents);
    let ans = count_three_largest_basins(&map);
    println!("{}", ans);
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

fn get_basins_from_point<const HEIGHT: usize, const WIDTH: usize>(
    map: &[[i32; WIDTH]; HEIGHT],
    start: (usize, usize),
    found: &mut HashSet<(usize, usize)>,
) {
    found.insert(start);
    let AdjacentPoints {
        left,
        right,
        above,
        below,
    } = get_adjacent_points(map, start);

    if let Some((left, next_pos)) = left {
        if left != 9 && !found.contains(&next_pos) {
            get_basins_from_point(map, next_pos, found);
        }
    }

    if let Some((right, next_pos)) = right {
        if right != 9 && !found.contains(&next_pos) {
            get_basins_from_point(map, next_pos, found);
        }
    }

    if let Some((above, next_pos)) = above {
        if above != 9 && !found.contains(&next_pos) {
            get_basins_from_point(map, next_pos, found);
        }
    }

    if let Some((below, next_pos)) = below {
        if below != 9 && !found.contains(&next_pos) {
            get_basins_from_point(map, next_pos, found);
        }
    }
}

struct AdjacentPoints {
    left: Option<(i32, (usize, usize))>,
    right: Option<(i32, (usize, usize))>,
    above: Option<(i32, (usize, usize))>,
    below: Option<(i32, (usize, usize))>,
}

fn get_adjacent_points<const HEIGHT: usize, const WIDTH: usize>(
    map: &[[i32; WIDTH]; HEIGHT],
    from: (usize, usize),
) -> AdjacentPoints {
    let (xx, yy) = from;
    let left = if xx > 0 {
        Some((map[yy][xx - 1], (xx - 1, yy)))
    } else {
        None
    };
    let right = if xx < WIDTH - 1 {
        Some((map[yy][xx + 1], (xx + 1, yy)))
    } else {
        None
    };
    let above = if yy > 0 {
        Some((map[yy - 1][xx], (xx, yy - 1)))
    } else {
        None
    };
    let below = if yy < HEIGHT - 1 {
        Some((map[yy + 1][xx], (xx, yy + 1)))
    } else {
        None
    };

    AdjacentPoints {
        left,
        right,
        below,
        above,
    }
}

fn count_three_largest_basins<const HEIGHT: usize, const WIDTH: usize>(
    map: &[[i32; WIDTH]; HEIGHT],
) -> usize {
    let low_points = get_low_points(map);

    let mut basins: Vec<usize> = low_points
        .into_iter()
        .map(|(_, lp)| {
            let mut basins = HashSet::new();
            get_basins_from_point(map, lp, &mut basins);
            basins
        })
        .map(|basins| basins.len())
        .collect();
    basins.sort_by(|a, b| b.partial_cmp(a).unwrap());
    basins.into_iter().take(3).product()
}

fn get_low_points<const HEIGHT: usize, const WIDTH: usize>(
    map: &[[i32; WIDTH]; HEIGHT],
) -> Vec<(i32, (usize, usize))> {
    let mut low_points = Vec::new();
    for yy in 0..HEIGHT {
        for xx in 0..WIDTH {
            let num_at_pos = map[yy][xx];
            let AdjacentPoints {
                left,
                right,
                above,
                below,
            } = get_adjacent_points(map, (xx, yy));
            let mut is_smaller = true;
            if let Some((left, _)) = left {
                if num_at_pos >= left {
                    is_smaller = false;
                }
            }
            if let Some((right, _)) = right {
                if num_at_pos >= right {
                    is_smaller = false;
                }
            }
            if let Some((above, _)) = above {
                if num_at_pos >= above {
                    is_smaller = false;
                }
            }
            if let Some((below, _)) = below {
                if num_at_pos >= below {
                    is_smaller = false;
                }
            }
            if is_smaller {
                low_points.push((num_at_pos, (xx, yy)));
            }
        }
    }
    low_points
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_low_points() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let map = parse::<5, 10>(&input);
        let low_points = get_low_points(&map);
        assert_eq!(4, low_points.len());
        assert_eq!((1, (1, 0)), low_points[0]);
        assert_eq!((0, (9, 0)), low_points[1]);
        assert_eq!((5, (2, 2)), low_points[2]);
        assert_eq!((5, (6, 4)), low_points[3]);
    }

    #[test]
    fn test_example_basins() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let map = parse::<5, 10>(&input);
        let ans = count_three_largest_basins(&map);
        assert_eq!(1134, ans);
    }
}
