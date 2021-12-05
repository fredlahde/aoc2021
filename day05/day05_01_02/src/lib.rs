use std::collections::{HashMap, HashSet};

/// solves `y - (m * x)`
fn solve_for_b(p1: (i32, i32), slope: i32) -> i32 {
    p1.1 - (slope * p1.0)
}

/// checks if `y = (m * x) + b`
fn is_point_on_line_with_defined_slope(p: (i32, i32), m: i32, b: i32) -> bool {
    p.1 == (m * p.0) + b
}

#[derive(Debug)]
enum Slope {
    Defined(i32),
    Undefined,
}

/// Tries to calculate `(y2 - y1) / (x2 -x1)`
/// Can return `Slope::Undefined` in the event that `x2 == x1`
fn get_slope(p1: (i32, i32), p2: (i32, i32)) -> Slope {
    if (p2.0 - p1.0) == 0 {
        return Slope::Undefined;
    }
    Slope::Defined((p2.1 - p1.1) / (p2.0 - p1.0))
}

pub fn parse<const N: usize>(input: &str) -> [Option<((i32, i32), (i32, i32))>; N] {
    let mut arr = [None; N];
    let mut split = input.split('\n').filter(|s| !s.is_empty());
    for ii in 0..N {
        let mut split = split.next().unwrap().split_whitespace();
        let (p1, p2) = (split.nth(0).unwrap(), split.nth(1).unwrap());
        let (mut p1, mut p2) = (p1.split(','), p2.split(','));

        let mut p1 = p1.map(|y| y.parse::<i32>().unwrap());
        let mut p1 = (p1.next().unwrap(), p1.next().unwrap());
        let mut p2 = p2.map(|y| y.parse().unwrap());
        let mut p2 = (p2.next().unwrap(), p2.next().unwrap());

        arr[ii] = Some((p1, p2));
    }
    arr
}

pub fn filter_points_with_same_x_or_y<const N: usize>(
    points: &mut [Option<((i32, i32), (i32, i32))>; N],
) {
    for ii in 0..points.len() {
        let pp = points[ii];
        if let Some(pp) = pp {
            if !(pp.0 .0 == pp.1 .0 || pp.0 .1 == pp.1 .1) {
                points[ii] = None;
            }
        }
    }
}

fn get_points_on_line_between_points(p1: (i32, i32), p2: (i32, i32)) -> HashSet<(i32, i32)> {
    let slope = get_slope(p1, p2);
    let mut points: HashSet<(i32, i32)> = HashSet::with_capacity(500);
    points.insert(p1);
    points.insert(p2);
    match slope {
        Slope::Undefined => {
            let min = p1.1.min(p2.1);
            let max = p1.1.max(p2.1);
            for yy in min..=max {
                let x = (p1.0, yy);
                if !points.contains(&x) {
                    points.insert((p1.0, yy));
                }
            }
        }
        Slope::Defined(slope) => {
            let b = solve_for_b(p1, slope);
            let min_xx = p1.0.min(p2.0);
            let max_xx = p1.0.max(p2.0);

            let min_yy = p1.1.min(p2.1);
            let max_yy = p1.1.max(p2.1);
            for xx in min_xx..=max_xx {
                for yy in min_yy..=max_yy {
                    let is_on_slope = is_point_on_line_with_defined_slope((xx, yy), slope, b);
                    if is_on_slope {
                        points.insert((xx, yy));
                    }
                }
            }
        }
    };
    points
}

pub fn get_count_of_points_with_at_least_2_overlaps<const N: usize>(
    points: [Option<((i32, i32), (i32, i32))>; N],
) -> usize {
    let mut mem: HashMap<(i32, i32), u32> = HashMap::new();
    let mut filtered = points
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap());

    for point in filtered {
        let between = get_points_on_line_between_points(point.0, point.1);
        for bb in between {
            let entry = mem.entry(bb).or_insert(0);
            *entry += 1;
        }
    }

    mem.iter().filter(|(_, count)| *count >= &2).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_point_on_line_defined_slope() {
        let m = 3;
        let b = 1;
        let p = (2, 7);

        let answer = is_point_on_line_with_defined_slope(p, m, b);
        assert!(answer);
    }

    #[test]
    fn test_is_point_on_line_defined_slope2() {
        let m = 3;
        let b = 1;
        let p = (4, 8);

        let answer = is_point_on_line_with_defined_slope(p, m, b);
        assert!(!answer);
    }

    #[test]
    fn test_points_in_between_undefined_slope() {
        let (p1, p2) = ((2, 2), (2, 1));
        let between = get_points_on_line_between_points(p1, p2);
        assert_eq!(2, between.len());
        assert!(between.contains(&(2, 1)));
        assert!(between.contains(&(2, 2)));
    }

    #[test]
    fn test_points_in_between_defined_slope() {
        let (p1, p2) = ((9, 7), (7, 7));
        let between = get_points_on_line_between_points(p1, p2);
        dbg!(&between);
        assert_eq!(3, between.len());
        assert!(between.contains(&(9, 7)));
        assert!(between.contains(&(8, 7)));
        assert!(between.contains(&(7, 7)));
    }

    #[test]
    fn test_aoc_part1() {
        let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

        let mut point_pairs = parse::<10>(input);
        assert_eq!((0, 9), point_pairs[0].unwrap().0);
        assert_eq!((8, 2), point_pairs[point_pairs.len() - 1].unwrap().1);
        filter_points_with_same_x_or_y(&mut point_pairs);
        let points_between = get_count_of_points_with_at_least_2_overlaps(point_pairs);
        assert_eq!(5, points_between);
    }

    #[test]
    fn test_aoc_part1_parse() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let mut point_pairs = parse::<10>(input);
        assert_eq!((0, 9), point_pairs[0].unwrap().0);
        assert_eq!((8, 2), point_pairs[point_pairs.len() - 1].unwrap().1);
        filter_points_with_same_x_or_y(&mut point_pairs);
        let filtered = point_pairs.into_iter().filter(|x| x.is_some()).count();
        assert_eq!(6, filtered);

        let x = point_pairs
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| (x, get_slope(x.unwrap().0, x.unwrap().1)))
            .collect::<Vec<_>>();
        dbg!(x);
    }

    #[test]
    fn test_solve_for_b_01() {
        // 94 = 0*654 + b
        // 0*654 + b = 94
        // (94-0*654) = b
        //  y  m x
        let p1 = (654, 94);
        let p2 = (76, 94);
        let slope = 0;

        let x = solve_for_b(p1, slope);
        assert_eq!(94, x);
    }

    #[test]
    fn test_solve_for_b_02() {
        let p1 = (28, 971);
        let slope = -1;

        let x = solve_for_b(p1, slope);
        assert_eq!(999, x);
    }

    #[test]
    fn test_slope() {
        let (p1, p2) = ((958, 965), (54, 61));

        let slope = get_slope(p1, p2);

        if let Slope::Defined(slope) = slope {
            assert_eq!(1, slope);
        } else {
            panic!("got undefined slope");
        }
    }

    #[test]
    fn test_slope_undefined() {
        let (p1, p2) = ((958, 965), (958, 61));

        let slope = get_slope(p1, p2);

        assert!(matches!(slope, Slope::Undefined));
    }

    #[test]
    fn test_slope_defined() {
        let (p1, p2) = ((958, 965), (4, 61));

        let slope = get_slope(p1, p2);

        assert!(matches!(slope, Slope::Defined(_)));
    }
}
