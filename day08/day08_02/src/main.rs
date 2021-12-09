use std::fs::File;
use std::io::Read;

fn main() {
    let mut fd = File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let parsed = parse(&contents);
    let ans = part_one(&parsed);
    println!("part one {}", ans);
}

const COUNT_ONE: usize = 2;
const COUNT_FOUR: usize = 4;
const COUNT_SEVEN: usize = 3;
const COUNT_EIGHT: usize = 7;

fn part_one(parsed: &[(Vec<&str>, Vec<&str>)]) -> usize {
    parsed
        .iter()
        .map(|it| &it.1)
        .flat_map(|it| it.iter())
        .map(|it| it.len())
        .filter(|it| matches!(*it, COUNT_ONE | COUNT_FOUR | COUNT_SEVEN | COUNT_EIGHT))
        .count()
}

#[derive(Default, Debug)]
struct SevenSegmentDisplay {
    top: char,
    top_left: char,
    top_right: char,
    middle: char,
    bottom_left: char,
    bottom_right: char,
    bottom: char,
}

impl SevenSegmentDisplay {
    fn is_zero(&self, output: &str) -> bool {
        output.contains(self.top)
            && output.contains(self.top_left)
            && output.contains(self.top_right)
            && output.contains(self.bottom_left)
            && output.contains(self.bottom_right)
            && output.contains(self.bottom)
            && !output.contains(self.middle)
    }

    fn is_one(&self, output: &str) -> bool {
        !output.contains(self.top)
            && !output.contains(self.top_left)
            && output.contains(self.top_right)
            && !output.contains(self.bottom_left)
            && output.contains(self.bottom_right)
            && !output.contains(self.bottom)
            && !output.contains(self.middle)
    }

    fn is_two(&self, output: &str) -> bool {
        output.contains(self.top)
            && !output.contains(self.top_left)
            && output.contains(self.top_right)
            && output.contains(self.bottom_left)
            && !output.contains(self.bottom_right)
            && output.contains(self.bottom)
            && output.contains(self.middle)
    }

    fn is_three(&self, output: &str) -> bool {
        output.contains(self.top)
            && !output.contains(self.top_left)
            && output.contains(self.top_right)
            && !output.contains(self.bottom_left)
            && output.contains(self.bottom_right)
            && output.contains(self.bottom)
            && output.contains(self.middle)
    }

    fn is_four(&self, output: &str) -> bool {
        !output.contains(self.top)
            && output.contains(self.top_left)
            && output.contains(self.top_right)
            && !output.contains(self.bottom_left)
            && output.contains(self.bottom_right)
            && !output.contains(self.bottom)
            && output.contains(self.middle)
    }

    fn is_five(&self, output: &str) -> bool {
        output.contains(self.top)
            && output.contains(self.top_left)
            && !output.contains(self.top_right)
            && !output.contains(self.bottom_left)
            && output.contains(self.bottom_right)
            && output.contains(self.bottom)
            && output.contains(self.middle)
    }

    fn is_six(&self, output: &str) -> bool {
        output.contains(self.top)
            && output.contains(self.top_left)
            && !output.contains(self.top_right)
            && output.contains(self.bottom_left)
            && output.contains(self.bottom_right)
            && output.contains(self.bottom)
            && output.contains(self.middle)
    }

    fn is_seven(&self, output: &str) -> bool {
        output.contains(self.top)
            && !output.contains(self.top_left)
            && output.contains(self.top_right)
            && !output.contains(self.bottom_left)
            && output.contains(self.bottom_right)
            && !output.contains(self.bottom)
            && !output.contains(self.middle)
    }

    fn is_eight(&self, output: &str) -> bool {
        output.contains(self.top)
            && output.contains(self.top_left)
            && output.contains(self.top_right)
            && output.contains(self.bottom_left)
            && output.contains(self.bottom_right)
            && output.contains(self.bottom)
            && output.contains(self.middle)
    }

    fn is_nine(&self, output: &str) -> bool {
        output.contains(self.top)
            && output.contains(self.top_left)
            && output.contains(self.top_right)
            && !output.contains(self.bottom_left)
            && output.contains(self.bottom_right)
            && output.contains(self.bottom)
            && output.contains(self.middle)
    }
    fn decode(&self, output: &str) -> u8 {
        if self.is_zero(output) {
            0
        } else if self.is_one(output) {
            1
        } else if self.is_two(output) {
            2
        } else if self.is_three(output) {
            3
        } else if self.is_four(output) {
            4
        } else if self.is_five(output) {
            5
        } else if self.is_six(output) {
            6
        } else if self.is_seven(output) {
            7
        } else if self.is_eight(output) {
            8
        } else if self.is_nine(output) {
            9
        } else {
            panic!("unknown pattern {}", output);
        }
    }
}

