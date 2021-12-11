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
    pos: [Option<(u32, (usize, usize))>; 8],
}

impl AdjacentPoints {
    fn for_each<F>(&self, mut f: F)
    where
        F: FnMut((u32, (usize, usize))),
    {
        self.pos.into_iter().for_each(|x| {
            if let Some(y) = x {
                f(y)
            }
        });
    }
}

fn get_adjacent_points<const HEIGHT: usize, const WIDTH: usize>(
    map: &[[u32; WIDTH]; HEIGHT],
    from: (usize, usize),
) -> AdjacentPoints {
    let mut ret = AdjacentPoints::default();
    let (xx, yy) = from;

    if xx > 0 {
        ret.pos[0] = Some((map[yy][xx - 1], (xx - 1, yy)));
    }

    if xx < WIDTH - 1 {
        ret.pos[1] = Some((map[yy][xx + 1], (xx + 1, yy)));
    }

    if yy > 0 {
        ret.pos[2] = Some((map[yy - 1][xx], (xx, yy - 1)));
    }

    if yy < HEIGHT - 1 {
        ret.pos[3] = Some((map[yy + 1][xx], (xx, yy + 1)));
    }

    if yy > 0 && xx > 0 {
        ret.pos[4] = Some((map[yy - 1][xx - 1], (xx - 1, yy - 1)))
    }

    if yy > 0 && xx < WIDTH - 1 {
        ret.pos[5] = Some((map[yy - 1][xx + 1], (xx + 1, yy - 1)))
    }

    if yy < HEIGHT - 1 && xx > 0 {
        ret.pos[6] = Some((map[yy + 1][xx - 1], (xx - 1, yy + 1)));
    }

    if yy < HEIGHT - 1 && xx < WIDTH - 1 {
        ret.pos[7] = Some((map[yy + 1][xx + 1], (xx + 1, yy + 1)));
    }

    ret
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

    get_adjacent_points(map, start).for_each(|(_, next_pos)| {
        let y = map[next_pos.1][next_pos.0];
        map[next_pos.1][next_pos.0] = y + 1;
        if y + 1 > 9 && !found.contains(&next_pos) {
            recurse_flashes_from_point(map, next_pos, found);
        }
    });
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
        let count = part2(&mut matrix);
        assert_eq!(195, count);
    }
}