#[derive(Default)]
struct UniqSignals {
    one: String,
    four: String,
    seven: String,
    eight: String,
}

impl UniqSignals {
    fn into_seven_segments(&self) -> SevenSegmentDisplay {
        let mut display = SevenSegmentDisplay::default();
        display.top_right = self.one.chars().nth(0).unwrap();
        display.bottom_right = self.one.chars().nth(1).unwrap();

        display.top_left = self.four.chars().nth(0).unwrap();
        display.middle = self.four.chars().nth(2).unwrap();
        display.top = self
            .seven
            .chars()
            .filter(|it| *it != display.top_right && *it != display.bottom_right)
            .nth(0)
            .unwrap();
        let mut eight_candidates = self.eight.chars().filter(|it| {
            *it != display.top_right
                && *it != display.bottom_right
                && *it != display.top
                && *it != display.middle
                && *it != display.top_left
        });
        display.bottom_left = eight_candidates.nth(0).unwrap();
        display.bottom = eight_candidates.nth(1).unwrap();
        display
    }
}

impl From<Vec<&str>> for UniqSignals {
    fn from(signals: Vec<&str>) -> Self {
        let mut ret = Self::default();
        for signal in signals {
            match signal.len() {
                COUNT_ONE => ret.one = signal.to_owned(),
                COUNT_FOUR => ret.four = signal.to_owned(),
                COUNT_SEVEN => ret.seven = signal.to_owned(),
                COUNT_EIGHT => ret.eight = signal.to_owned(),
                _ => {}
            }
        }
        ret
    }
}

fn parse<'a>(input: &'a str) -> Vec<(Vec<&'a str>, Vec<&'a str>)> {
    let mut ret = Vec::new();
    let lines = input.split('\n');
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut split = line.split('|');
        let signals = split.next().unwrap();
        let signals = signals.split_whitespace().collect::<Vec<_>>();
        let outputs = split.next().unwrap();
        let outputs = outputs.split_whitespace().collect::<Vec<_>>();
        ret.push((signals, outputs));
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let parsed = parse(&input);
        let part_one_ans = part_one(&parsed);
        assert_eq!(26, part_one_ans);
    }

    #[test]
    fn test_part_two() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let parsed = parse(&input);
        let uniq: Vec<&str> = parsed
            .iter()
            .flat_map(|it| it.0.as_slice())
            .map(|it| *it)
            .collect();
        let uniq: UniqSignals = uniq.into();
        assert_eq!("be", uniq.one);
        assert_eq!("cgeb", uniq.four);
        assert_eq!("edb", uniq.seven);
        assert_eq!("cfbegad", uniq.eight);
        let display = uniq.into_seven_segments();
        for output in &parsed[0].1 {
            println!("{} {}", output, display.decode(output));
        }
    }

    #[test]
    fn test_get_uniq_singals() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let parsed = parse(&input);
        let uniq: Vec<&str> = parsed
            .iter()
            .flat_map(|it| it.0.as_slice())
            .map(|it| *it)
            .collect();
        let uniq: UniqSignals = uniq.into();
        assert_eq!("be", uniq.one);
        assert_eq!("cgeb", uniq.four);
        assert_eq!("edb", uniq.seven);
        assert_eq!("cfbegad", uniq.eight);
    }

    #[test]
    fn test_get_seven_segment() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let parsed = parse(&input);
        let uniq: Vec<&str> = parsed
            .iter()
            .flat_map(|it| it.0.as_slice())
            .map(|it| *it)
            .collect();
        let uniq: UniqSignals = uniq.into();
        assert_eq!("be", uniq.one);
        assert_eq!("cgeb", uniq.four);
        assert_eq!("edb", uniq.seven);
        assert_eq!("cfbegad", uniq.eight);
        let seven_segment = uniq.into_seven_segments();
        assert_eq!('b', seven_segment.top_right);
        assert_eq!('e', seven_segment.bottom_right);

        assert_eq!('c', seven_segment.top_left);
        assert_eq!('e', seven_segment.middle);

        assert_eq!('d', seven_segment.top);
        assert_eq!('f', seven_segment.bottom_left);
        assert_eq!('a', seven_segment.bottom);
        dbg!(seven_segment);
    }
}
